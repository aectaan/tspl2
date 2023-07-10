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
pub enum Country {
    Usa = 1,
    CanadianFrench = 2,
    SpanishLatinAmerica = 3,
    Dutch = 31,
    Belgian = 32,
    French = 33,
    Spanish = 34,
    Hungarian = 36,
    Yugoslavian = 38,
    Italian = 39,
    Switzerland = 41,
    Slovak = 42,
    UnitedKingdom = 44,
    Danish = 45,
    Swedish = 46,
    Norwegian = 47,
    Polish = 48,
    German = 49,
    Brazil = 55,
    English = 61,
    Portuguese = 351,
    Finnish = 358,
}

#[derive(Debug)]
pub enum Codepage7Bit {
    Usa,
    British,
    German,
    French,
    Danish,
    Italian,
    Spanish,
    Swedish,
    Swiss,
}

#[derive(Debug)]
pub enum Codepage8Bit {
    UnitedStates,
    Greek,
    Multilingual,
    Greek1,
    Slavic,
    Cyrillic,
    Turkish,
    Portuguese,
    Icelandic,
    Hebrew,
    CanadianFrench,
    Arabic,
    Nordic,
    Russian,
    Greek2,
}

#[derive(Debug)]
pub enum CodepageWindows {
    CentralEurope,
    Cyrillic,
    Latin1,
    Greek,
    Turkish,
    Hebrew,
    Arabic,
    Baltic,
    Vietnam,
    Japanese,
    ChineseSiplified,
    Korean,
    ChineseTraditional,
    Utf8,
}

#[derive(Debug)]
pub enum CodepageIso {
    Latin1,
    Latin2,
    Latin3,
    Baltic,
    Cyrillic,
    Arabic,
    Greek,
    Hebrew,
    Turkish,
    Latin6,
    Latin9,
}

#[derive(Debug)]
pub enum Codepage {
    Codepage7Bit(Codepage7Bit),
    Codepage8Bit(Codepage8Bit),
    CodepageWindows(CodepageWindows),
    CodepageIso(CodepageIso),
}

#[derive(Debug)]
pub struct Tape {
    width: Size,
    height: Option<Size>,
    gap: Size,
    gap_offset: Option<Size>,
}

#[derive(Debug)]
pub enum Selftest {
    /// Print a self-test page with whole printer information.
    All,
    /// Print a pattern to check the status of print head heat line.
    Pattern,
    /// Print a self-test page with Ethernet settings.
    Ethernet,
    /// Print a self-test page with Wi-Fi settings.
    Wlan,
    /// Print a self-test page with RS-232 settings.
    Rs232,
    /// Print a self-test page with printer settings.
    System,
    /// Print a self-test page with emulated language settings.
    Z,
    /// Print a self-test page with Bluetooth settings.
    Bt,
}

