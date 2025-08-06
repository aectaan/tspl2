mod barcode;
mod codepage;
mod country;
mod font;
mod miscellaneous;
mod rss_type;
mod selftest;
mod size;

pub use barcode::Barcode;
pub use codepage::Codepage;
pub use country::Country;
pub use font::Font;
pub use miscellaneous::{
    Alignment, BitmapMode, HumanReadable, NarrowWide, QrCodeJustification, Rotation,
};
pub use rss_type::RssType;
pub use selftest::Selftest;
pub use size::Size;

use anyhow::{anyhow, Ok, Result};
use std::io::Write;

#[derive(Debug)]
pub struct Tape {
    pub width: Size,
    pub height: Option<Size>,
    pub gap: Size,
    pub gap_offset: Option<Size>,
}

pub struct Printer {
    file: std::fs::File,
    resolution: u32,
}

impl Printer {
    /// Create a new printer with predefined resolution.
    pub fn with_resolution(path: &str, tape: Tape, dpi: u32) -> Result<Self> {
        let file = std::fs::File::options().read(true).write(true).open(path)?;
        let mut printer = Self {
            file,
            resolution: dpi,
        };

        printer
            .size(tape.width, tape.height)?
            .gap(tape.gap, tape.gap_offset)?
            .cls()?;

        Ok(printer)
    }

    fn write(&mut self, cmd: impl ToString) -> Result<&mut Self> {
        let cmd = cmd.to_string() + "\r\n";
        self.file
            .write_all(cmd.as_bytes())
            .map_err(|e| anyhow!(e))?;
        Ok(self)
    }

    fn write_bytes(&mut self, mut cmd: Vec<u8>) -> Result<&mut Self> {
        let crlf = b"\r\n";
        cmd.extend(crlf);
        self.file.write_all(&cmd).map_err(|e| anyhow!(e))?;
        Ok(self)
    }

    /// This command defines the label width and height.
    /// Label length must be provided for firmware version <V8.13.
    fn size(&mut self, width: Size, height: Option<Size>) -> Result<&mut Self> {
        let cmd = height.map_or_else(
            || format!("SIZE {width}"),
            |height| format!("SIZE {width},{height}"),
        );
        self.write(cmd)
    }

    /// Defines the gap distance between two labels.
    /// Optional offset distance of the gap may be provided.
    fn gap(&mut self, gap: Size, gap_offset: Option<Size>) -> Result<&mut Self> {
        let cmd = gap_offset.map_or_else(
            || format!("GAP {gap}"),
            |offset| format!("GAP {gap},{offset}"),
        );
        self.write(cmd)
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
            Some((x, y)) => format!(
                "GAPDETECT {},{}",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => String::from("GAPDETECT"),
        };
        self.write(cmd)
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
            Some((x, y)) => format!(
                "BLINEDETECT {},{}",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => String::from("BLINEDETECT"),
        };
        self.write(cmd)
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
            Some((x, y)) => format!(
                "AUTODETECT {},{}",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => String::from("AUTODETECT"),
        };
        self.write(cmd)
    }

    /// This command sets the height of the black line and the user-defined extra label feeding length each form feed takes.
    /// Both parameters should be in the same measurement type (mm/inch/dot).
    pub fn bline(&mut self, black_line_height: Size, extra_feeding_len: Size) -> Result<&mut Self> {
        let cmd = format!("BLINE {black_line_height},{extra_feeding_len}");
        self.write(cmd)
    }

    /// This command defines the selective, extra label feeding length each form feed takes, which,
    /// especially in peel-off mode and cutter mode, is used to adjust label stop position,
    /// so as for label to register at proper places for the intended purposes.
    /// The printer back tracks the extra feeding length before the next run of printing.
    pub fn offset(&mut self, offset: Size) -> Result<&mut Self> {
        let cmd = format!("OFFSET {offset}");
        self.write(cmd)
    }

    /// This command defines the print speed.
    /// Available speeds in inch/sec should be checked for your printer model.
    pub fn speed(&mut self, speed: &str) -> Result<&mut Self> {
        let cmd = format!("SPEED {speed}");
        self.write(cmd)
    }

    /// This command sets the printing darkness from lightest(0) to darkest(15). Default density is 8.
    pub fn density(&mut self, density: u8) -> Result<&mut Self> {
        let cmd = match density {
            1..=15 => format!("DENSITY {density}"),
            _ => return Err(anyhow!("Density should be in range 0..15")),
        };
        self.write(cmd)
    }

    /// This command defines the printout direction and mirror image. This will be stored in the printer memory.
    pub fn direction(
        &mut self,
        reversed_direction: bool,
        mirrored_image: bool,
    ) -> Result<&mut Self> {
        let cmd = format!(
            "DIRECTION {},{}",
            reversed_direction as u8, mirrored_image as u8
        );
        self.write(cmd)
    }

