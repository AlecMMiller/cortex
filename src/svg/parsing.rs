use std::path::Path;

use lyon::math::Point;
use lyon::path::PathEvent;
use lyon::tessellation::geometry_builder::*;
use lyon::tessellation::{self, FillOptions, FillTessellator, StrokeOptions, StrokeTessellator};
use tracing::debug;
use usvg::{tiny_skia_path, Group, Size, Transform};

use super::gpu_types::{GpuPrimitive, GpuTransform, GpuVertex};

pub const FALLBACK_COLOR: usvg::Color = usvg::Color {
    red: 0,
    green: 0,
    blue: 0,
};

pub struct Svg {
    primitives: Vec<GpuPrimitive>,
    transforms: Vec<GpuTransform>,
    pub mesh: VertexBuffers<GpuVertex, u32>,
    pub size: Size,
}

impl Svg {
    pub fn get_primitives(&self, color: u32, scale: f32) -> Vec<GpuPrimitive> {
        let scale = 2.0 * scale / self.size.width();
        self.primitives
            .iter()
            .map(|prim| GpuPrimitive {
                transform: prim.transform,
                color,
                scale,
                _pad: prim._pad,
            })
            .collect()
    }

    pub fn get_transforms(&self, x: f32, y: f32) -> Vec<GpuTransform> {
        self.transforms
            .iter()
            .map(|trans| {
                let mut d1 = trans.data0;
                d1[2] = x;
                d1[3] = y;
                GpuTransform {
                    data0: trans.data0,
                    data1: d1,
                }
            })
            .collect()
    }

    #[tracing::instrument()]
    pub fn load(path: &Path) -> Self {
        let mut fill_tess = FillTessellator::new();
        let mut stroke_tess = StrokeTessellator::new();
        let mut mesh: VertexBuffers<GpuVertex, u32> = VertexBuffers::new();

        let opt = usvg::Options::default();
        let file_data = std::fs::read(path).unwrap();
        let rtree = usvg::Tree::from_data(&file_data, &opt).unwrap();
        let mut transforms = Vec::new();
        let mut primitives = Vec::new();

        let mut prev_transform = usvg::Transform {
            sx: f32::NAN,
            kx: f32::NAN,
            ky: f32::NAN,
            sy: f32::NAN,
            tx: f32::NAN,
            ty: f32::NAN,
        };
        collect_geom(
            &rtree.root(),
            &mut prev_transform,
            &mut transforms,
            &mut primitives,
            &mut fill_tess,
            &mut mesh,
            &mut stroke_tess,
        );

        debug!(
            vertices = mesh.vertices.len(),
            indices = mesh.indices.len(),
            "Finished tessellation",
        );

        Self {
            primitives,
            transforms,
            mesh,
            size: rtree.size(),
        }
    }
}

pub fn collect_geom(
    group: &Group,
    prev_transform: &mut Transform,
    transforms: &mut Vec<GpuTransform>,
    primitives: &mut Vec<GpuPrimitive>,
    fill_tess: &mut FillTessellator,
    mesh: &mut VertexBuffers<GpuVertex, u32>,
    stroke_tess: &mut StrokeTessellator,
) {
    for node in group.children() {
        if let usvg::Node::Group(group) = node {
            collect_geom(
                group,
                prev_transform,
                transforms,
                primitives,
                fill_tess,
                mesh,
                stroke_tess,
            )
        } else if let usvg::Node::Path(p) = &node {
            let t = node.abs_transform();
            if t != *prev_transform {
                transforms.push(GpuTransform {
                    data0: [t.sx, t.kx, t.ky, t.sy],
                    data1: [t.tx, t.ty, 0.0, 0.0],
                });
            }
            *prev_transform = t;

            let transform_idx = transforms.len() as u32 - 1;

            if let Some(fill) = p.fill() {
                // fall back to always use color fill
                // no gradients (yet?)
                let color = match fill.paint() {
                    usvg::Paint::Color(c) => *c,
                    _ => FALLBACK_COLOR,
                };

                primitives.push(GpuPrimitive::new(
                    transform_idx,
                    color,
                    fill.opacity().get(),
                ));

                fill_tess
                    .tessellate(
                        convert_path(p),
                        &FillOptions::tolerance(0.01),
                        &mut BuffersBuilder::new(
                            mesh,
                            VertexCtor {
                                prim_id: primitives.len() as u32 - 1,
                            },
                        ),
                    )
                    .expect("Error during tessellation!");
            }

            if let Some(stroke) = p.stroke() {
                let (stroke_color, stroke_opts) = convert_stroke(stroke);
                primitives.push(GpuPrimitive::new(
                    transform_idx,
                    stroke_color,
                    stroke.opacity().get(),
                ));
                let _ = stroke_tess.tessellate(
                    convert_path(p),
                    &stroke_opts.with_tolerance(0.01),
                    &mut BuffersBuilder::new(
                        mesh,
                        VertexCtor {
                            prim_id: primitives.len() as u32 - 1,
                        },
                    ),
                );
            }
        }
    }
}

