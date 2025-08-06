use strum_macros::Display;

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

/// Clockwise rotation.
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
