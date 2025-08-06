use strum_macros::Display;

#[derive(Debug, Display)]
pub enum Font {
    /// Monotye CG Triumvirate Bold Condensed, font width and height is stretchable.
    #[strum(serialize = "0")]
    FontMonotye,
    /// 8 x 12 fixed pitch dot font.
    #[strum(serialize = "1")]
    Font8x12,
    /// 12 x 20 fixed pitch dot font.
    #[strum(serialize = "2")]
    Font12x20,
    /// 16 x 24 fixed pitch dot font.
    #[strum(serialize = "3")]
    Font16x24,
    /// 24 x 32 fixed pitch dot font.
    #[strum(serialize = "4")]
    Font24x32,
    /// 32 x 48 dot fixed pitch font.
    #[strum(serialize = "5")]
    Font32x48,
    /// 14 x 19 dot fixed pitch font OCR-B.
    #[strum(serialize = "6")]
    Font14x19,
    /// 21 x 27 dot fixed pitch font OCR-B.
    #[strum(serialize = "7")]
    Font21x27,
    /// 14 x25 dot fixed pitch font OCR-A.
    #[strum(serialize = "8")]
    Font14x25,
    /// Monotye CG Triumvirate Bold Condensed,
    /// font width and height proportion is fixed.
    #[strum(serialize = "ROMAN.TTF")]
    FontRoman,
    /// EPL2 font 1.
    #[strum(serialize = "1.EFT")]
    FontEpl1,
    /// EPL2 font 2.
    #[strum(serialize = "2.EFT")]
    FontEpl2,
    /// EPL2 font 3.
    #[strum(serialize = "3.RFT")]
    FontEpl3,
    /// EPL2 font 4.
    #[strum(serialize = "4.EFT")]
    FontEpl4,
    /// EPL2 font 5.
    #[strum(serialize = "5.EFT")]
    FontEpl5,
    /// ZPL2 font A.
    #[strum(serialize = "A.FNT")]
    FontZplA,
    /// ZPL2 font A.
    #[strum(serialize = "B.FNT")]
    FontZplB,
    /// ZPL2 font D.
    #[strum(serialize = "D.FNT")]
    FontZplD,
    /// ZPL2 font E8.
    #[strum(serialize = "E8.FNT")]
    FontZplE8,
    /// ZPL2 font F.
    #[strum(serialize = "F.FNT")]
    FontZplF,
    /// ZPL2 font G.
    #[strum(serialize = "G.FNT")]
    FontZplG,
    /// ZPL2 font H8.
    #[strum(serialize = "H8.FNT")]
    FontZplH8,
    /// ZPL2 font GS.
    #[strum(serialize = "GS.FNT")]
    FontZplGs,
}