pub fn convert_path(p: &usvg::Path) -> PathConvIter {
    PathConvIter {
        iter: p.data().segments(),
        first: Point::new(0.0, 0.0),
        prev: Point::new(0.0, 0.0),
        deferred: None,
        needs_end: false,
    }
}

pub fn convert_stroke(s: &usvg::Stroke) -> (usvg::Color, StrokeOptions) {
    let color = match s.paint() {
        usvg::Paint::Color(c) => *c,
        _ => FALLBACK_COLOR,
    };
    let linecap = match s.linecap() {
        usvg::LineCap::Butt => tessellation::LineCap::Butt,
        usvg::LineCap::Square => tessellation::LineCap::Square,
        usvg::LineCap::Round => tessellation::LineCap::Round,
    };
    let linejoin = match s.linejoin() {
        usvg::LineJoin::Miter => tessellation::LineJoin::Miter,
        usvg::LineJoin::MiterClip => tessellation::LineJoin::MiterClip,
        usvg::LineJoin::Bevel => tessellation::LineJoin::Bevel,
        usvg::LineJoin::Round => tessellation::LineJoin::Round,
    };

    let opt = StrokeOptions::tolerance(0.01)
        .with_line_width(s.width().get())
        .with_line_cap(linecap)
        .with_line_join(linejoin);

    (color, opt)
}

/// Some glue between usvg's iterators and lyon's.
pub struct PathConvIter<'a> {
    iter: tiny_skia_path::PathSegmentsIter<'a>,
    prev: Point,
    first: Point,
    needs_end: bool,
    deferred: Option<PathEvent>,
}

impl<'l> Iterator for PathConvIter<'l> {
    type Item = PathEvent;
    fn next(&mut self) -> Option<PathEvent> {
        if self.deferred.is_some() {
            return self.deferred.take();
        }

        let next = self.iter.next();
        match next {
            Some(tiny_skia_path::PathSegment::MoveTo(pt)) => {
                if self.needs_end {
                    let last = self.prev;
                    let first = self.first;
                    self.needs_end = false;
                    self.prev = Point::new(pt.x, pt.y);
                    self.deferred = Some(PathEvent::Begin { at: self.prev });
                    self.first = self.prev;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    self.first = Point::new(pt.x, pt.y);
                    self.needs_end = true;
                    Some(PathEvent::Begin { at: self.first })
                }
            }
            Some(tiny_skia_path::PathSegment::LineTo(pt)) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = Point::new(pt.x, pt.y);
                Some(PathEvent::Line {
                    from,
                    to: self.prev,
                })
            }
            Some(tiny_skia_path::PathSegment::CubicTo(p1, p2, p0)) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = Point::new(p0.x, p0.y);
                Some(PathEvent::Cubic {
                    from,
                    ctrl1: Point::new(p1.x, p1.y),
                    ctrl2: Point::new(p2.x, p2.y),
                    to: self.prev,
                })
            }
            Some(tiny_skia_path::PathSegment::QuadTo(p0, p1)) => {
                self.needs_end = true;
                let from = self.prev;
                self.prev = Point::new(p1.x, p1.y);
                Some(PathEvent::Quadratic {
                    from,
                    ctrl: Point::new(p0.x, p0.y),
                    to: self.prev,
                })
            }
            Some(tiny_skia_path::PathSegment::Close) => {
                self.needs_end = false;
                self.prev = self.first;
                Some(PathEvent::End {
                    last: self.prev,
                    first: self.first,
                    close: true,
                })
            }
            None => {
                if self.needs_end {
                    self.needs_end = false;
                    let last = self.prev;
                    let first = self.first;
                    Some(PathEvent::End {
                        last,
                        first,
                        close: false,
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub struct VertexCtor {
    pub prim_id: u32,
}

impl FillVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::FillVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}

impl StrokeVertexConstructor<GpuVertex> for VertexCtor {
    fn new_vertex(&mut self, vertex: tessellation::StrokeVertex) -> GpuVertex {
        GpuVertex {
            position: vertex.position().to_array(),
            prim_id: self.prim_id,
        }
    }
}
