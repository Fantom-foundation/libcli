enum PeerSelectorOptions {
    Random,
    Smart,
    Fair,
    Unfair,
    Franky,
}

impl FromStr for PeerSelectorOptions {
    type Err = ();

    fn from_str(s: &str) -> Result<PeerSelectorOptions, ()> {
        match s {
            "random" => Ok(PeerSelectorOptions::Debug),
            "smart" => Ok(PeerSelectorOptions::Info),
            "fair" => Ok(PeerSelectorOptions::Warn),
            "unfair" => Ok(PeerSelectorOptions::Error),
            "franky" => Ok(PeerSelectorOptions::Fatal),
            _ => Err(()),
        }
    }
}
