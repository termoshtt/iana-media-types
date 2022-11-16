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
            Message::FeedbackReport => write!(f, "message/feedback-report")?,
            Message::Global => write!(f, "message/global")?,
            Message::GlobalDeliveryStatus => write!(f, "message/global-delivery-status")?,
            Message::GlobalDispositionNotification => {
                write!(f, "message/global-disposition-notification")?
            }
            Message::GlobalHeaders => write!(f, "message/global-headers")?,
            Message::Http => write!(f, "message/http")?,
            Message::ImdnXml => write!(f, "message/imdn+xml")?,
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
            "message/feedback-report" => Ok(Message::FeedbackReport),
            "message/global" => Ok(Message::Global),
            "message/global-delivery-status" => Ok(Message::GlobalDeliveryStatus),
            "message/global-disposition-notification" => Ok(Message::GlobalDispositionNotification),
            "message/global-headers" => Ok(Message::GlobalHeaders),
            "message/http" => Ok(Message::Http),
            "message/imdn+xml" => Ok(Message::ImdnXml),
            "message/sip" => Ok(Message::Sip),
            "message/sipfrag" => Ok(Message::Sipfrag),
            "message/tracking-status" => Ok(Message::TrackingStatus),
            "message/vnd.wfa.wsc" => Ok(Message::VndWfaWsc),
            _ => Err(()),
        }
    }
}
