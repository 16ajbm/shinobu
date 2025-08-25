pub mod svg_painter;

use self::svg_painter::SvgPainter;

const COLOURS_PER_PALETTE: usize = 6;

// Hexadecimal color representation
type Colour = String;

// Dots are the individual coloured elements that make up a plate. Although they are called "dots",
// they can come in a variety of shapes.
#[derive(Debug, Clone)]
struct Dot {
    x: u32,
    y: u32,
    size: u32,
    color: Colour,
    shape: String,
}

#[derive(Debug, Clone)]
pub struct Plate {
    pub dots: Vec<Dot>,
    pub config: PlateConfig,
}

impl Plate {
    pub fn new(config: PlateConfig) -> Self {
        Self {
            dots: Self::generate_dots(config.clone()),
            config,
        }
    }

    fn generate_dots(config: PlateConfig) -> Vec<Dot> {
        let mut dots = Vec::new();
        for x in (0..config.width).step_by(10) {
            for y in (0..config.height).step_by(10) {
                let dot = Dot {
                    x,
                    y,
                    size: config.dot_size,
                    color: config.foreground.colors[0].clone(),
                    shape: "circle".to_string(),
                };
                dots.push(dot);
            }
        }
        dots
    }
}

#[derive(Debug, Clone)]
pub struct PlateConfig {
    pub dot_size: u32,
    pub width: u32,
    pub height: u32,
    pub foreground: Palette,
    pub background: Palette,
    pub density: u32,
}

impl PlateConfig {
    pub fn new(width: u32, height: u32, density: u32, dot_size: u32) -> Self {
        Self {
            dot_size,
            width,
            height,
            foreground: Palette::new("#ffffff".to_string()),
            background: Palette::default(),
            density,
        }
    }
}

// Opting for struct instead of alias in case there is any other additional attributes that
// may be needed in the future.
#[derive(Debug, Clone)]
pub struct Palette {
    colors: [Colour; COLOURS_PER_PALETTE],
}

impl Palette {
    fn new(colour: Colour) -> Self {
        Self {
            colors: std::array::from_fn(|_| colour.clone()),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::new("#000000ff".to_string())
    }
}

pub struct Generator {
    painter: Box<dyn PlatePainter>,
}

pub trait PlatePainter {
    fn paint(&self, plate: Plate) -> String;
}

impl Generator {
    pub fn new() -> Self {
        Self {
            painter: Box::new(SvgPainter),
        }
    }

    pub fn paint(&self, plate: Plate) -> String {
        self.painter.paint(plate)
    }
}
