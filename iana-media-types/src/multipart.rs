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
pub enum Multipart {
    #[doc = "multipart/appledouble"]
    #[serde(alias = "multipart/appledouble")]
    Alternative,
    #[doc = "multipart/byteranges"]
    #[serde(alias = "multipart/byteranges")]
    Appledouble,
    #[doc = "multipart/encrypted"]
    #[serde(alias = "multipart/encrypted")]
    Byteranges,
    #[doc = "multipart/example"]
    #[serde(alias = "multipart/example")]
    Digest,
    #[doc = "multipart/form-data"]
    #[serde(alias = "multipart/form-data")]
    Encrypted,
    #[doc = "multipart/header-set"]
    #[serde(alias = "multipart/header-set")]
    Example,
    #[doc = "multipart/multilingual"]
    #[serde(alias = "multipart/multilingual")]
    FormData,
    #[doc = "multipart/related"]
    #[serde(alias = "multipart/related")]
    HeaderSet,
    #[doc = "multipart/report"]
    #[serde(alias = "multipart/report")]
    Mixed,
    #[doc = "multipart/signed"]
    #[serde(alias = "multipart/signed")]
    Multilingual,
    #[doc = "multipart/vnd.bint.med-plus"]
    #[serde(alias = "multipart/vnd.bint.med-plus")]
    #[serde(alias = "bmed")]
    Parallel,
    #[doc = "multipart/voice-message"]
    #[serde(alias = "multipart/voice-message")]
    #[serde(alias = "vpm")]
    Related,
    #[doc = "multipart/x-mixed-replace"]
    #[serde(alias = "multipart/x-mixed-replace")]
    Report,
}
impl ::std::fmt::Display for Multipart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Multipart::Alternative => write!(f, "multipart/appledouble")?,
            Multipart::Appledouble => write!(f, "multipart/byteranges")?,
            Multipart::Byteranges => write!(f, "multipart/encrypted")?,
            Multipart::Digest => write!(f, "multipart/example")?,
            Multipart::Encrypted => write!(f, "multipart/form-data")?,
            Multipart::Example => write!(f, "multipart/header-set")?,
            Multipart::FormData => write!(f, "multipart/multilingual")?,
            Multipart::HeaderSet => write!(f, "multipart/related")?,
            Multipart::Mixed => write!(f, "multipart/report")?,
            Multipart::Multilingual => write!(f, "multipart/signed")?,
            Multipart::Parallel => write!(f, "multipart/vnd.bint.med-plus")?,
            Multipart::Related => write!(f, "multipart/voice-message")?,
            Multipart::Report => write!(f, "multipart/x-mixed-replace")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Multipart {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "multipart/appledouble" => Ok(Multipart::Alternative),
            "multipart/byteranges" => Ok(Multipart::Appledouble),
            "multipart/encrypted" => Ok(Multipart::Byteranges),
            "multipart/example" => Ok(Multipart::Digest),
            "multipart/form-data" => Ok(Multipart::Encrypted),
            "multipart/header-set" => Ok(Multipart::Example),
            "multipart/multilingual" => Ok(Multipart::FormData),
            "multipart/related" => Ok(Multipart::HeaderSet),
            "multipart/report" => Ok(Multipart::Mixed),
            "multipart/signed" => Ok(Multipart::Multilingual),
            "multipart/vnd.bint.med-plus" => Ok(Multipart::Parallel),
            "multipart/voice-message" => Ok(Multipart::Related),
            "multipart/x-mixed-replace" => Ok(Multipart::Report),
            _ => Err(()),
        }
    }
}
