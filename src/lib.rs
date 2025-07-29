use anyhow::{anyhow, Ok, Result};
use log::debug;
#[cfg(target_os = "windows")]
use std::ffi::CString;
use std::fmt::Display;
use strum_macros::Display;

#[derive(Debug, Clone)]
pub enum Size {
    Imperial(f32),
    Metric(f32),
    Dots(u32),
}

impl Size {
    fn to_dots_raw(&self, resolution: u32) -> u32 {
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

#[derive(Debug, Display)]
pub enum Country {
    #[strum(serialize = "1")]
    Usa = 1,
    #[strum(serialize = "2")]
    CanadianFrench = 2,
    #[strum(serialize = "3")]
    SpanishLatinAmerica = 3,
    #[strum(serialize = "31")]
    Dutch = 31,
    #[strum(serialize = "32")]
    Belgian = 32,
    #[strum(serialize = "33")]
    French = 33,
    #[strum(serialize = "34")]
    Spanish = 34,
    #[strum(serialize = "36")]
    Hungarian = 36,
    #[strum(serialize = "38")]
    Yugoslavian = 38,
    #[strum(serialize = "39")]
    Italian = 39,
    #[strum(serialize = "41")]
    Switzerland = 41,
    #[strum(serialize = "42")]
    Slovak = 42,
    #[strum(serialize = "44")]
    UnitedKingdom = 44,
    #[strum(serialize = "45")]
    Danish = 45,
    #[strum(serialize = "46")]
    Swedish = 46,
    #[strum(serialize = "47")]
    Norwegian = 47,
    #[strum(serialize = "48")]
    Polish = 48,
    #[strum(serialize = "49")]
    German = 49,
    #[strum(serialize = "55")]
    Brazil = 55,
    #[strum(serialize = "61")]
    English = 61,
    #[strum(serialize = "351")]
    Portuguese = 351,
    #[strum(serialize = "358")]
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
    pub width: Size,
    pub height: Option<Size>,
    pub gap: Size,
    pub gap_offset: Option<Size>,
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
pub enum RssType {
    ///RSS14,
    #[strum(serialize = "RSS14")]
    Rss14,
    ///RSS14 Truncated,
    #[strum(serialize = "RSS14T")]
    Rss14T,
    ///RSS14 Stacked,
    #[strum(serialize = "RSS14S")]
    Rss14S,
    ///RSS14 Stacked Omnidirectional,
    #[strum(serialize = "RSS14SO")]
    Rss14So,
    ///RSS Limited,
    #[strum(serialize = "RSSLIM")]
    RssLim,
    ///RSS Expanded,
    #[strum(serialize = "RSSEXP")]
    RssExp,
    ///UPC-A,
    #[strum(serialize = "UPCA")]
    UpcA,
    ///UPC-E,
    #[strum(serialize = "UPCE")]
    UpcE,
    ///EAN13,
    #[strum(serialize = "EAN13")]
    Ean13,
    ///EAN8,
    #[strum(serialize = "EAN8")]
    Ean8,
    ///UCC/EAN-128 & CC-A/B,
    #[strum(serialize = "UCC128CCA")]
    Ucc128Cca,
    ///UCC/EAN-128 & CC-C,
    #[strum(serialize = "UCC128CCC")]
    Ucc128Ccc,
}

#[derive(Debug, Display)]
pub enum Font {
    /// Monotye CG Triumvirate Bold Condensed, font width and height is stretchable
    #[strum(serialize = "0")]
    FontMonotye,
    /// 8 x 12 fixed pitch dot font
    #[strum(serialize = "1")]
    Font8x12,
    /// 12 x 20 fixed pitch dot font
    #[strum(serialize = "2")]
    Font12x20,
    /// 16 x 24 fixed pitch dot font
    #[strum(serialize = "3")]
    Font16x24,
    /// 24 x 32 fixed pitch dot font
    #[strum(serialize = "4")]
    Font24x32,
    /// 32 x 48 dot fixed pitch font
    #[strum(serialize = "5")]
    Font32x48,
    /// 14 x 19 dot fixed pitch font OCR-B
    #[strum(serialize = "6")]
    Font14x19,
    /// 21 x 27 dot fixed pitch font OCR-B
    #[strum(serialize = "7")]
    Font21x27,
    /// 14 x25 dot fixed pitch font OCR-A
    #[strum(serialize = "8")]
    Font14x25,
    /// Monotye CG Triumvirate Bold Condensed, font width and height proportion is fixed
    #[strum(serialize = "ROMAN.TTF")]
    FontRoman,
    /// EPL2 font 1
    #[strum(serialize = "1.EFT")]
    FontEpl1,
    /// EPL2 font 2
    #[strum(serialize = "2.EFT")]
    FontEpl2,
    /// EPL2 font 3
    #[strum(serialize = "3.RFT")]
    FontEpl3,
    /// EPL2 font 4
    #[strum(serialize = "4.EFT")]
    FontEpl4,
    /// EPL2 font 5
    #[strum(serialize = "5.EFT")]
    FontEpl5,
    /// ZPL2 font A
    #[strum(serialize = "A.FNT")]
    FontZplA,
    /// ZPL2 font A
    #[strum(serialize = "B.FNT")]
    FontZplB,
    /// ZPL2 font D
    #[strum(serialize = "D.FNT")]
    FontZplD,
    /// ZPL2 font E8
    #[strum(serialize = "E8.FNT")]
    FontZplE8,
    /// ZPL2 font F
    #[strum(serialize = "F.FNT")]
    FontZplF,
    /// ZPL2 font G
    #[strum(serialize = "G.FNT")]
    FontZplG,
    /// ZPL2 font H8
    #[strum(serialize = "H8.FNT")]
    FontZplH8,
    /// ZPL2 font GS
    #[strum(serialize = "GS.FNT")]
    FontZplGs,
}

#[derive(Debug, Display)]
pub enum HumanReadable {
    #[strum(serialize = "0")]
    NotReadable = 0,
    #[strum(serialize = "1")]
    ReadableAlignsToLeft = 1,
    #[strum(serialize = "2")]
    ReadableAlignsToCenter = 2,
    #[strum(serialize = "3")]
    ReadableAlignsToRight = 3,
}

/// Clockwise rotation
#[derive(Debug, Display)]
pub enum Rotation {
    #[strum(serialize = "0")]
    NoRotation = 0,
    #[strum(serialize = "90")]
    Rotation90 = 90,
    #[strum(serialize = "180")]
    Rotation180 = 180,
    #[strum(serialize = "270")]
    Rotation270 = 270,
}

#[derive(Debug, Display)]
pub enum Alignment {
    #[strum(serialize = "0")]
    Default = 0,
    #[strum(serialize = "1")]
    Left = 1,
    #[strum(serialize = "2")]
    Center = 2,
    #[strum(serialize = "3")]
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
    #[strum(serialize = "0")]
    Overwrite = 0,
    #[strum(serialize = "1")]
    Or = 1,
    #[strum(serialize = "2")]
    Xor = 2,
}

#[derive(Debug, Display)]
pub enum QrCodeJustification {
    #[strum(serialize = "J1")]
    UpperLeft,
    #[strum(serialize = "J2")]
    UpperCenter,
    #[strum(serialize = "J3")]
    UpperRight,
    #[strum(serialize = "J4")]
    CenterLeft,
    #[strum(serialize = "J5")]
    Center,
    #[strum(serialize = "J6")]
    CenterRight,
    #[strum(serialize = "J7")]
    BottomLeft,
    #[strum(serialize = "J8")]
    BottomCenter,
    #[strum(serialize = "J9")]
    BottomRight,
}

impl Into<&'static str> for QrCodeJustification {
    fn into(self) -> &'static str {
        match self {
            QrCodeJustification::UpperLeft => "J1",
            QrCodeJustification::UpperCenter => "J2",
            QrCodeJustification::UpperRight => "J3",
            QrCodeJustification::CenterLeft => "J4",
            QrCodeJustification::Center => "J5",
            QrCodeJustification::CenterRight => "J6",
            QrCodeJustification::BottomLeft => "J7",
            QrCodeJustification::BottomCenter => "J8",
            QrCodeJustification::BottomRight => "J9",
        }
    }
}

#[derive(Debug, Display)]
pub enum QrCodeModel {
    M1, //(default), original version
    M2, //enhanced version (Almost smart phone is supported by this version.)
}

impl Into<&'static str> for QrCodeModel {
    fn into(self) -> &'static str {
        match self {
            QrCodeModel::M1 => "M1",
            QrCodeModel::M2 => "M2",
        }
    }
}

