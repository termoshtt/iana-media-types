#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Message {
    #[doc = "message/bhttp"]
    Bhttp,
    #[doc = "message/CPIM"]
    Cpim,
    #[doc = "message/delivery-status"]
    DeliveryStatus,
    #[doc = "message/disposition-notification"]
    DispositionNotification,
    #[doc = "message/example"]
    Example,
    #[doc = "message/feedback-report"]
    FeedbackReport,
    #[doc = "message/global"]
    Global,
    #[doc = "message/global-delivery-status"]
    GlobalDeliveryStatus,
    #[doc = "message/global-disposition-notification"]
    GlobalDispositionNotification,
    #[doc = "message/global-headers"]
    GlobalHeaders,
    #[doc = "message/http"]
    Http,
    #[doc = "message/imdn+xml"]
    ImdnXml,
    #[doc = "message/sip"]
    Sip,
    #[doc = "message/sipfrag"]
    Sipfrag,
    #[doc = "message/tracking-status"]
    TrackingStatus,
    #[doc = "message/vnd.wfa.wsc"]
    VndWfaWsc,
    Other(String),
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
            Message::Other(template) => write!(f, "{}", template)?,
        }
        Ok(())
    }
}
impl From<&str> for Message {
    fn from(input: &str) -> Self {
        match input {
            "message/bhttp" => Message::Bhttp,
            "message/CPIM" => Message::Cpim,
            "message/delivery-status" => Message::DeliveryStatus,
            "message/disposition-notification" => Message::DispositionNotification,
            "message/example" => Message::Example,
            "message/feedback-report" => Message::FeedbackReport,
            "message/global" => Message::Global,
            "message/global-delivery-status" => Message::GlobalDeliveryStatus,
            "message/global-disposition-notification" => Message::GlobalDispositionNotification,
            "message/global-headers" => Message::GlobalHeaders,
            "message/http" => Message::Http,
            "message/imdn+xml" => Message::ImdnXml,
            "message/sip" => Message::Sip,
            "message/sipfrag" => Message::Sipfrag,
            "message/tracking-status" => Message::TrackingStatus,
            "message/vnd.wfa.wsc" => Message::VndWfaWsc,
            _ => Message::Other(input.to_string()),
        }
    }
}
