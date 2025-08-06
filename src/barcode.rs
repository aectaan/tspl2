use strum_macros::Display;

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
    /// Code 39, switching standard and full ASCII mode automatically.
    #[strum(serialize = "39")]
    Barcode39,
    /// Code 39 with check digit.
    #[strum(serialize = "39C")]
    Barcode39C,
    /// Code 93.
    #[strum(serialize = "93")]
    Barcode93,
    /// EAN 13.
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
    /// UPC-A.
    #[strum(serialize = "UPCA")]
    BarcodeUpca,
    /// UPC-A with 2 digits add-on.
    #[strum(serialize = "UPCA+2")]
    BarcodeUpcaPlus2,
    /// UPC-A with 5 digits add-on.
    #[strum(serialize = "UPCA+5")]
    BarcodeUpaPlus5,
    /// UPC-E.
    #[strum(serialize = "UPCE")]
    BarcodeUpce,
    /// UPC-E with 2 digits add-on.
    #[strum(serialize = "UPCE+2")]
    BarcodeUpcePlus2,
    /// UPC-E with 5 digits add-on.
    #[strum(serialize = "UPCE+5")]
    BarcodeUpePlus5,
    /// MSI.
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
