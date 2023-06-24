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
pub enum Message {
    #[doc = "message/bhttp"]
    #[serde(alias = "message/bhttp")]
    Bhttp,
    #[doc = "message/CPIM"]
    #[serde(alias = "message/CPIM")]
    Cpim,
    #[doc = "message/delivery-status"]
    #[serde(alias = "message/delivery-status")]
    DeliveryStatus,
    #[doc = "message/disposition-notification"]
    #[serde(alias = "message/disposition-notification")]
    DispositionNotification,
    #[doc = "message/example"]
    #[serde(alias = "message/example")]
    Example,
    #[doc = "message/feedback-report"]
    #[serde(alias = "message/feedback-report")]
    ExternalBody,
    #[doc = "message/global"]
    #[serde(alias = "message/global")]
    #[serde(alias = "u8msg")]
    FeedbackReport,
    #[doc = "message/global-delivery-status"]
    #[serde(alias = "message/global-delivery-status")]
    #[serde(alias = "u8dsn")]
    Global,
    #[doc = "message/global-disposition-notification"]
    #[serde(alias = "message/global-disposition-notification")]
    #[serde(alias = "u8mdn")]
    GlobalDeliveryStatus,
    #[doc = "message/global-headers"]
    #[serde(alias = "message/global-headers")]
    #[serde(alias = "u8hdr")]
    GlobalDispositionNotification,
    #[doc = "message/http"]
    #[serde(alias = "message/http")]
    GlobalHeaders,
    #[doc = "message/imdn+xml"]
    #[serde(alias = "message/imdn+xml")]
    Http,
    #[doc = "message/mls"]
    #[serde(alias = "message/mls")]
    ImdnXml,
    #[doc = "message/ohttp-req"]
    #[serde(alias = "message/ohttp-req")]
    Mls,
    #[doc = "message/ohttp-res"]
    #[serde(alias = "message/ohttp-res")]
    OhttpReq,
    #[doc = "message/sip"]
    #[serde(alias = "message/sip")]
    OhttpRes,
    #[doc = "message/sipfrag"]
    #[serde(alias = "message/sipfrag")]
    Partial,
    #[doc = "message/tracking-status"]
    #[serde(alias = "message/tracking-status")]
    Rfc822,
    #[doc = "message/vnd.wfa.wsc"]
    #[serde(alias = "message/vnd.wfa.wsc")]
    Sip,
}
impl ::std::fmt::Display for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Message::Bhttp => write!(f, "message/bhttp")?,
            Message::Cpim => write!(f, "message/CPIM")?,
            Message::DeliveryStatus => write!(f, "message/delivery-status")?,
            Message::DispositionNotification => write!(f, "message/disposition-notification")?,
            Message::Example => write!(f, "message/example")?,
            Message::ExternalBody => write!(f, "message/feedback-report")?,
            Message::FeedbackReport => write!(f, "message/global")?,
            Message::Global => write!(f, "message/global-delivery-status")?,
            Message::GlobalDeliveryStatus => write!(f, "message/global-disposition-notification")?,
            Message::GlobalDispositionNotification => write!(f, "message/global-headers")?,
            Message::GlobalHeaders => write!(f, "message/http")?,
            Message::Http => write!(f, "message/imdn+xml")?,
            Message::ImdnXml => write!(f, "message/mls")?,
            Message::Mls => write!(f, "message/ohttp-req")?,
            Message::OhttpReq => write!(f, "message/ohttp-res")?,
            Message::OhttpRes => write!(f, "message/sip")?,
            Message::Partial => write!(f, "message/sipfrag")?,
            Message::Rfc822 => write!(f, "message/tracking-status")?,
            Message::Sip => write!(f, "message/vnd.wfa.wsc")?,
        }
        Ok(())
    }
}
impl ::std::str::FromStr for Message {
    type Err = ();
    fn from_str(input: &str) -> ::std::result::Result<Self, Self::Err> {
        match input {
            "message/bhttp" => Ok(Message::Bhttp),
            "message/CPIM" => Ok(Message::Cpim),
            "message/delivery-status" => Ok(Message::DeliveryStatus),
            "message/disposition-notification" => Ok(Message::DispositionNotification),
            "message/example" => Ok(Message::Example),
            "message/feedback-report" => Ok(Message::ExternalBody),
            "message/global" => Ok(Message::FeedbackReport),
            "message/global-delivery-status" => Ok(Message::Global),
            "message/global-disposition-notification" => Ok(Message::GlobalDeliveryStatus),
            "message/global-headers" => Ok(Message::GlobalDispositionNotification),
            "message/http" => Ok(Message::GlobalHeaders),
            "message/imdn+xml" => Ok(Message::Http),
            "message/mls" => Ok(Message::ImdnXml),
            "message/ohttp-req" => Ok(Message::Mls),
            "message/ohttp-res" => Ok(Message::OhttpReq),
            "message/sip" => Ok(Message::OhttpRes),
            "message/sipfrag" => Ok(Message::Partial),
            "message/tracking-status" => Ok(Message::Rfc822),
            "message/vnd.wfa.wsc" => Ok(Message::Sip),
            _ => Err(()),
        }
    }
}