// don't know what meant
#[derive(Debug, Display)]
pub enum QrCodeMask {
    S0,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7, // default
    S8,
}

impl Into<&'static str> for QrCodeMask {
    fn into(self) -> &'static str {
        match self {
            QrCodeMask::S0 => "S0",
            QrCodeMask::S1 => "S1",
            QrCodeMask::S2 => "S2",
            QrCodeMask::S3 => "S3",
            QrCodeMask::S4 => "S4",
            QrCodeMask::S5 => "S5",
            QrCodeMask::S6 => "S6",
            QrCodeMask::S7 => "S7",
            QrCodeMask::S8 => "S8",
        }
    }
}

pub struct Printer {
    #[cfg(target_os = "linux")]
    file: std::fs::File,
    #[cfg(target_os = "windows")]
    printer_name: CString,
    resolution: u32,
}

impl Printer {
    /// Create a new printer with predefined resolution.
    pub fn with_resolution(path: &str, tape: Tape, dpi: u32) -> Result<Self> {
        let mut printer = Self {
            #[cfg(target_os = "linux")]
            file: std::fs::File::options().read(true).write(true).open(path)?,
            #[cfg(target_os = "windows")]
            printer_name: CString::new(path).unwrap_or_default(),
            resolution: dpi,
        };

        printer
            .size(tape.width, tape.height)?
            .gap(tape.gap, tape.gap_offset)?
            .cls()?;

        Ok(printer)
    }

