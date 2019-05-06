
enum LogOptions {
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
    Paniic
}

impl FromStr for LogOptions {
    type Err = ();

    fn from_str(s: &str) -> Result<Options, ()> {
        match s {
            "debug" => Ok(LogOptions::Debug),
            "info" => Ok(LogOptions::Info),
            "warn" => Ok(LogOptions::Warn),
            "error" => Ok(LogOptions::Error),
            "fatal" => Ok(LogOptions::Fatal),
            "panic" => Ok(LogOptions::Paniic),
            _ => Err(()),
        }
    }
}