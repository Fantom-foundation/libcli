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
            "random" => Ok(PeerSelectorOptions::Random),
            "smart" => Ok(PeerSelectorOptions::Smart),
            "fair" => Ok(PeerSelectorOptions::Fair),
            "unfair" => Ok(PeerSelectorOptions::Unfair),
            "franky" => Ok(PeerSelectorOptions::Franky),
            _ => Err(()),
        }
    }
}
