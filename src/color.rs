use encase::ShaderType;
use glam::Vec4;
use tracing::debug;
use wgpu::{Buffer, BufferDescriptor, BufferUsages, Color, Device};

type PaletteDefinition = [u32; 12];

pub struct PaletteBuffer {
    pub buffer: Buffer,
}

pub enum Palette {
    Crust = 0,
    Mantle = 1,
    Base = 2,
    Surface0 = 3,
    Surface1 = 4,
    Surface2 = 5,
    Overlay0 = 6,
    Overlay1 = 7,
    Overlay2 = 8,
    Subtext0 = 9,
    Subtext1 = 10,
    Text = 11,
}

pub const MOCHA: PaletteDefinition = [
    0x11111b, 0x181825, 0x1e1e2e, 0x313244, 0x45475a, 0x585b70, 0x6c7086, 0x7f849c, 0x9399b2,
    0xa6adc8, 0xbac2de, 0xcdd6f4,
];

#[repr(C)]
#[derive(Debug, ShaderType)]
pub struct Swatch {
    pub color: Vec4,
}

impl Swatch {
    pub fn as_wgsl_bytes(data: [Self; 12]) -> encase::internal::Result<Vec<u8>> {
        let mut buffer = encase::UniformBuffer::new(Vec::new());
        buffer.write(&data)?;
        Ok(buffer.into_inner())
    }
}

impl Default for Swatch {
    fn default() -> Self {
        Self {
            color: Vec4::new(0.0.into(), 0.0.into(), 0.0.into(), 0.0.into()),
        }
    }
}

impl PaletteBuffer {
    pub fn new(device: &Device) -> Self {
        debug!("Creating palette buffer");
        let buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: size_of::<Swatch>() as u64 * MOCHA.len() as u64,
            usage: wgpu::BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self { buffer }
    }
}

fn make_color_raw(c: u32) -> (f64, f64, f64) {
    let f = |xu: u32| {
        let x = (xu & 0xFF) as f64 / 255.0;
        if x > 0.04045 {
            ((x + 0.055) / 1.055).powf(2.4)
        } else {
            x / 12.92
        }
    };

    return (f(c >> 16), f(c >> 8), f(c));
}

pub fn make_color(c: u32, a: f64) -> Color {
    let (r, g, b) = make_color_raw(c);

    Color { r, g, b, a }
}

pub fn make_swatch(c: u32, a: f32) -> Swatch {
    let (r, g, b) = make_color_raw(c);
    Swatch {
        color: Vec4::new(r as f32, g as f32, b as f32, a as f32),
    }
}

pub fn make_pallete(palette: PaletteDefinition) -> [Swatch; 12] {
    let mut result: [Swatch; 12] = Default::default();

    for (idx, color) in palette.into_iter().enumerate() {
        result[idx] = make_swatch(color, 1.0);
    }

    result
}