impl Display for Selftest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::All => write!(f, ""),
            Self::Pattern => write!(f, "PATTERN"),
            Self::Ethernet => write!(f, "ETHERNET"),
            Self::Wlan => write!(f, "WLAN"),
            Self::Rs232 => write!(f, "RS232"),
            Self::System => write!(f, "SYSTEM"),
            Self::Z => write!(f, "Z"),
            Self::Bt => write!(f, "BT"),
        }
    }
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
            None => self.file.write_all("GAPDETECT\r\n".as_bytes())?,
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
            None => self.file.write_all("BLINEDETECT\r\n".as_bytes())?,
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
            None => self.file.write_all("AUTODETECT\r\n".as_bytes())?,
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
            None => self
                .file
                .write_all(format!("SHIFT {y_dots}\r\n").as_bytes())?,
        }

        Ok(self)
    }

    /// This command orients the keyboard for use in different countries via
    /// defining special characters on the KP-200 series portable LCD keyboard (option).
    pub fn country(&mut self, country: Country) -> Result<&mut Self> {
        self.file
            .write_all(format!("COUNTRY {:03}\r\n", country as u16).as_bytes())?;
        Ok(self)
    }

    /// This command defines the code page of international character set.
    pub fn codepage(&mut self, codepage: Codepage) -> Result<&mut Self> {
        let code = match codepage {
            Codepage::Codepage7Bit(x) => match x {
                Codepage7Bit::Usa => "USA",
                Codepage7Bit::British => "BRI",
                Codepage7Bit::German => "GER",
                Codepage7Bit::French => "FRE",
                Codepage7Bit::Danish => "DAN",
                Codepage7Bit::Italian => "ITA",
                Codepage7Bit::Spanish => "SPA",
                Codepage7Bit::Swedish => "SWE",
                Codepage7Bit::Swiss => "SWI",
            },
            Codepage::Codepage8Bit(x) => match x {
                Codepage8Bit::UnitedStates => "437",
                Codepage8Bit::Greek => "737",
                Codepage8Bit::Multilingual => "850",
                Codepage8Bit::Greek1 => "851",
                Codepage8Bit::Slavic => "852",
                Codepage8Bit::Cyrillic => "855",
                Codepage8Bit::Turkish => "857",
                Codepage8Bit::Portuguese => "860",
                Codepage8Bit::Icelandic => "861",
                Codepage8Bit::Hebrew => "862",
                Codepage8Bit::CanadianFrench => "863",
                Codepage8Bit::Arabic => "864",
                Codepage8Bit::Nordic => "865",
                Codepage8Bit::Russian => "866",
                Codepage8Bit::Greek2 => "869",
            },
            Codepage::CodepageWindows(x) => match x {
                CodepageWindows::CentralEurope => "1250",
                CodepageWindows::Cyrillic => "1251",
                CodepageWindows::Latin1 => "1252",
                CodepageWindows::Greek => "1253",
                CodepageWindows::Turkish => "1254",
                CodepageWindows::Hebrew => "1255",
                CodepageWindows::Arabic => "1256",
                CodepageWindows::Baltic => "1257",
                CodepageWindows::Vietnam => "1258",
                CodepageWindows::Japanese => "932",
                CodepageWindows::ChineseSiplified => "936",
                CodepageWindows::Korean => "949",
                CodepageWindows::ChineseTraditional => "950",
                CodepageWindows::Utf8 => "UTF-8",
            },
            Codepage::CodepageIso(x) => match x {
                CodepageIso::Latin1 => "8859-1",
                CodepageIso::Latin2 => "8859-2",
                CodepageIso::Latin3 => "8859-3",
                CodepageIso::Baltic => "8859-4",
                CodepageIso::Cyrillic => "8859-5",
                CodepageIso::Arabic => "8859-6",
                CodepageIso::Greek => "8859-7",
                CodepageIso::Hebrew => "8859-8",
                CodepageIso::Turkish => "8859-9",
                CodepageIso::Latin6 => "8859-10",
                CodepageIso::Latin9 => "8859-15",
            },
        };

        self.file
            .write_all(format!("CODEPAGE {code}\r\n").as_bytes())?;

        Ok(self)
    }

    /// This command clears the image buffer.
    pub fn cls(&mut self) -> Result<&mut Self> {
        self.file.write_all("CLS\r\n".as_bytes())?;
        Ok(self)
    }

    /// This command feeds label with the specified length. The length is specified by dot.
    pub fn feed(&mut self, feed_dot: u16) -> Result<&mut Self> {
        match feed_dot {
            0..=9999 => self
                .file
                .write_all(format!("FEED {feed_dot}\r\n").as_bytes())?,
            _ => {
                return Err(anyhow!(
                    "feed length must be in range 0..9999, got {:?}",
                    feed_dot
                ))
            }
        }
        Ok(self)
    }

    /// This command feeds the label in reverse. The length is specified by dot.
    /// For TSPL printers only
    pub fn backup(&mut self, feed_dot: u16) -> Result<&mut Self> {
        match feed_dot {
            0..=9999 => self
                .file
                .write_all(format!("BACKUP {feed_dot}\r\n").as_bytes())?,
            _ => {
                return Err(anyhow!(
                    "backup length must be in range 0..9999, got {:?}",
                    feed_dot
                ))
            }
        }
        Ok(self)
    }

    /// This command feeds the label in reverse. The length is specified by dot.
    /// For TSPL2 printers only
    pub fn backfeed(&mut self, feed_dot: u16) -> Result<&mut Self> {
        match feed_dot {
            0..=9999 => self
                .file
                .write_all(format!("BACKFEED {feed_dot}\r\n").as_bytes())?,
            _ => {
                return Err(anyhow!(
                    "backfeed length must be in range 0..9999, got {:?}",
                    feed_dot
                ))
            }
        }
        Ok(self)
    }

    /// This command feeds label to the beginning of next label.
    pub fn formfeed(&mut self) -> Result<&mut Self> {
        self.file.write_all("FORMFEED\r\n".as_bytes())?;

        Ok(self)
    }

    /// This command will feed label until the internal sensor has determined the origin.
    /// Size and gap of the label should be defined before using this command.
    /// For TSPL programming printer: Back label to origin position.
    /// For TSPL2 programming printer: Feed label to origin position
    pub fn home(&mut self) -> Result<&mut Self> {
        self.file.write_all("HOME\r\n".as_bytes())?;
        Ok(self)
    }

    /// This command prints the label format currently stored in the image buffer.
    pub fn print(&mut self, sets: u32, copies: Option<u32>) -> Result<&mut Self> {
        match sets {
            1..=999999999 => {
                if let Some(copies) = copies {
                    match copies {
                        1..=999999999 => self
                            .file
                            .write_all(format!("PRINT {sets},{copies}\r\n").as_bytes())?,
                        _ => {
                            return Err(anyhow!(
                                "Copies qty must be in range 1..999999999, got {:?}",
                                copies
                            ))
                        }
                    }
                } else {
                    self.file
                        .write_all(format!("PRINT {sets}\r\n").as_bytes())?;
                }
            }
            _ => {
                return Err(anyhow!(
                    "Sets qty must be in range 1..999999999, got {:?}",
                    sets
                ))
            }
        }

        Ok(self)
    }

    /// This command controls the sound frequency of the beeper. There are 10 levels of sounds, from 0 to 9.
    /// The timing control can be set by the "interval" parameter, in range 1..4095
    pub fn sound(&mut self, level: u8, interval: u16) -> Result<&mut Self> {
        match (level, interval) {
            (0..=9, 1..=4095) => self
                .file
                .write_all(format!("SOUND {level},{interval}\r\n").as_bytes())?,
            _ => return Err(anyhow!("wrong sound parameters")),
        }
        Ok(self)
    }

    /// This command activates the cutter to immediately cut the labels without back feeding the label.
    pub fn cut(&mut self) -> Result<&mut Self> {
        self.file.write_all("CUT\r\n".as_bytes())?;
        Ok(self)
    }

    /// If the gap sensor is not set to a suitable sensitivity while feeding labels,
    /// the printer will not be able to locate the correct position of the gap.
    /// This command stops label feeding and makes the red LED flash if the printer
    /// does not locate gap after feeding the length of one label plus one preset value.
    ///
    /// N The maximum length for sensor detecting.
    ///
    /// Minpaper The minimum length of paper.
    ///
    /// Maxgap The maximum length of gap.
    pub fn limit_feed(
        &mut self,
        n: Size,
        minpaper_maxgap: Option<(Size, Size)>,
    ) -> Result<&mut Self> {
        match minpaper_maxgap {
            Some((x, y)) => self
                .file
                .write_all(format!("LIMITFEED {n},{x},{y}\r\n").as_bytes())?,
            None => self
                .file
                .write_all(format!("LIMITFEED {n}\r\n").as_bytes())?,
        }

        Ok(self)
    }

    /// At this command, the printer will print out the printer information.
    pub fn selftest(&mut self, test_kind: Selftest) -> Result<&mut Self> {
        self.file
            .write_all(format!("SELFTEST {test_kind}\r\n").as_bytes())?;
        Ok(self)
    }

    /// Let the printer wait until process of commands (before EOJ) be finished then go on the next command.
    pub fn eoj(&mut self) -> Result<&mut Self> {
        self.file.write_all("EOJ\r\n".as_bytes())?;
        Ok(self)
    }

    /// Let the printer wait specific period of time then go on next command.
    pub fn delay(&mut self, delay: std::time::Duration) -> Result<&mut Self> {
        let d = delay.as_millis();
        self.file.write_all(format!("DELAY {d}\r\n").as_bytes())?;
        Ok(self)
    }

    /// This command can show the image, which is in printer’s image buffer, on LCD panel.
    pub fn display(&self) {
        unimplemented!("DIY!")
    }

    /// This command can restore printer settings to defaults.
    pub fn initial_printer(&mut self) -> Result<&mut Self> {
        self.file.write_all("INITIALPRINTER\r\n".as_bytes())?;
        Ok(self)
    }

    /// This command can design user's own menu with a database resident on the printer.
    pub fn menu(&self) {
        unimplemented!("DIY!");
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
