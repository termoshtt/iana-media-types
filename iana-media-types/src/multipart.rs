#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Multipart {
    #[doc = "multipart/appledouble"]
    Appledouble,
    #[doc = "multipart/byteranges"]
    Byteranges,
    #[doc = "multipart/encrypted"]
    Encrypted,
    #[doc = "multipart/example"]
    Example,
    #[doc = "multipart/form-data"]
    FormData,
    #[doc = "multipart/header-set"]
    HeaderSet,
    #[doc = "multipart/multilingual"]
    Multilingual,
    #[doc = "multipart/related"]
    Related,
    #[doc = "multipart/report"]
    Report,
    #[doc = "multipart/signed"]
    Signed,
    #[doc = "multipart/vnd.bint.med-plus"]
    VndBintMedPlus,
    #[doc = "multipart/voice-message"]
    VoiceMessage,
    #[doc = "multipart/x-mixed-replace"]
    XMixedReplace,
    Other(String),
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
            Multipart::Other(template) => write!(f, "{}", template)?,
        }
        Ok(())
    }
}
impl From<&str> for Multipart {
    fn from(input: &str) -> Self {
        match input {
            "multipart/appledouble" => Multipart::Appledouble,
            "multipart/byteranges" => Multipart::Byteranges,
            "multipart/encrypted" => Multipart::Encrypted,
            "multipart/example" => Multipart::Example,
            "multipart/form-data" => Multipart::FormData,
            "multipart/header-set" => Multipart::HeaderSet,
            "multipart/multilingual" => Multipart::Multilingual,
            "multipart/related" => Multipart::Related,
            "multipart/report" => Multipart::Report,
            "multipart/signed" => Multipart::Signed,
            "multipart/vnd.bint.med-plus" => Multipart::VndBintMedPlus,
            "multipart/voice-message" => Multipart::VoiceMessage,
            "multipart/x-mixed-replace" => Multipart::XMixedReplace,
            _ => Multipart::Other(input.to_string()),
        }
    }
}
