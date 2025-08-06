use strum_macros::Display;

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
