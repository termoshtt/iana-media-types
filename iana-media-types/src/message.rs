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
    FeedbackReport,
    #[doc = "message/global"]
    #[serde(alias = "message/global")]
    Global,
    #[doc = "message/global-delivery-status"]
    #[serde(alias = "message/global-delivery-status")]
    GlobalDeliveryStatus,
    #[doc = "message/global-disposition-notification"]
    #[serde(alias = "message/global-disposition-notification")]
    GlobalDispositionNotification,
    #[doc = "message/global-headers"]
    #[serde(alias = "message/global-headers")]
    GlobalHeaders,
    #[doc = "message/http"]
    #[serde(alias = "message/http")]
    Http,
    #[doc = "message/imdn+xml"]
    #[serde(alias = "message/imdn+xml")]
    ImdnXml,
    #[doc = "message/mls"]
    #[serde(alias = "message/mls")]
    Mls,
    #[doc = "message/ohttp-req"]
    #[serde(alias = "message/ohttp-req")]
    OhttpReq,
    #[doc = "message/ohttp-res"]
    #[serde(alias = "message/ohttp-res")]
    OhttpRes,
    #[doc = "message/sip"]
    #[serde(alias = "message/sip")]
    Sip,
    #[doc = "message/sipfrag"]
    #[serde(alias = "message/sipfrag")]
    Sipfrag,
    #[doc = "message/tracking-status"]
    #[serde(alias = "message/tracking-status")]
    TrackingStatus,
    #[doc = "message/vnd.wfa.wsc"]
    #[serde(alias = "message/vnd.wfa.wsc")]
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
            Message::Mls => write!(f, "message/mls")?,
            Message::OhttpReq => write!(f, "message/ohttp-req")?,
            Message::OhttpRes => write!(f, "message/ohttp-res")?,
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
            "message/mls" => Ok(Message::Mls),
            "message/ohttp-req" => Ok(Message::OhttpReq),
            "message/ohttp-res" => Ok(Message::OhttpRes),
            "message/sip" => Ok(Message::Sip),
            "message/sipfrag" => Ok(Message::Sipfrag),
            "message/tracking-status" => Ok(Message::TrackingStatus),
            "message/vnd.wfa.wsc" => Ok(Message::VndWfaWsc),
            _ => Err(()),
        }
    }
}
