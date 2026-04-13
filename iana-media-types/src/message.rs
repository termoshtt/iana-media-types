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
    #[serde(rename = "message/bhttp")]
    Bhttp,
    #[doc = "message/CPIM"]
    #[serde(rename = "message/CPIM")]
    Cpim,
    #[doc = "message/delivery-status"]
    #[serde(rename = "message/delivery-status")]
    DeliveryStatus,
    #[doc = "message/disposition-notification"]
    #[serde(rename = "message/disposition-notification")]
    DispositionNotification,
    #[doc = "message/example"]
    #[serde(rename = "message/example")]
    Example,
    #[doc = "message/external-body"]
    #[serde(rename = "message/external-body")]
    ExternalBody,
    #[doc = "message/feedback-report"]
    #[serde(rename = "message/feedback-report")]
    FeedbackReport,
    #[doc = "message/global"]
    #[serde(rename = "message/global")]
    Global,
    #[doc = "message/global-delivery-status"]
    #[serde(rename = "message/global-delivery-status")]
    GlobalDeliveryStatus,
    #[doc = "message/global-disposition-notification"]
    #[serde(rename = "message/global-disposition-notification")]
    GlobalDispositionNotification,
    #[doc = "message/global-headers"]
    #[serde(rename = "message/global-headers")]
    GlobalHeaders,
    #[doc = "message/http"]
    #[serde(rename = "message/http")]
    Http,
    #[doc = "message/imdn+xml"]
    #[serde(rename = "message/imdn+xml")]
    ImdnXml,
    #[doc = "message/mls"]
    #[serde(rename = "message/mls")]
    Mls,
    #[doc = "message/ohttp-chunked-req"]
    #[serde(rename = "message/ohttp-chunked-req")]
    OhttpChunkedReq,
    #[doc = "message/ohttp-chunked-res"]
    #[serde(rename = "message/ohttp-chunked-res")]
    OhttpChunkedRes,
    #[doc = "message/ohttp-req"]
    #[serde(rename = "message/ohttp-req")]
    OhttpReq,
    #[doc = "message/ohttp-res"]
    #[serde(rename = "message/ohttp-res")]
    OhttpRes,
    #[doc = "message/partial"]
    #[serde(rename = "message/partial")]
    Partial,
    #[doc = "message/rfc822"]
    #[serde(rename = "message/rfc822")]
    Rfc822,
    #[doc = "message/sip"]
    #[serde(rename = "message/sip")]
    Sip,
    #[doc = "message/sipfrag"]
    #[serde(rename = "message/sipfrag")]
    Sipfrag,
    #[doc = "message/tracking-status"]
    #[serde(rename = "message/tracking-status")]
    TrackingStatus,
    #[doc = "message/vnd.wfa.wsc"]
    #[serde(rename = "message/vnd.wfa.wsc")]
    VndWfaWsc,
}
impl ::std::fmt::Display for Message {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Message::Bhttp => write!(f, "message/bhttp")?,
            Message::Cpim => write!(f, "message/CPIM")?,
            Message::DeliveryStatus => write!(f, "message/delivery-status")?,
            Message::DispositionNotification => write!(f, "message/disposition-notification")?,
            Message::Example => write!(f, "message/example")?,
            Message::ExternalBody => write!(f, "message/external-body")?,
            Message::FeedbackReport => write!(f, "message/feedback-report")?,
            Message::Global => write!(f, "message/global")?,
            Message::GlobalDeliveryStatus => write!(f, "message/global-delivery-status")?,
            Message::GlobalDispositionNotification => {
                write!(f, "message/global-disposition-notification")?
            }
            Message::GlobalHeaders => write!(f, "message/global-headers")?,
            Message::Http => write!(f, "message/http")?,
            Message::ImdnXml => write!(f, "message/imdn+xml")?,
            Message::Mls => write!(f, "message/mls")?,
            Message::OhttpChunkedReq => write!(f, "message/ohttp-chunked-req")?,
            Message::OhttpChunkedRes => write!(f, "message/ohttp-chunked-res")?,
            Message::OhttpReq => write!(f, "message/ohttp-req")?,
            Message::OhttpRes => write!(f, "message/ohttp-res")?,
            Message::Partial => write!(f, "message/partial")?,
            Message::Rfc822 => write!(f, "message/rfc822")?,
            Message::Sip => write!(f, "message/sip")?,
            Message::Sipfrag => write!(f, "message/sipfrag")?,
            Message::TrackingStatus => write!(f, "message/tracking-status")?,
            Message::VndWfaWsc => write!(f, "message/vnd.wfa.wsc")?,
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
            "message/external-body" => Ok(Message::ExternalBody),
            "message/feedback-report" => Ok(Message::FeedbackReport),
            "message/global" => Ok(Message::Global),
            "message/global-delivery-status" => Ok(Message::GlobalDeliveryStatus),
            "message/global-disposition-notification" => Ok(Message::GlobalDispositionNotification),
            "message/global-headers" => Ok(Message::GlobalHeaders),
            "message/http" => Ok(Message::Http),
            "message/imdn+xml" => Ok(Message::ImdnXml),
            "message/mls" => Ok(Message::Mls),
            "message/ohttp-chunked-req" => Ok(Message::OhttpChunkedReq),
            "message/ohttp-chunked-res" => Ok(Message::OhttpChunkedRes),
            "message/ohttp-req" => Ok(Message::OhttpReq),
            "message/ohttp-res" => Ok(Message::OhttpRes),
            "message/partial" => Ok(Message::Partial),
            "message/rfc822" => Ok(Message::Rfc822),
            "message/sip" => Ok(Message::Sip),
            "message/sipfrag" => Ok(Message::Sipfrag),
            "message/tracking-status" => Ok(Message::TrackingStatus),
            "message/vnd.wfa.wsc" => Ok(Message::VndWfaWsc),
            _ => Err(()),
        }
    }
}