    /// copy from raw-printer
    #[cfg(target_os = "windows")]
    fn write_command(&mut self, bytes: &[u8]) -> Result<()> {
        use std::ffi::CString;
        use std::ptr;
        use windows::Win32::Graphics::Printing::{
            ClosePrinter, EndDocPrinter, EndPagePrinter, OpenPrinterA, StartDocPrinterA,
            StartPagePrinter, WritePrinter, DOC_INFO_1A, PRINTER_ACCESS_USE, PRINTER_DEFAULTSA,
            PRINTER_HANDLE,
        };

        let mut printer_handle = PRINTER_HANDLE {
            Value: std::ptr::null_mut(),
        };

        // Open the printer
        unsafe {
            let pd = PRINTER_DEFAULTSA {
                pDatatype: windows::core::PSTR(ptr::null_mut()),
                pDevMode: ptr::null_mut(),
                DesiredAccess: PRINTER_ACCESS_USE,
            };

            if OpenPrinterA(
                windows::core::PCSTR(self.printer_name.as_bytes().as_ptr()),
                &mut printer_handle,
                Some(&pd),
            )
            .is_ok()
            {
                let doc_name_cstring = CString::new("Print Job").unwrap_or_default();

                let doc_info = DOC_INFO_1A {
                    pDocName: windows::core::PSTR(doc_name_cstring.as_ptr() as *mut u8),
                    pOutputFile: windows::core::PSTR::null(),
                    pDatatype: windows::core::PSTR("RAW\0".as_ptr() as *mut u8),
                };

                // Start the document
                let job = StartDocPrinterA(printer_handle, 1, &doc_info as *const _ as _);
                if job == 0 {
                    return Err(anyhow!(windows::core::Error::from_win32()));
                }

                // Start the page
                if !StartPagePrinter(printer_handle).as_bool() {
                    return Err(anyhow!(windows::core::Error::from_win32()));
                }

                let mut bytes_written: u32 = 0;
                if !WritePrinter(
                    printer_handle,
                    bytes.as_ptr() as _,
                    bytes.len() as u32,
                    &mut bytes_written,
                )
                .as_bool()
                {
                    return Err(anyhow!(windows::core::Error::from_win32()));
                }

                // End the page and document
                let _ = EndPagePrinter(printer_handle);
                let _ = EndDocPrinter(printer_handle);
                let _ = ClosePrinter(printer_handle);
                Ok(())
            } else {
                Err(anyhow!(windows::core::Error::from_win32()))
            }
        }
    }

    #[inline]
    #[cfg(target_os = "linux")]
    fn write_command(&mut self, bytes: &[u8]) -> Result<()> {
        use std::io::Write;

        self.file.write_all(bytes)?;
        Ok(())
    }

