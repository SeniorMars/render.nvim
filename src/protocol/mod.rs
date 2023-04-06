// defines the protocol enum
// pub enum Protocol {
//     KITTY,
//     // SIXEL,
//     // ITERM2,
// }

mod kitty;
// mod iterm2;
// mod sixel;

pub enum Protocol {
    Kitty,
    Unknown,
}

impl Protocol {
    pub fn from_str(s: &str) -> Self {
        match s {
            "kitty" => Self::Kitty,
            _ => Self::Unknown,
        }
    }
}