    /// This command defines the reference point of the label. The reference (origin) point varies with the print direction.
    pub fn reference(&mut self, x: Size, y: Size) -> Result<&mut Self> {
        let cmd = format!(
            "REFERENCE {},{}",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution)
        );
        self.write(cmd)
    }

    /// This command moves the label’s horizontal and vertical position. A positive value moves the label
    /// further from the printing direction; a negative value moves the label towards the printing direction.
    pub fn shift(&mut self, x: Option<Size>, y: Size) -> Result<&mut Self> {
        let cmd = match x {
            Some(x) => format!(
                "SHIFT {},{}",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution)
            ),
            None => format!("SHIFT {}", y.to_dots_raw(self.resolution)),
        };
        self.write(cmd)
    }

    /// This command orients the keyboard for use in different countries via
    /// defining special characters on the KP-200 series portable LCD keyboard (option).
    pub fn country(&mut self, country: Country) -> Result<&mut Self> {
        let cmd = format!("COUNTRY {:03}", country as u16);
        self.write(cmd)
    }

    /// This command defines the code page of international character set.
    pub fn codepage(&mut self, codepage: Codepage) -> Result<&mut Self> {
        let cmd = format!("CODEPAGE {codepage}");
        self.write(cmd)
    }

    /// This command clears the image buffer.
    pub fn cls(&mut self) -> Result<&mut Self> {
        let cmd = "CLS";
        self.write(cmd)
    }

    /// This command feeds label with the specified length.
    pub fn feed(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
        let cmd = match feed_dot {
            0..=9999 => format!("FEED {feed_dot}"),
            _ => {
                return Err(anyhow!(
                    "feed length must be in range 0..9999 in dots, got {feed_dot}"
                ))
            }
        };
        self.write(cmd)
    }

    /// This command feeds the label in reverse.
    /// For TSPL printers only.
    pub fn backup(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
        let cmd = match feed_dot {
            0..=9999 => format!("BACKUP {feed_dot}"),
            _ => {
                return Err(anyhow!(
                    "backup length must be in range 0..9999, got {feed_dot}"
                ))
            }
        };
        self.write(cmd)
    }

    /// This command feeds the label in reverse. The length is specified by dot.
    /// For TSPL2 printers only.
    pub fn backfeed(&mut self, feed: Size) -> Result<&mut Self> {
        let feed_dot = feed.to_dots_raw(self.resolution);
        let cmd = match feed_dot {
            0..=9999 => format!("BACKFEED {feed_dot}"),
            _ => {
                return Err(anyhow!(
                    "backfeed length must be in range 0..9999, got {feed_dot}"
                ))
            }
        };
        self.write(cmd)
    }

    /// This command feeds label to the beginning of next label.
    pub fn formfeed(&mut self) -> Result<&mut Self> {
        let cmd = "FORMFEED";
        self.write(cmd)
    }

    /// This command will feed label until the internal sensor has determined the origin.
    /// Size and gap of the label should be defined before using this command.
    /// For TSPL programming printer: Back label to origin position.
    /// For TSPL2 programming printer: Feed label to origin position.
    pub fn home(&mut self) -> Result<&mut Self> {
        let cmd = "HOME";
        self.write(cmd)
    }

    /// This command prints the label format currently stored in the image buffer.
    pub fn print(&mut self, sets: u32, copies: Option<u32>) -> Result<&mut Self> {
        let cmd = match sets {
            1..=999999999 => {
                if let Some(copies) = copies {
                    match copies {
                        1..=999999999 => format!("PRINT {sets},{copies}"),
                        _ => {
                            return Err(anyhow!(
                                "Copies qty must be in range 1..999999999, got {copies}"
                            ))
                        }
                    }
                } else {
                    format!("PRINT {sets}")
                }
            }
            _ => {
                return Err(anyhow!(
                    "Sets qty must be in range 1..999999999, got {sets}"
                ))
            }
        };

        self.write(cmd)
    }

    /// This command controls the sound frequency of the beeper. There are 10 levels of sounds, from 0 to 9.
    /// The timing control can be set by the "interval" parameter, in range 1..4095.
    pub fn sound(&mut self, level: u8, interval: u16) -> Result<&mut Self> {
        let cmd = match (level, interval) {
            (0..=9, 1..=4095) => format!("SOUND {level},{interval}"),
            _ => return Err(anyhow!("wrong sound parameters")),
        };
        self.write(cmd)
    }

    /// This command activates the cutter to immediately cut the labels without back feeding the label.
    pub fn cut(&mut self) -> Result<&mut Self> {
        let cmd = "CUT";
        self.write(cmd)
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
            Some((x, y)) => format!("LIMITFEED {n},{x},{y}"),
            None => format!("LIMITFEED {n}"),
        };
        self.write(cmd)
    }

    /// At this command, the printer will print out the printer information.
    pub fn selftest(&mut self, test_kind: Selftest) -> Result<&mut Self> {
        let cmd = format!("SELFTEST {test_kind}");
        self.write(cmd)
    }

    /// Let the printer wait until process of commands (before EOJ) be finished then go on the next command.
    pub fn eoj(&mut self) -> Result<&mut Self> {
        let cmd = "EOJ";
        self.write(cmd)
    }

    /// Let the printer wait specific period of time then go on next command.
    pub fn delay(&mut self, delay: std::time::Duration) -> Result<&mut Self> {
        let cmd = format!("DELAY {}", delay.as_millis());
        self.write(cmd)
    }

    /// This command can show the image, which is in printer’s image buffer, on LCD panel.
    pub fn display(&self) {
        unimplemented!()
    }

    /// This command can restore printer settings to defaults.
    pub fn initial_printer(&mut self) -> Result<&mut Self> {
        let cmd = "INITIALPRINTER";
        self.write(cmd)
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
            "BAR {},{},{},{}",
            x_upper_left.to_dots_raw(self.resolution),
            y_upper_left.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution)
        );
        self.write(cmd)
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
                "BARCODE {},{},\"{}\",{},{},{},{},{}, \"{}\"",
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
                "BARCODE {},{},\"{}\",{},{},{},{}, \"{}\"",
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
        self.write(cmd)
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
            "TLC39 {},{},{},{},{},{},{},{}, \"{},{},{}\"",
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
        self.write(cmd)
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
        self.write_bytes(cmd)
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
            "BOX {},{},{},{},{},{}",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            x_end.to_dots_raw(self.resolution),
            y_end.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution),
            radius.unwrap_or(Size::Dots(0)).to_dots_raw(self.resolution)
        );
        self.write(cmd)
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
            "CIRCLE {},{},{},{}",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            diameter.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution)
        );
        self.write(cmd)
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
            "ELLIPSE {},{},{},{},{}",
            x_upper_left.to_dots_raw(self.resolution),
            y_upper_left.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution)
        );
        self.write(cmd)
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
            "CODABLOCK {},{},{},{},{},\"{}\"",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution),
            rotate,
            row_height,
            module_width,
            content
        );
        self.write(cmd)
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

        cmd.push_str(&format!(" \"{content}\""));

        self.write(cmd)
    }

    /// This command clears a specified region in the image buffer.
    pub fn erase(&mut self, x: Size, y: Size, width: Size, height: Size) -> Result<&mut Self> {
        let cmd = format!(
            "ERASE {},{},{},{}",
            x.to_dots_raw(self.resolution),
            y.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution)
        );
        self.write(cmd)
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
            "PDF417 {},{},{},{},{},\"{}\"",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution),
            rotate,
            content
        );
        self.write(cmd)
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
            "AZTEC {},{},{},{},{},{},{},{},{},{},{}",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            rotate,
            size,
            ecp,
            flg as u8,
            menu as u8,
            multi,
            reversed as u8,
            content.len(),
            content
        );
        self.write(cmd)
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
        let col_num = col_num.map_or(0, |x| match x {
            1..=4 => x,
            _ => 0,
        });

        let module_width = module_width
            .unwrap_or(Size::Dots(1))
            .to_dots_raw(self.resolution);
        let module_height = module_height
            .unwrap_or(Size::Dots(10))
            .to_dots_raw(self.resolution);

        let cmd = format!(
            "MPDF417 {},{},{},{},{},{}, \"{}\"",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            rotate,
            module_width,
            module_height,
            col_num,
            content,
        );
        self.write(cmd)
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

        let cmd = match justification {
            Some(justification) => format!(
                "QRCODE {},{},{},{},A,{},{},\"{}\"",
                x_upper_left.to_dots_raw(self.resolution),
                y_upper_left.to_dots_raw(self.resolution),
                ecc_level,
                cellwidth_dot,
                rotate,
                justification,
                content
            ),
            None => format!(
                "QRCODE {},{},{},{},A,{},\"{}\"",
                x_upper_left.to_dots_raw(self.resolution),
                y_upper_left.to_dots_raw(self.resolution),
                ecc_level,
                cellwidth_dot,
                rotate,
                content
            ),
        };
        self.write(cmd)
    }

    /// This command is used to draw a RSS bar code on the label format.
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
                        "RSS {},{}, \"{}\",{},{},{},{}, \"{}\"",
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
                        "RSS {},{}, \"{}\",{},{},{},{}, \"{}\"",
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
                    "RSS {},{}, \"{}\",{},{},{}, \"{}\"",
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
        self.write(cmd)
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
            "REVERSE {},{},{},{}",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            width.to_dots_raw(self.resolution),
            height.to_dots_raw(self.resolution)
        );
        self.write(cmd)
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
            "DIAGONAL {},{},{},{},{}",
            x_start.to_dots_raw(self.resolution),
            y_start.to_dots_raw(self.resolution),
            x_end.to_dots_raw(self.resolution),
            y_end.to_dots_raw(self.resolution),
            thickness.to_dots_raw(self.resolution)
        );
        self.write(cmd)
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
                "TEXT {},{},\"{}\",{},{},{},{}, \"{}\"",
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
                "TEXT {},{},\"{}\",{},{},{}, \"{}\"",
                x.to_dots_raw(self.resolution),
                y.to_dots_raw(self.resolution),
                font,
                rotate,
                multiply_x,
                multiply_y,
                content
            ),
        };
        self.write(cmd)
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

        cmd.push_str(&format!("\"{}\"", content));
        self.write(cmd)
    }
}
