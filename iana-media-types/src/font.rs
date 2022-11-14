#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Font {
    Collection,
    Otf,
    Sfnt,
    Ttf,
    Woff,
    Woff2,
    Other(String),
}
impl ::std::fmt::Display for Font {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Font::Collection => write!(f, "font/collection")?,
            Font::Otf => write!(f, "font/otf")?,
            Font::Sfnt => write!(f, "font/sfnt")?,
            Font::Ttf => write!(f, "font/ttf")?,
            Font::Woff => write!(f, "font/woff")?,
            Font::Woff2 => write!(f, "font/woff2")?,
            Font::Other(template) => write!(f, "{}", template)?,
        }
        Ok(())
    }
}
impl From<&str> for Font {
    fn from(input: &str) -> Self {
        match input {
            "font/collection" => Font::Collection,
            "font/otf" => Font::Otf,
            "font/sfnt" => Font::Sfnt,
            "font/ttf" => Font::Ttf,
            "font/woff" => Font::Woff,
            "font/woff2" => Font::Woff2,
            _ => Font::Other(input.to_string()),
        }
    }
}
