use anyhow::{anyhow, Ok, Result};
use std::{fmt::Display, io::Write};

#[derive(Debug, Clone)]
pub enum Size {
    Imperial(f32),
    Metric(f32),
    Dots(u32),
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
}

impl Printer {
    /// Create a new printer from file. Usually it is somewhere in '/dev/usb/lp*'.
    pub fn new(path: String, tape: Tape) -> Result<Self> {
        let file = std::fs::File::options().append(true).open(path)?;
        let mut printer = Self { file };

        printer
            .size(tape.width, tape.height)?
            .gap(tape.gap, tape.gap_offset)?;

        Ok(printer)
    }

    /// This command defines the label width and height.
    /// Label length must be provided for firmware version <V8.13
    fn size(&mut self, width: Size, height: Option<Size>) -> Result<&mut Self> {
        match height {
            Some(height) => self
                .file
                .write_all(format!("SIZE {width},{height}\r\n").as_bytes())?,
            None => self
                .file
                .write_all(format!("SIZE {width}\r\n").as_bytes())?,
        };

        Ok(self)
    }

    /// Defines the gap distance between two labels.
    /// Optional offset distance of the gap may be provided
    fn gap(&mut self, gap: Size, gap_offset: Option<Size>) -> Result<&mut Self> {
        match gap_offset {
            Some(offset) => self
                .file
                .write_all(format!("GAP {gap},{offset}\r\n").as_bytes())?,
            None => self.file.write_all(format!("GAP {gap}\r\n").as_bytes())?,
        };

        Ok(self)
    }

    /// This command feeds the paper through the gap sensor in an effort
    /// to determine the paper and gap sizes, respectively.
    /// This command references the user’s approximate measurements.
    /// If the measurements conflict with the actual size, the GAPDETECT command will not work properly.
    /// This calibration method can be applied to the labels with pre-printed logos or texts.
    ///
    /// `dots` input tuple represent optional parameters
    /// dots.0: Paper length (in dots)
    /// dots.1: Gap length (in dots)
    /// If the None is passed then the printer will calibrate and determine the paper length and gap size automatically.
    pub fn gap_detect(&mut self, dots: Option<(u32, u32)>) -> Result<&mut Self> {
        match dots {
            Some((x, y)) => self
                .file
                .write_all(format!("GAPDETECT {x},{y}\r\n").as_bytes())?,
            None => self.file.write_all(format!("GAPDETECT\r\n").as_bytes())?,
        }
        Ok(self)
    }

    /// This command feeds the paper through the black mark sensor in an effort to determine
    /// the paper and black mark sizes, respectively. This command references the user’s approximate measurements.
    /// If the measurements conflict with the actual size, the BLINEDETECT command will not work properly.
    /// This calibration method can be applied to the labels with pre-printed logos or texts.
    ///
    /// `dots` input tuple represent optional parameters
    /// dots.0: Paper length (in dots)
    /// dots.1: Gap length (in dots)
    /// If the None is passed then the printer will calibrate and determine the paper length and gap size automatically.
    pub fn bline_detect(&mut self, dots: Option<(u32, u32)>) -> Result<&mut Self> {
        match dots {
            Some((x, y)) => self
                .file
                .write_all(format!("BLINEDETECT {x},{y}\r\n").as_bytes())?,
            None => self.file.write_all(format!("BLINEDETECT\r\n").as_bytes())?,
        }
        Ok(self)
    }

    /// This command feeds the paper through the gap/black mark sensor in an effort to determine
    /// the paper and gap/black mark sizes, respectively. This command references the user’s approximate measurements.
    /// If the measurements conflict with the actual size, the AUTODETECT command will not work properly.
    /// This calibration method can be applied to the labels with pre-printed logos or texts.
    ///
    /// `dots` input tuple represent optional parameters
    /// dots.0: Paper length (in dots)
    /// dots.1: Gap length (in dots)
    /// If the None is passed then the printer will calibrate and determine the paper length and gap size automatically.
    pub fn auto_detect(&mut self, dots: Option<(u32, u32)>) -> Result<&mut Self> {
        match dots {
            Some((x, y)) => self
                .file
                .write_all(format!("AUTODETECT {x},{y}\r\n").as_bytes())?,
            None => self.file.write_all(format!("AUTODETECT\r\n").as_bytes())?,
        }
        Ok(self)
    }

    /// This command sets the height of the black line and the user-defined extra label feeding length each form feed takes.
    /// Both parameters should be in the same measurement type (mm/inch/dot)
    pub fn bline(&mut self, black_line_height: Size, extra_feeding_len: Size) -> Result<&mut Self> {
        self.file
            .write_all(format!("BLINE {black_line_height},{extra_feeding_len}\r\n").as_bytes())?;
        Ok(self)
    }

    /// This command defines the selective, extra label feeding length each form feed takes, which,
    /// especially in peel-off mode and cutter mode, is used to adjust label stop position,
    /// so as for label to register at proper places for the intended purposes.
    /// The printer back tracks the extra feeding length before the next run of printing.
    pub fn offset(&mut self, offset: Size) -> Result<&mut Self> {
        self.file
            .write_all(format!("OFFSET {offset}\r\n").as_bytes())?;
        Ok(self)
    }

    /// This command defines the print speed.
    /// Available speeds in inch/sec should be checked for your printer model
    pub fn speed(&mut self, speed: &str) -> Result<&mut Self> {
        self.file
            .write_all(format!("SPEED {speed}\r\n").as_bytes())?;
        Ok(self)
    }

    /// This command sets the printing darkness from lightest(0) to darkest(15). Default density is 8.
    pub fn density(&mut self, density: u8) -> Result<&mut Self> {
        match density {
            1..=15 => self
                .file
                .write_all(format!("DENSITY {density}\r\n").as_bytes())?,
            _ => return Err(anyhow!("Density should be in range 0..15")),
        };

        Ok(self)
    }

    /// This command defines the printout direction and mirror image. This will be stored in the printer memory.
    pub fn direction(
        &mut self,
        reversed_direction: bool,
        mirrored_image: bool,
    ) -> Result<&mut Self> {
        self.file.write_all(
            format!(
                "DIRECTION {},{}\r\n",
                reversed_direction as u8, mirrored_image as u8
            )
            .as_bytes(),
        )?;

        Ok(self)
    }

    /// This command defines the reference point of the label. The reference (origin) point varies with the print direction.
    pub fn reference(&mut self, x_dots: u32, y_dots: u32) -> Result<&mut Self> {
        self.file
            .write_all(format!("REFERENCE {x_dots},{y_dots}\r\n").as_bytes())?;

        Ok(self)
    }

    /// This command moves the label’s horizontal and vertical position. A positive value moves the label
    /// further from the printing direction; a negative value moves the label towards the printing direction.
    pub fn shift(&mut self, x_dots: Option<i16>, y_dots: i16) -> Result<&mut Self> {
        match x_dots {
            Some(x_dots) => self
                .file
                .write_all(format!("SHIFT {x_dots},{y_dots}\r\n").as_bytes())?,
            None => self.file.write_all(format!("SHIFT {y_dots}\r\n").as_bytes())?,
        }

        Ok(self)
    }
}

#[test]
fn test() -> Result<()> {
    let tape = Tape {
        width: Size::Metric(30.0),
        height: Some(Size::Metric(20.0)),
        gap: Size::Metric(1.4),
        gap_offset: None,
    };

    let mut printer = Printer::new("/ololo".to_string(), tape)?;

    printer
        .gap_detect(Some((1, 1)))?
        .auto_detect(Some((1, 1)))?;

    Ok(())
}
