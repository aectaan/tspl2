use strum_macros::Display;

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
