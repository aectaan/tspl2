use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Size {
    Imperial(f32),
    Metric(f32),
    Dots(u32),
}

impl Size {
    pub(crate) fn to_dots_raw(&self, resolution: u32) -> u32 {
        match self {
            Self::Imperial(x) => (*x * resolution as f32) as u32,
            Self::Metric(x) => (*x / 25.4 * resolution as f32) as u32,
            Self::Dots(x) => *x,
        }
    }
}

impl Display for Size {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Imperial(x) => write!(f, "{x}"),
            Self::Metric(x) => write!(f, "{x} mm"),
            Self::Dots(x) => write!(f, "{x} dot"),
        }
    }
}
