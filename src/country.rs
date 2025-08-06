use strum_macros::Display;

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
