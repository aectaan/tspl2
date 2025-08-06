use strum_macros::Display;

#[derive(Debug, Display)]
pub enum RssType {
    /// RSS14.
    #[strum(serialize = "RSS14")]
    Rss14,
    /// RSS14 Truncated.
    #[strum(serialize = "RSS14T")]
    Rss14T,
    /// RSS14 Stacked.
    #[strum(serialize = "RSS14S")]
    Rss14S,
    /// RSS14 Stacked Omnidirectional.
    #[strum(serialize = "RSS14SO")]
    Rss14So,
    /// RSS Limited.
    #[strum(serialize = "RSSLIM")]
    RssLim,
    /// RSS Expanded.
    #[strum(serialize = "RSSEXP")]
    RssExp,
    /// UPC-A.
    #[strum(serialize = "UPCA")]
    UpcA,
    /// UPC-E.
    #[strum(serialize = "UPCE")]
    UpcE,
    /// EAN13.
    #[strum(serialize = "EAN13")]
    Ean13,
    /// EAN8.
    #[strum(serialize = "EAN8")]
    Ean8,
    /// UCC/EAN-128 & CC-A/B.
    #[strum(serialize = "UCC128CCA")]
    Ucc128Cca,
    /// UCC/EAN-128 & CC-C.
    #[strum(serialize = "UCC128CCC")]
    Ucc128Ccc,
}
