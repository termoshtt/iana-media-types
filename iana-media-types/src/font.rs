#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    :: serde :: Serialize,
    :: serde :: Deserialize,
)]
pub enum Font {
    #[doc = "font/collection"]
    #[serde(alias = "font/collection")]
    Collection,
    #[doc = "font/otf"]
    #[serde(alias = "font/otf")]
    Otf,
    #[doc = "font/sfnt"]
    #[serde(alias = "font/sfnt")]
    Sfnt,
    #[doc = "font/ttf"]
    #[serde(alias = "font/ttf")]
    Ttf,
    #[doc = "font/woff"]
    #[serde(alias = "font/woff")]
    Woff,
    #[doc = "font/woff2"]
    #[serde(alias = "font/woff2")]
    Woff2,
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
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Font {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "font/collection" => Ok(Font::Collection),
            "font/otf" => Ok(Font::Otf),
            "font/sfnt" => Ok(Font::Sfnt),
            "font/ttf" => Ok(Font::Ttf),
            "font/woff" => Ok(Font::Woff),
            "font/woff2" => Ok(Font::Woff2),
            _ => Err(()),
        }
    }
}
