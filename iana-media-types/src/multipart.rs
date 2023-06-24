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
    #[serde(rename = "multipart/appledouble")]
    Alternative,
    #[doc = "multipart/byteranges"]
    #[serde(rename = "multipart/byteranges")]
    Appledouble,
    #[doc = "multipart/encrypted"]
    #[serde(rename = "multipart/encrypted")]
    Byteranges,
    #[doc = "multipart/example"]
    #[serde(rename = "multipart/example")]
    Digest,
    #[doc = "multipart/form-data"]
    #[serde(rename = "multipart/form-data")]
    Encrypted,
    #[doc = "multipart/header-set"]
    #[serde(rename = "multipart/header-set")]
    Example,
    #[doc = "multipart/multilingual"]
    #[serde(rename = "multipart/multilingual")]
    FormData,
    #[doc = "multipart/related"]
    #[serde(rename = "multipart/related")]
    HeaderSet,
    #[doc = "multipart/report"]
    #[serde(rename = "multipart/report")]
    Mixed,
    #[doc = "multipart/signed"]
    #[serde(rename = "multipart/signed")]
    Multilingual,
    #[doc = "multipart/vnd.bint.med-plus"]
    #[serde(rename = "multipart/vnd.bint.med-plus")]
    #[serde(alias = "bmed")]
    Parallel,
    #[doc = "multipart/voice-message"]
    #[serde(rename = "multipart/voice-message")]
    #[serde(alias = "vpm")]
    Related,
    #[doc = "multipart/x-mixed-replace"]
    #[serde(rename = "multipart/x-mixed-replace")]
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
            "multipart/vnd.bint.med-plus" | "bmed" => Ok(Multipart::Parallel),
            "multipart/voice-message" | "vpm" => Ok(Multipart::Related),
            "multipart/x-mixed-replace" => Ok(Multipart::Report),
            _ => Err(()),
        }
    }
}