    /// This command defines the label width and height.
    /// Label length must be provided for firmware version <V8.13
    fn size(&mut self, width: Size, height: Option<Size>) -> Result<&mut Self> {
        let cmd = match height {
            Some(height) => format!("SIZE {width},{height}\r\n"),
            None => format!("SIZE {width}\r\n"),
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

        Ok(self)
    }

    /// Defines the gap distance between two labels.
    /// Optional offset distance of the gap may be provided
    fn gap(&mut self, gap: Size, gap_offset: Option<Size>) -> Result<&mut Self> {
        let cmd = match gap_offset {
            Some(offset) => format!("GAP {gap},{offset}\r\n"),
            None => format!("GAP {gap}\r\n"),
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

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
        let cmd = match calib {
            Some((x, y)) => &format!(
                "GAPDETECT {},{}\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => "GAPDETECT\r\n",
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
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
        let cmd = match calib {
            Some((x, y)) => &format!(
                "BLINEDETECT {},{}\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => "BLINEDETECT\r\n",
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
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
        let cmd = match calib {
            Some((x, y)) => &format!(
                "AUTODETECT {},{}\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => "AUTODETECT\r\n",
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command sets the height of the black line and the user-defined extra label feeding length each form feed takes.
    /// Both parameters should be in the same measurement type (mm/inch/dot)
    pub fn bline(&mut self, black_line_height: Size, extra_feeding_len: Size) -> Result<&mut Self> {
        let cmd = format!("BLINE {black_line_height},{extra_feeding_len}\r\n");

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command defines the selective, extra label feeding length each form feed takes, which,
    /// especially in peel-off mode and cutter mode, is used to adjust label stop position,
    /// so as for label to register at proper places for the intended purposes.
    /// The printer back tracks the extra feeding length before the next run of printing.
    pub fn offset(&mut self, offset: Size) -> Result<&mut Self> {
        let cmd = format!("OFFSET {offset}\r\n");
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command defines the print speed.
    /// Available speeds in inch/sec should be checked for your printer model
    pub fn speed(&mut self, speed: &str) -> Result<&mut Self> {
        let cmd = format!("SPEED {speed}\r\n");
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command sets the printing darkness from lightest(0) to darkest(15). Default density is 8.
    pub fn density(&mut self, density: u8) -> Result<&mut Self> {
        let cmd = match density {
            1..=15 => format!("DENSITY {density}\r\n"),
            _ => return Err(anyhow!("Density should be in range 0..15")),
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command defines the printout direction and mirror image. This will be stored in the printer memory.
    pub fn direction(
        &mut self,
        reversed_direction: bool,
        mirrored_image: bool,
    ) -> Result<&mut Self> {
        let cmd = format!(
            "DIRECTION {},{}\r\n",
            reversed_direction as u8, mirrored_image as u8
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

        Ok(self)
    }

    /// This command defines the reference point of the label. The reference (origin) point varies with the print direction.
    pub fn reference(&mut self, x: Size, y: Size) -> Result<&mut Self> {
        let cmd = format!(
            "REFERENCE {},{}\r\n",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution)
        );

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command moves the label’s horizontal and vertical position. A positive value moves the label
    /// further from the printing direction; a negative value moves the label towards the printing direction.
    pub fn shift(&mut self, x: Option<Size>, y: Size) -> Result<&mut Self> {
        let cmd = match x {
            Some(x) => format!(
                "SHIFT {},{}\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => format!("SHIFT {}\r\n", y.to_dots_raw(self.resolution)),
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command orients the keyboard for use in different countries via
    /// defining special characters on the KP-200 series portable LCD keyboard (option).
    pub fn country(&mut self, country: Country) -> Result<&mut Self> {
        let cmd = format!("COUNTRY {:03}\r\n", country as u16);
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command defines the code page of international character set.
    pub fn codepage(&mut self, codepage: Codepage) -> Result<&mut Self> {
        let cmd = format!("CODEPAGE {codepage}\r\n");
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command clears the image buffer.
    pub fn cls(&mut self) -> Result<&mut Self> {
        let cmd = "CLS\r\n";
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command feeds label with the specified length
    pub fn feed(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
        let cmd = match feed_dot {
            0..=9999 => format!("FEED {feed_dot}\r\n"),
            _ => {
                return Err(anyhow!(
                    "feed length must be in range 0..9999 in dots, got {:?}",
                    feed_dot
                ))
            }
        };
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command feeds the label in reverse.
    /// For TSPL printers only
    pub fn backup(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
        let cmd = match feed_dot {
            0..=9999 => format!("BACKUP {feed_dot}\r\n"),
            _ => {
                return Err(anyhow!(
                    "backup length must be in range 0..9999, got {:?}",
                    feed_dot
                ))
            }
        };
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command feeds the label in reverse. The length is specified by dot.
    /// For TSPL2 printers only
    pub fn backfeed(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
        let cmd = match feed_dot {
            0..=9999 => format!("BACKFEED {feed_dot}\r\n"),
            _ => {
                return Err(anyhow!(
                    "backfeed length must be in range 0..9999, got {:?}",
                    feed_dot
                ))
            }
        };
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command feeds label to the beginning of next label.
    pub fn formfeed(&mut self) -> Result<&mut Self> {
        let cmd = "FORMFEED\r\n";
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command will feed label until the internal sensor has determined the origin.
    /// Size and gap of the label should be defined before using this command.
    /// For TSPL programming printer: Back label to origin position.
    /// For TSPL2 programming printer: Feed label to origin position
    pub fn home(&mut self) -> Result<&mut Self> {
        let cmd = "HOME\r\n";
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command prints the label format currently stored in the image buffer.
    pub fn print(&mut self, sets: u32, copies: Option<u32>) -> Result<&mut Self> {
        let cmd = match sets {
            1..=999999999 => {
                if let Some(copies) = copies {
                    match copies {
                        1..=999999999 => format!("PRINT {sets},{copies}\r\n"),
                        _ => {
                            return Err(anyhow!(
                                "Copies qty must be in range 1..999999999, got {:?}",
                                copies
                            ))
                        }
                    }
                } else {
                    format!("PRINT {sets}\r\n")
                }
            }
            _ => {
                return Err(anyhow!(
                    "Sets qty must be in range 1..999999999, got {:?}",
                    sets
                ))
            }
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command controls the sound frequency of the beeper. There are 10 levels of sounds, from 0 to 9.
    /// The timing control can be set by the "interval" parameter, in range 1..4095
    pub fn sound(&mut self, level: u8, interval: u16) -> Result<&mut Self> {
        let cmd = match (level, interval) {
            (0..=9, 1..=4095) => format!("SOUND {level},{interval}\r\n"),
            _ => return Err(anyhow!("wrong sound parameters")),
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command activates the cutter to immediately cut the labels without back feeding the label.
    pub fn cut(&mut self) -> Result<&mut Self> {
        let cmd = "CUT\r\n";
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
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
        let cmd = match minpaper_maxgap {
            Some((x, y)) => format!("LIMITFEED {n},{x},{y}\r\n"),
            None => format!("LIMITFEED {n}\r\n"),
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

        Ok(self)
    }

    /// At this command, the printer will print out the printer information.
    pub fn selftest(&mut self, test_kind: Selftest) -> Result<&mut Self> {
        let cmd = format!("SELFTEST {test_kind}\r\n");
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// Let the printer wait until process of commands (before EOJ) be finished then go on the next command.
    pub fn eoj(&mut self) -> Result<&mut Self> {
        let cmd = "EOJ\r\n";
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// Let the printer wait specific period of time then go on next command.
    pub fn delay(&mut self, delay: std::time::Duration) -> Result<&mut Self> {
        let cmd = format!("DELAY {}\r\n", delay.as_millis());
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command can show the image, which is in printer’s image buffer, on LCD panel.
    pub fn display(&self) {
        unimplemented!()
    }

    /// This command can restore printer settings to defaults.
    pub fn initial_printer(&mut self) -> Result<&mut Self> {
        let cmd = "INITIALPRINTER\r\n";
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command can design user's own menu with a database resident on the printer.
    pub fn menu(&self) {
        unimplemented!()
    }

    /// This command draws a bar on the label format.
    pub fn bar(
        &mut self,
        x_upper_left: Size,
        y_upper_left: Size,
        width: Size,
        height: Size,
    ) -> Result<&mut Self> {
        let cmd = format!(
            "BAR {},{},{},{}\r\n",
            x_upper_left.to_dots_raw(self.resolution),
            y_upper_left.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution)
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
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
        rotate: Rotation,
        narrow_wide: NarrowWide,
        alignment: Option<Alignment>,
        content: &str,
    ) -> Result<&mut Self> {
        let cmd = if let Some(alignment) = alignment {
            format!(
                "BARCODE {},{},\"{}\",{},{},{},{},{}, \"{}\"\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution),
                code_type,
                height.to_dots_raw(self.resolution),
                human_readable,
                rotate,
                narrow_wide,
                alignment,
                content
            )
        } else {
            format!(
                "BARCODE {},{},\"{}\",{},{},{},{}, \"{}\"\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution),
                code_type,
                height.to_dots_raw(self.resolution),
                human_readable,
                rotate,
                narrow_wide,
                content
            )
        };

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command draws TLC39, TCIF Linked Bar Code 3 of 9, barcode.
    pub fn tlc39(
        &mut self,
        x: Size,
        y: Size,
        rotate: Rotation,
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

        let cmd = format!(
            "TLC39 {},{},{},{},{},{},{},{}, \"{},{},{}\"\r\n",
            x,
            y,
            rotate,
            height,
            narrow,
            wide,
            cellwidth,
            cellheight,
            eci_number,
            serial_number,
            additional_data
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command draws bitmap images (as opposed to BMP graphic files).
    pub fn bitmap(
        &mut self,
        x: Size,
        y: Size,
        width_bytes: u16,
        height_dots: u16,
        mode: BitmapMode,
        bitmap_data: Vec<u8>,
    ) -> Result<&mut Self> {
        let crlf = vec![b'\r', b'\n'];
        let mut cmd = format!(
            "BITMAP {},{},{},{},{},",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution),
            width_bytes,
            height_dots,
            mode
        )
        .as_bytes()
        .to_vec();
        cmd.extend(bitmap_data);
        cmd.extend(crlf);

        self.write_command(&cmd)?;

        Ok(self)
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
        let cmd = format!(
            "BOX {},{},{},{},{},{}\r\n",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            x_end.to_dots_raw(self.resolution),
            y_end.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution),
            radius.unwrap_or(Size::Dots(0)).to_dots_raw(self.resolution)
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

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
        let cmd = format!(
            "CIRCLE {},{},{},{}\r\n",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            diameter.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution)
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
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
        let cmd = format!(
            "ELLIPSE {},{},{},{},{}\r\n",
            x_upper_left.to_dots_raw(self.resolution),
            y_upper_left.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution)
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command draws CODABLOCK F mode barcode.
    pub fn codablock(
        &mut self,
        x: Size,
        y: Size,
        rotate: Rotation,
        row_height: Option<Size>,
        module_width: Option<Size>,
        content: &str,
    ) -> Result<&mut Self> {
        let row_height = row_height
            .unwrap_or(Size::Dots(8))
            .to_dots_raw(self.resolution);
        let module_width = module_width
            .unwrap_or(Size::Dots(8))
            .to_dots_raw(self.resolution);

        let cmd = format!(
            "CODABLOCK {},{},{},{},{},\"{}\"\r\n",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution),
            rotate,
            row_height,
            module_width,
            content
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

        Ok(self)
    }

    /// This command defines a DataMatrix 2D bar code. Currently, only ECC200 error correction is supported.
    pub fn data_matrix(
        &mut self,
        x: Size,
        y: Size,
        exp_width: Size,
        exp_height: Size,
        escape_symbol: Option<char>,
        module_size: Option<Size>,
        rotate: Option<Rotation>,
        rectangular: Option<bool>,
        row_size: Option<u8>,
        col_size: Option<u8>,
        content: &str,
    ) -> Result<&mut Self> {
        let mut cmd = format!(
            "DMATRIX {},{},{},{},",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution),
            exp_width.to_dots_raw(self.resolution),
            exp_height.to_dots_raw(self.resolution)
        );

        if let Some(c) = escape_symbol {
            let c = c as u8;
            if (0..=127).contains(&c) {
                cmd.push_str(&format!("c{},", c));
            } else {
                return Err(anyhow!("Wrong ASCII symbol"));
            }
        }

        if let Some(x) = module_size {
            cmd.push_str(&format!("x{},", x.to_dots_raw(self.resolution)));
        }

        if let Some(r) = rotate {
            cmd.push_str(&format!("r{},", r));
        }

        if let Some(a) = rectangular {
            let a = if a { 1 } else { 0 };
            cmd.push_str(&format!("a{},", a));
        }

        if let Some(row) = row_size {
            if (10..=144).contains(&row) {
                cmd.push_str(&format!("{row},"));
            } else {
                return Err(anyhow!("Row size doesnt match limit [10;144]"));
            }
        }

        if let Some(col) = col_size {
            if (10..=144).contains(&col) {
                cmd.push_str(&format!("{col},"));
            } else {
                return Err(anyhow!("Column size doesnt match limit [10;144]"));
            }
        }

        cmd.push_str(&format!(" \"{}\"\r\n", content));

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command clears a specified region in the image buffer.
    pub fn erase(&mut self, x: Size, y: Size, width: Size, height: Size) -> Result<&mut Self> {
        let cmd = format!(
            "ERASE {},{},{},{}\r\n",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution)
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command defines a PDF417 2D bar code.
    pub fn pdf417(
        &mut self,
        x_start: Size,
        y_start: Size,
        width: Size,
        height: Size,
        rotate: Rotation,
        content: &str,
    ) -> Result<&mut Self> {
        let cmd = format!(
            "PDF417 {},{},{},{},{},\"{}\"\r\n",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution),
            rotate,
            content
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command defines a AZTEC 2D bar code.
    pub fn aztec(
        &mut self,
        x_start: Size,
        y_start: Size,
        rotate: Rotation,
        size: u8,
        ecp: u16,
        flg: bool,
        menu: bool,
        multi: u8,
        reversed: bool,
        content: &str,
    ) -> Result<&mut Self> {
        if !(1..=20).contains(&size) {
            return Err(anyhow!("Wrong size settings. min: 1, max: 20"));
        }
        if ecp > 300 {
            return Err(anyhow!("Wrong error control parameter. Max: 300"));
        }
        if !(1..=26).contains(&multi) {
            return Err(anyhow!("Wrong number of symbols. min: 1, max: 26"));
        }

        let cmd = format!(
            "AZTEC {},{},{},{},{},{},{},{},{},{},{}\r\n",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            rotate,
            size,
            ecp,
            flg as u8,
            menu as u8,
            multi,
            reversed as u8,
            content.as_bytes().len(),
            content
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

        Ok(self)
    }

    /// This command defines a Micro PDF 417 bar code.
    pub fn mpdf417(
        &mut self,
        x_start: Size,
        y_start: Size,
        rotate: Rotation,
        module_width: Option<Size>,
        module_height: Option<Size>,
        col_num: Option<usize>,
        content: &str,
    ) -> Result<&mut Self> {
        let col_num = match col_num {
            Some(x) => match x {
                1..=4 => x,
                _ => 0,
            },
            _ => 0,
        };

        let module_width = module_width
            .unwrap_or(Size::Dots(1))
            .to_dots_raw(self.resolution);
        let module_height = module_height
            .unwrap_or(Size::Dots(10))
            .to_dots_raw(self.resolution);

        let cmd = format!(
            "MPDF417 {},{},{},{},{},{}, \"{}\"\r\n",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            rotate,
            module_width,
            module_height,
            col_num,
            content,
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;

        Ok(self)
    }

    /// This command prints QR code.
    pub fn qrcode(
        &mut self,
        x_upper_left: Size,
        y_upper_left: Size,
        ecc_level: u8,
        cellwidth_dot: u8,
        rotate: Rotation,
        justification: Option<QrCodeJustification>,
        model: Option<QrCodeModel>,
        mask: Option<QrCodeMask>,
        area: Option<u32>,
        content: &str,
    ) -> Result<&mut Self> {
        let ecc_level = match ecc_level {
            0..=6 => 'L',
            7..=14 => 'M',
            15..=24 => 'Q',
            _ => 'H',
        };
        if !(1..=10).contains(&cellwidth_dot) {
            return Err(anyhow!("Wrong cellwidth value. min: 1, max: 10"));
        }

        let mut cmd = format!(
            "QRCODE {},{},{},{},A,{},",
            x_upper_left.to_dots_raw(self.resolution),
            y_upper_left.to_dots_raw(self.resolution),
            ecc_level,
            cellwidth_dot,
            rotate,
        );
        if let Some(justification) = justification {
            cmd.push_str(justification.into());
            cmd.push(',');
        }
        if let Some(model) = model {
            cmd.push_str(model.into());
            cmd.push(',');
        }
        if let Some(mask) = mask {
            cmd.push_str(mask.into());
            cmd.push(',');
        }
        if let Some(area) = area {
            cmd.push_str(&format!("X{area},"));
        }
        cmd.push('"');
        cmd.push_str(content);
        cmd.push_str("\"\r\n");

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command is used to draw a RSS bar code on the label format
    pub fn rss(
        &mut self,
        x_upper_left: Size,
        y_upper_left: Size,
        rss_type: RssType,
        rotate: Rotation,
        module_width: Size,
        separator_height: usize,
        seg_width: Option<usize>,
        lin_height: Option<usize>,
        content: &str,
    ) -> Result<&mut Self> {
        let pix_mult = module_width.to_dots_raw(self.resolution);
        if !(1..=10).contains(&pix_mult) {
            return Err(anyhow!("Wrong module resolution"));
        }

        if separator_height != 1 && separator_height != 2 {
            return Err(anyhow!("Wrong separator height"));
        }

        let cmd = match rss_type {
            RssType::RssExp => match seg_width {
                Some(seg_width) => {
                    if !(2..=22).contains(&seg_width) {
                        return Err(anyhow!("Wrong segment width. 2 to 22 accepted"));
                    }
                    format!(
                        "RSS {},{}, \"{}\",{},{},{},{}, \"{}\"\r\n",
                        x_upper_left.to_dots_raw(self.resolution),
                        y_upper_left.to_dots_raw(self.resolution),
                        rss_type,
                        rotate,
                        pix_mult,
                        separator_height,
                        seg_width,
                        content
                    )
                }
                None => return Err(anyhow!("Missed segment width")),
            },
            RssType::Ucc128Cca | RssType::Ucc128Ccc => match lin_height {
                Some(lin_height) => {
                    if !(1..=500).contains(&lin_height) {
                        return Err(anyhow!("Wrong line height. 1 to 500 accepted"));
                    }
                    format!(
                        "RSS {},{}, \"{}\",{},{},{},{}, \"{}\"\r\n",
                        x_upper_left.to_dots_raw(self.resolution),
                        y_upper_left.to_dots_raw(self.resolution),
                        rss_type,
                        rotate,
                        pix_mult,
                        separator_height,
                        lin_height,
                        content
                    )
                }
                None => return Err(anyhow!("UCC/EAN-128 height missed")),
            },
            _ => {
                format!(
                    "RSS {},{}, \"{}\",{},{},{}, \"{}\"\r\n",
                    x_upper_left.to_dots_raw(self.resolution),
                    y_upper_left.to_dots_raw(self.resolution),
                    rss_type,
                    rotate,
                    pix_mult,
                    separator_height,
                    content
                )
            }
        };
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command reverses a region in image buffer.
    pub fn reverse(
        &mut self,
        x_start: Size,
        y_start: Size,
        width: Size,
        height: Size,
    ) -> Result<&mut Self> {
        let cmd = format!(
            "REVERSE {},{},{},{}\r\n",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution)
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    /// This command is used to draw a diagonal.
    pub fn diagonal(
        &mut self,
        x_start: Size,
        y_start: Size,
        x_end: Size,
        y_end: Size,
        thickness: Size,
    ) -> Result<&mut Self> {
        let cmd = format!(
            "DIAGONAL {},{},{},{},{}\r\n",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            x_end.to_dots_raw(self.resolution),
            y_end.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution)
        );
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    pub fn text(
        &mut self,
        x: Size,
        y: Size,
        font: Font,
        rotate: Rotation,
        multiply_x: u8,
        multiply_y: u8,
        alignment: Option<Alignment>,
        content: &str,
    ) -> Result<&mut Self> {
        if !(1..=10).contains(&multiply_x) || !(1..=10).contains(&multiply_y) {
            return Err(anyhow!("Wrong multiplication. Should be in range 1-10"));
        }
        let cmd = match alignment {
            Some(alignment) => format!(
                "TEXT {},{},\"{}\",{},{},{},{}, \"{}\"\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution),
                font,
                rotate,
                multiply_x,
                multiply_y,
                alignment,
                content
            ),
            None => format!(
                "TEXT {},{},\"{}\",{},{},{}, \"{}\"\r\n",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution),
                font,
                rotate,
                multiply_x,
                multiply_y,
                content
            ),
        };
        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }

    pub fn block(
        &mut self,
        x: Size,
        y: Size,
        width: Size,
        height: Size,
        font: Font,
        rotate: Rotation,
        multiply_x: u8,
        multiply_y: u8,
        space: Option<Size>,
        alignment: Option<Alignment>,
        fit: Option<bool>,
        content: &str,
    ) -> Result<&mut Self> {
        if !(1..=10).contains(&multiply_x) || !(1..=10).contains(&multiply_y) {
            return Err(anyhow!("Wrong multiplication. Should be in range 1-10"));
        }

        if content.len() > 4096 {
            return Err(anyhow!("Overflow. Max content length 4096"));
        }

        let mut cmd = format!(
            "TEXT {},{},{},{},\"{}\",{},{},{},",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution),
            font,
            rotate,
            multiply_x,
            multiply_y,
        );

        if let Some(space) = space {
            cmd.push_str(&format!("{},", space.to_dots_raw(self.resolution)));
        }

        if let Some(alignment) = alignment {
            cmd.push_str(&format!("{},", alignment));
        }
        if let Some(fit) = fit {
            cmd.push_str(&format!("{},", fit as u8));
        }

        cmd.push_str(&format!("\"{}\"\r\n", content));

        debug!("{cmd}");
        self.write_command(cmd.as_bytes())?;
        Ok(self)
    }
}
