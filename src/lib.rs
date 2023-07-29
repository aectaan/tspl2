use anyhow::{anyhow, Ok, Result};
use std::{
    fmt::Display,
    fs::File,
    io::{Read, Write},
};
use strum_macros::Display;

#[derive(Debug, Clone)]
pub enum Size {
    Imperial(f32),
    Metric(f32),
    Dots(i32),
}

impl Size {
    fn to_dots_raw(&self, resolution: u32) -> i32 {
        match self {
            Self::Imperial(x) => (*x * resolution as f32) as i32,
            Self::Metric(x) => (*x / 25.4 * resolution as f32) as i32,
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

#[derive(Debug, Display)]
pub enum Codepage7Bit {
    #[strum(serialize = "USA")]
    Usa,
    #[strum(serialize = "BRI")]
    British,
    #[strum(serialize = "GER")]
    German,
    #[strum(serialize = "FRE")]
    French,
    #[strum(serialize = "DAN")]
    Danish,
    #[strum(serialize = "ITA")]
    Italian,
    #[strum(serialize = "SPA")]
    Spanish,
    #[strum(serialize = "SWE")]
    Swedish,
    #[strum(serialize = "SWI")]
    Swiss,
}

#[derive(Debug, Display)]
pub enum Codepage8Bit {
    #[strum(serialize = "437")]
    UnitedStates,
    #[strum(serialize = "737")]
    Greek,
    #[strum(serialize = "850")]
    Multilingual,
    #[strum(serialize = "851")]
    Greek1,
    #[strum(serialize = "852")]
    Slavic,
    #[strum(serialize = "855")]
    Cyrillic,
    #[strum(serialize = "857")]
    Turkish,
    #[strum(serialize = "860")]
    Portuguese,
    #[strum(serialize = "861")]
    Icelandic,
    #[strum(serialize = "862")]
    Hebrew,
    #[strum(serialize = "863")]
    CanadianFrench,
    #[strum(serialize = "864")]
    Arabic,
    #[strum(serialize = "865")]
    Nordic,
    #[strum(serialize = "866")]
    Russian,
    #[strum(serialize = "869")]
    Greek2,
}

#[derive(Debug, Display)]
pub enum CodepageWindows {
    #[strum(serialize = "1250")]
    CentralEurope,
    #[strum(serialize = "1251")]
    Cyrillic,
    #[strum(serialize = "1252")]
    Latin1,
    #[strum(serialize = "1253")]
    Greek,
    #[strum(serialize = "1254")]
    Turkish,
    #[strum(serialize = "1255")]
    Hebrew,
    #[strum(serialize = "1256")]
    Arabic,
    #[strum(serialize = "1257")]
    Baltic,
    #[strum(serialize = "1258")]
    Vietnam,
    #[strum(serialize = "932")]
    Japanese,
    #[strum(serialize = "936")]
    ChineseSiplified,
    #[strum(serialize = "949")]
    Korean,
    #[strum(serialize = "950")]
    ChineseTraditional,
    #[strum(serialize = "UTF-8")]
    Utf8,
}

#[derive(Debug, Display)]
pub enum CodepageIso {
    #[strum(serialize = "8859-1")]
    Latin1,
    #[strum(serialize = "8859-2")]
    Latin2,
    #[strum(serialize = "8859-3")]
    Latin3,
    #[strum(serialize = "8859-4")]
    Baltic,
    #[strum(serialize = "8859-5")]
    Cyrillic,
    #[strum(serialize = "8859-6")]
    Arabic,
    #[strum(serialize = "8859-7")]
    Greek,
    #[strum(serialize = "8859-8")]
    Hebrew,
    #[strum(serialize = "8859-9")]
    Turkish,
    #[strum(serialize = "8859-10")]
    Latin6,
    #[strum(serialize = "8859-15")]
    Latin9,
}

#[derive(Debug, Display)]
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

#[derive(Debug, Display)]
pub enum Selftest {
    /// Print a self-test page with whole printer information.
    #[strum(serialize = "")]
    All,
    /// Print a pattern to check the status of print head heat line.
    #[strum(serialize = "PATTERN")]
    Pattern,
    /// Print a self-test page with Ethernet settings.
    #[strum(serialize = "ETHERNET")]
    Ethernet,
    /// Print a self-test page with Wi-Fi settings.
    #[strum(serialize = "WLAN")]
    Wlan,
    /// Print a self-test page with RS-232 settings.
    #[strum(serialize = "RS232")]
    Rs232,
    /// Print a self-test page with printer settings.
    #[strum(serialize = "SYSTEM")]
    System,
    /// Print a self-test page with emulated language settings.
    #[strum(serialize = "Z")]
    Z,
    /// Print a self-test page with Bluetooth settings.
    #[strum(serialize = "BT")]
    Bt,
}

#[derive(Debug, Display)]
pub enum Barcode {
    /// Code 128, switching code subset automatically.
    #[strum(serialize = "128")]
    Barcode128,
    /// Code 128, switching code subset manually.
    #[strum(serialize = "128M")]
    Barcode128M,
    /// EAN128, switching code subset automatically.
    #[strum(serialize = "EAN128")]
    BarcodeEan128,
    /// EAN128M, switching code subset manually.
    #[strum(serialize = "EAN128M")]
    BarcodeEan128M,
    /// Interleaved 2 of 5.
    #[strum(serialize = "25")]
    Barcode25,
    /// Interleaved 2 of 5 with check digit.
    #[strum(serialize = "25C")]
    Barcode25C,
    /// Standard 2 of 5.
    #[strum(serialize = "25S")]
    Barcode25S,
    /// Industrial 2 of 5.
    #[strum(serialize = "25I")]
    Barcode25I,
    /// Code 39, switching standard and full ASCII mode automatically
    #[strum(serialize = "39")]
    Barcode39,
    /// Code 39 with check digit.
    #[strum(serialize = "39C")]
    Barcode39C,
    /// Code 93.
    #[strum(serialize = "93")]
    Barcode93,
    /// EAN 13
    #[strum(serialize = "EAN13")]
    BarcodeEan13,
    /// EAN 13 with 2 digits add-on.
    #[strum(serialize = "EAN13+2")]
    BarcodeEan13Plus2,
    /// EAN 13 with 5 digits add-on.
    #[strum(serialize = "EAN13+5")]
    BarcodeEan13Plus5,
    /// EAN 8.
    #[strum(serialize = "EAN8")]
    BarcodeEan8,
    /// EAN 8 with 2 digits add-on.
    #[strum(serialize = "EAN8+2")]
    BarcodeEan8Plus2,
    /// EAN 8 with 5 digits add-on.
    #[strum(serialize = "EAN8+5")]
    BarcodeEan8Plus5,
    /// Codabar.
    #[strum(serialize = "CODA")]
    BarcodeCoda,
    /// Postnet.
    #[strum(serialize = "POST")]
    BarcodePost,
    /// UPC-A
    #[strum(serialize = "UPCA")]
    BarcodeUpca,
    /// UPC-A with 2 digits add-on.
    #[strum(serialize = "UPCA+2")]
    BarcodeUpcaPlus2,
    /// UPC-A with 5 digits add-on.
    #[strum(serialize = "UPCA+5")]
    BarcodeUpaPlus5,
    /// UPC-E
    #[strum(serialize = "UPCE")]
    BarcodeUpce,
    /// UPC-E with 2 digits add-on.
    #[strum(serialize = "UPCE+2")]
    BarcodeUpcePlus2,
    /// UPC-E with 5 digits add-on.
    #[strum(serialize = "UPCE+5")]
    BarcodeUpePlus5,
    /// MSI
    #[strum(serialize = "MSI")]
    BarcodeMsi,
    /// MSI with check digit.
    #[strum(serialize = "MSIC")]
    BarcodeMsic,
    /// PLESSEY.
    #[strum(serialize = "PLESSEY")]
    BarcodePlessey,
    /// China post.
    #[strum(serialize = "CPOST")]
    BarcodeCpost,
    /// ITF14.
    #[strum(serialize = "ITF14")]
    BarcodeItf14,
    /// EAN14.
    #[strum(serialize = "EAN14")]
    BarcodeEan14,
    /// Code 11.
    #[strum(serialize = "11")]
    Barcode11,
    /// Telepen. *Since V6.89EZ.
    #[strum(serialize = "TELEPEN")]
    BarcodeTelepen,
    /// Telepen number. *Since V6.89EZ.
    #[strum(serialize = "TELEPENN")]
    BarcodeTelepenN,
    /// Planet. *Since V6.89EZ.
    #[strum(serialize = "PLANET")]
    BarcodePlanet,
    /// Code 49. *Since V6.89EZ.
    #[strum(serialize = "CODE49")]
    BarcodeCode49,
    /// eutsche Post Identcode. *Since V6.91EZ.
    #[strum(serialize = "DPI")]
    BarcodeDpi,
    /// Deutsche Post Leitcode. *Since V6.91EZ.
    #[strum(serialize = "DPL")]
    BarcodeDpl,
    /// A special use of Code 39. *Since V6.88EZ.
    #[strum(serialize = "LOGMARS")]
    BarcodeLogmars,
}

#[derive(Debug, Display)]
pub enum HumanReadable {
    NotReadable = 0,
    ReadableAlignsToLeft = 1,
    ReadableAlignsToCenter = 2,
    ReadableAlignsToRight = 3,
}

/// Clockwise rotation
#[derive(Debug, Display)]
pub enum Rotation {
    NoRotation = 0,
    Rotation90 = 90,
    Rotation180 = 180,
    Rotation270 = 270,
}

#[derive(Debug, Display)]
pub enum Alignment {
    Default = 0,
    Left = 1,
    Center = 2,
    Right = 3,
}

/// Specifies width in dots for narrow and wide elements respectively.
#[derive(Debug, Display)]
pub enum NarrowWide {
    #[strum(serialize = "1,1")]
    N1W1,
    #[strum(serialize = "1,2")]
    N1W2,
    #[strum(serialize = "1,3")]
    N1W3,
    #[strum(serialize = "2,5")]
    N2W5,
    #[strum(serialize = "3,7")]
    N3W7,
}

#[derive(Debug, Display)]
pub enum BitmapMode {
    Overwrite = 0,
    Or = 1,
    Xor = 2,
}

pub struct Printer {
    file: std::fs::File,
    resolution: u32,
}

impl Printer {
    /// Create a new printer from file. Usually it is somewhere in '/dev/usb/lp*'.
    pub fn new(path: String, tape: Tape) -> Result<Self> {
        let mut file = std::fs::File::options().read(true).write(true).open(path)?;
        let resolution = Self::resolution(&mut file)?;
        let mut printer = Self { file, resolution };

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
                .write_all(format!("SIZE {width},{height}\r\n",).as_bytes())?,
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
    /// `calib` input tuple represent optional parameters
    /// calib.0: Paper length
    /// calib.1: Gap length
    /// If the None is passed then the printer will calibrate and determine the paper length and gap size automatically.
    pub fn gap_detect(&mut self, calib: Option<(Size, Size)>) -> Result<&mut Self> {
        match calib {
            Some((x, y)) => self.file.write_all(
                format!(
                    "GAPDETECT {},{}\r\n",
                    x.to_dots_raw(self.resolution),
                    y.to_dots_raw(self.resolution)
                )
                .as_bytes(),
            )?,
            None => self.file.write_all("GAPDETECT\r\n".as_bytes())?,
        }
        Ok(self)
    }

    /// This command feeds the paper through the black mark sensor in an effort to determine
    /// the paper and black mark sizes, respectively. This command references the user’s approximate measurements.
    /// If the measurements conflict with the actual size, the BLINEDETECT command will not work properly.
    /// This calibration method can be applied to the labels with pre-printed logos or texts.
    ///
    /// `calib` input tuple represent optional parameters
    /// calib.0: Paper length
    /// calib.1: Gap length
    /// If the None is passed then the printer will calibrate and determine the paper length and gap size automatically.
    pub fn bline_detect(&mut self, calib: Option<(Size, Size)>) -> Result<&mut Self> {
        match calib {
            Some((x, y)) => self.file.write_all(
                format!(
                    "BLINEDETECT {},{}\r\n",
                    x.to_dots_raw(self.resolution),
                    y.to_dots_raw(self.resolution)
                )
                .as_bytes(),
            )?,
            None => self.file.write_all("BLINEDETECT\r\n".as_bytes())?,
        }
        Ok(self)
    }

    /// This command feeds the paper through the gap/black mark sensor in an effort to determine
    /// the paper and gap/black mark sizes, respectively. This command references the user’s approximate measurements.
    /// If the measurements conflict with the actual size, the AUTODETECT command will not work properly.
    /// This calibration method can be applied to the labels with pre-printed logos or texts.
    ///
    /// `calib` input tuple represent optional parameters
    /// calib.0: Paper length
    /// calib.1: Gap length
    /// If the None is passed then the printer will calibrate and determine the paper length and gap size automatically.
    pub fn auto_detect(&mut self, calib: Option<(Size, Size)>) -> Result<&mut Self> {
        match calib {
            Some((x, y)) => self.file.write_all(
                format!(
                    "AUTODETECT {},{}\r\n",
                    x.to_dots_raw(self.resolution),
                    y.to_dots_raw(self.resolution)
                )
                .as_bytes(),
            )?,
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
    pub fn reference(&mut self, x: Size, y: Size) -> Result<&mut Self> {
        self.file.write_all(
            format!(
                "REFERENCE {},{}\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            )
            .as_bytes(),
        )?;

        Ok(self)
    }

    /// This command moves the label’s horizontal and vertical position. A positive value moves the label
    /// further from the printing direction; a negative value moves the label towards the printing direction.
    pub fn shift(&mut self, x: Option<Size>, y: Size) -> Result<&mut Self> {
        match x {
            Some(x) => self.file.write_all(
                format!(
                    "SHIFT {},{}\r\n",
                    x.to_dots_raw(self.resolution),
                    y.to_dots_raw(self.resolution)
                )
                .as_bytes(),
            )?,
            None => self
                .file
                .write_all(format!("SHIFT {}\r\n", y.to_dots_raw(self.resolution)).as_bytes())?,
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
        self.file
            .write_all(format!("CODEPAGE {codepage}\r\n").as_bytes())?;

        Ok(self)
    }

    /// This command clears the image buffer.
    pub fn cls(&mut self) -> Result<&mut Self> {
        self.file.write_all("CLS\r\n".as_bytes())?;
        Ok(self)
    }

    /// This command feeds label with the specified length
    pub fn feed(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
        match feed_dot {
            0..=9999 => self
                .file
                .write_all(format!("FEED {feed_dot}\r\n").as_bytes())?,
            _ => {
                return Err(anyhow!(
                    "feed length must be in range 0..9999 in dots, got {:?}",
                    feed_dot
                ))
            }
        }
        Ok(self)
    }

    /// This command feeds the label in reverse.
    /// For TSPL printers only
    pub fn backup(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
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
    pub fn backfeed(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
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
        todo!()
    }

    /// This command can restore printer settings to defaults.
    pub fn initial_printer(&mut self) -> Result<&mut Self> {
        self.file.write_all("INITIALPRINTER\r\n".as_bytes())?;
        Ok(self)
    }

    /// This command can design user's own menu with a database resident on the printer.
    pub fn menu(&self) {
        todo!();
    }

    /// This command draws a bar on the label format.
    pub fn bar(
        &mut self,
        x_upper_left: Size,
        y_upper_left: Size,
        width: Size,
        height: Size,
    ) -> Result<&mut Self> {
        self.file.write_all(
            format!(
                "BAR {},{},{},{}\r\n",
                x_upper_left.to_dots_raw(self.resolution),
                y_upper_left.to_dots_raw(self.resolution),
                width.to_dots_raw(self.resolution),
                height.to_dots_raw(self.resolution)
            )
            .as_bytes(),
        )?;
        Ok(self)
    }

    /// This command prints 1D barcodes.
    pub fn barcode(
        &mut self,
        x: Size,
        y: Size,
        code_type: Barcode,
        height: Size,
        human_readable: HumanReadable,
        rotation: Rotation,
        narrow_wide: NarrowWide,
        alignment: Option<Alignment>,
        content: &str,
    ) -> Result<&mut Self> {
        if let Some(alignment) = alignment {
            self.file.write_all(
                format!(
                    "BARCODE {},{},\"{}\",{},{},{},{},{}, \"{}\"\r\n",
                    x.to_dots_raw(self.resolution),
                    y.to_dots_raw(self.resolution),
                    code_type,
                    height.to_dots_raw(self.resolution),
                    human_readable,
                    rotation,
                    narrow_wide,
                    alignment,
                    content
                )
                .as_bytes(),
            )?;
        } else {
            self.file.write_all(
                format!(
                    "BARCODE {},{},\"{}\",{},{},{},{}, \"{}\"\r\n",
                    x.to_dots_raw(self.resolution),
                    y.to_dots_raw(self.resolution),
                    code_type,
                    height.to_dots_raw(self.resolution),
                    human_readable,
                    rotation,
                    narrow_wide,
                    content
                )
                .as_bytes(),
            )?;
        }
        Ok(self)
    }

    /// This command draws TLC39, TCIF Linked Bar Code 3 of 9, barcode.
    pub fn tlc39(
        &mut self,
        x: Size,
        y: Size,
        rotation: Rotation,
        height: Option<Size>,
        narrow: Option<Size>,
        wide: Option<Size>,
        cellwidth: Option<Size>,
        cellheight: Option<Size>,
        eci_number: &str,
        serial_number: &str,
        additional_data: &str,
    ) -> Result<&mut Self> {
        let x = x.to_dots_raw(self.resolution);
        let y = y.to_dots_raw(self.resolution);
        let height = height
            .unwrap_or(Size::Dots(40))
            .to_dots_raw(self.resolution);
        let narrow = narrow.unwrap_or(Size::Dots(2)).to_dots_raw(self.resolution);
        let wide = wide.unwrap_or(Size::Dots(4)).to_dots_raw(self.resolution);
        let cellwidth = cellwidth
            .unwrap_or(Size::Dots(2))
            .to_dots_raw(self.resolution);
        let cellheight = cellheight
            .unwrap_or(Size::Dots(4))
            .to_dots_raw(self.resolution);

        self.file.write_all(
            format!(
                "TLC39 {},{},{},{},{},{},{},{}, \"{},{},{}\"\r\n",
                x,
                y,
                rotation,
                height,
                narrow,
                wide,
                cellwidth,
                cellheight,
                eci_number,
                serial_number,
                additional_data
            )
            .as_bytes(),
        )?;
        Ok(self)
    }

    /// This command draws bitmap images (as opposed to BMP graphic files).
    pub fn bitmap(
        &mut self,
        x: Size,
        y: Size,
        width_bytes: u16,
        height: Size,
        mode: BitmapMode,
        bitmap_data: Vec<u8>,
    ) -> Result<&mut Self> {
        todo!()
    }

    /// This command draws rectangles on the label.
    pub fn rectangle(
        &mut self,
        x_start: Size,
        y_start: Size,
        x_end: Size,
        y_end: Size,
        thickness: Size,
        radius: Option<Size>,
    ) -> Result<&mut Self> {
        self.file.write_all(
            format!(
                "BOX {},{},{},{},{},{}\r\n",
                x_start.to_dots_raw(self.resolution),
                y_start.to_dots_raw(self.resolution),
                x_end.to_dots_raw(self.resolution),
                y_end.to_dots_raw(self.resolution),
                thickness.to_dots_raw(self.resolution),
                radius.unwrap_or(Size::Dots(0)).to_dots_raw(self.resolution)
            )
            .as_bytes(),
        )?;

        Ok(self)
    }

    /// This command draws a circle on the label.
    pub fn circle(
        &mut self,
        x_start: Size,
        y_start: Size,
        diameter: Size,
        thickness: Size,
    ) -> Result<&mut Self> {
        self.file.write_all(
            format!(
                "CIRCLE {},{},{},{}\r\n",
                x_start.to_dots_raw(self.resolution),
                y_start.to_dots_raw(self.resolution),
                diameter.to_dots_raw(self.resolution),
                thickness.to_dots_raw(self.resolution)
            )
            .as_bytes(),
        )?;
        Ok(self)
    }

    /// This command draws an ellipse on the label.
    pub fn ellipse(
        &mut self,
        x_upper_left: Size,
        y_upper_left: Size,
        width: Size,
        height: Size,
        thickness: Size,
    ) -> Result<&mut Self> {
        self.file.write_all(
            format!(
                "ELLIPSE {},{},{},{},{}\r\n",
                x_upper_left.to_dots_raw(self.resolution),
                y_upper_left.to_dots_raw(self.resolution),
                width.to_dots_raw(self.resolution),
                height.to_dots_raw(self.resolution),
                thickness.to_dots_raw(self.resolution)
            )
            .as_bytes(),
        )?;
        Ok(self)
    }

    fn resolution(file: &mut File) -> Result<u32> {
        file.write_all("GETSETTINGS$(\"INFORMATION\",\"DPI\")\r\n".as_bytes())?;
        let mut res = String::new();
        file.read_to_string(&mut res)?;
        Ok(res.parse::<u32>()?)
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
