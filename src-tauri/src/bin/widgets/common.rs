use iced::{Background, Border, Color, Padding, Pixels, Rectangle, Size, Vector};
use iced_core::renderer;

pub fn underline<Renderer>(
    regions: &Vec<Rectangle>,
    renderer: &mut Renderer,
    baseline: Vector,
    size: Pixels,
    color: Color,
) where
    Renderer: iced_core::text::Renderer,
{
    for bounds in regions {
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle::new(
                    bounds.position() + baseline - Vector::new(0.0, size.0 * 0.08),
                    Size::new(bounds.width, 1.0),
                ),
                ..Default::default()
            },
            color,
        );
    }
}

pub fn strikethrough<Renderer>(
    regions: &Vec<Rectangle>,
    renderer: &mut Renderer,
    baseline: Vector,
    size: Pixels,
    color: Color,
) where
    Renderer: iced_core::text::Renderer,
{
    for bounds in regions {
        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle::new(
                    bounds.position() + baseline - Vector::new(0.0, size.0 / 2.0),
                    Size::new(bounds.width, 1.0),
                ),
                ..Default::default()
            },
            color,
        );
    }
}

pub fn set_background<Renderer>(
    regions: &Vec<Rectangle>,
    renderer: &mut Renderer,
    padding: Padding,
    translation: Vector,
    border: Border,
    background: Background,
) where
    Renderer: iced_core::text::Renderer,
{
    for bounds in regions {
        let bounds = Rectangle::new(
            bounds.position() - Vector::new(padding.left, padding.top),
            bounds.size() + Size::new(padding.horizontal(), padding.vertical()),
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: bounds + translation,
                border,
                ..Default::default()
            },
            background,
        );
    }
}
