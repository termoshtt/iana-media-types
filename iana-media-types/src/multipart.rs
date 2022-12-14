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
    Appledouble,
    #[doc = "multipart/byteranges"]
    #[serde(rename = "multipart/byteranges")]
    Byteranges,
    #[doc = "multipart/encrypted"]
    #[serde(rename = "multipart/encrypted")]
    Encrypted,
    #[doc = "multipart/example"]
    #[serde(rename = "multipart/example")]
    Example,
    #[doc = "multipart/form-data"]
    #[serde(rename = "multipart/form-data")]
    FormData,
    #[doc = "multipart/header-set"]
    #[serde(rename = "multipart/header-set")]
    HeaderSet,
    #[doc = "multipart/multilingual"]
    #[serde(rename = "multipart/multilingual")]
    Multilingual,
    #[doc = "multipart/related"]
    #[serde(rename = "multipart/related")]
    Related,
    #[doc = "multipart/report"]
    #[serde(rename = "multipart/report")]
    Report,
    #[doc = "multipart/signed"]
    #[serde(rename = "multipart/signed")]
    Signed,
    #[doc = "multipart/vnd.bint.med-plus"]
    #[serde(rename = "multipart/vnd.bint.med-plus")]
    VndBintMedPlus,
    #[doc = "multipart/voice-message"]
    #[serde(rename = "multipart/voice-message")]
    VoiceMessage,
    #[doc = "multipart/x-mixed-replace"]
    #[serde(rename = "multipart/x-mixed-replace")]
    XMixedReplace,
}
impl ::std::fmt::Display for Multipart {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Multipart::Appledouble => write!(f, "multipart/appledouble")?,
            Multipart::Byteranges => write!(f, "multipart/byteranges")?,
            Multipart::Encrypted => write!(f, "multipart/encrypted")?,
            Multipart::Example => write!(f, "multipart/example")?,
            Multipart::FormData => write!(f, "multipart/form-data")?,
            Multipart::HeaderSet => write!(f, "multipart/header-set")?,
            Multipart::Multilingual => write!(f, "multipart/multilingual")?,
            Multipart::Related => write!(f, "multipart/related")?,
            Multipart::Report => write!(f, "multipart/report")?,
            Multipart::Signed => write!(f, "multipart/signed")?,
            Multipart::VndBintMedPlus => write!(f, "multipart/vnd.bint.med-plus")?,
            Multipart::VoiceMessage => write!(f, "multipart/voice-message")?,
            Multipart::XMixedReplace => write!(f, "multipart/x-mixed-replace")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Multipart {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "multipart/appledouble" => Ok(Multipart::Appledouble),
            "multipart/byteranges" => Ok(Multipart::Byteranges),
            "multipart/encrypted" => Ok(Multipart::Encrypted),
            "multipart/example" => Ok(Multipart::Example),
            "multipart/form-data" => Ok(Multipart::FormData),
            "multipart/header-set" => Ok(Multipart::HeaderSet),
            "multipart/multilingual" => Ok(Multipart::Multilingual),
            "multipart/related" => Ok(Multipart::Related),
            "multipart/report" => Ok(Multipart::Report),
            "multipart/signed" => Ok(Multipart::Signed),
            "multipart/vnd.bint.med-plus" => Ok(Multipart::VndBintMedPlus),
            "multipart/voice-message" => Ok(Multipart::VoiceMessage),
            "multipart/x-mixed-replace" => Ok(Multipart::XMixedReplace),
            _ => Err(()),
        }
    }
}
