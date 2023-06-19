use anyhow::Result;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Size {
    Imperial(f32),
    Metric(f32),
    Dots(f32),
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

#[derive(Debug)]
pub struct Tape {
    width: Size,
    height: Option<Size>,
    gap: Size,
    gap_offset: Option<Size>,
}

pub struct Printer {
    file: std::fs::File,
    tape: Tape,
    command: String,
}

impl Printer {
    /// Create a new printer from file. Usually it is somewhere in '/dev/usb/lp*'.
    pub fn new(path: String, tape: Tape) -> Result<Self> {
        let file = std::fs::File::options().append(true).open(path)?;
        Ok(Self {
            file,
            tape,
            command: String::new(),
        })
    }

    /// This command defines the label width and height.
    /// Label length must be provided for firmware version <V8.13
    fn size(&mut self, width: Size, height: Option<Size>) {
        match height {
            Some(height) => self.command += format!("SIZE {width},{height}\r\n").as_str(),
            None => self.command += format!("SIZE {width}\r\n").as_str(),
        }
    }

    /// Defines the gap distance between two labels.
    /// Optional offset distance of the gap may be provided
    fn gap(&mut self, gap: Size, gap_offset: Option<Size>){
        match gap_offset {
            Some(offset) => self.command += format!("GAP {gap},{offset}\r\n").as_str(),
            None => self.command += format!("GAP {gap}\r\n").as_str(),
        }
    }
}
