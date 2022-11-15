mod application;
mod audio;
mod font;
mod image;
mod message;
mod model;
mod multipart;
mod text;
mod video;

pub use application::*;
pub use audio::*;
pub use font::*;
pub use image::*;
pub use message::*;
pub use model::*;
pub use multipart::*;
pub use text::*;
pub use video::*;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum MediaType {
    /// `application/*` media type
    Application(Application),
    /// `audio/*` media type
    Audio(Audio),
    /// `font/*` media type
    Font(Font),
    /// `image/*` media type
    Image(Image),
    /// `message/*` media type
    Message(Message),
    /// `model/*` media type
    Model(Model),
    /// `multipart/*` media type
    Multipart(Multipart),
    /// `text/*` media type
    Text(Text),
    /// `video/*` media type
    Video(Video),
    /// media types not registered in IANA
    Other(String),
}

impl ::std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Application(e) => write!(f, "{}", e)?,
            Self::Audio(e) => write!(f, "{}", e)?,
            Self::Font(e) => write!(f, "{}", e)?,
            Self::Image(e) => write!(f, "{}", e)?,
            Self::Message(e) => write!(f, "{}", e)?,
            Self::Model(e) => write!(f, "{}", e)?,
            Self::Multipart(e) => write!(f, "{}", e)?,
            Self::Text(e) => write!(f, "{}", e)?,
            Self::Video(e) => write!(f, "{}", e)?,
            Self::Other(other) => write!(f, "{}", other)?,
        }
        Ok(())
    }
}

impl From<&str> for MediaType {
    fn from(input: &str) -> Self {
        use std::str::FromStr;

        if let Ok(e) = Application::from_str(input) {
            return Self::Application(e);
        }
        if let Ok(e) = Audio::from_str(input) {
            return Self::Audio(e);
        }
        if let Ok(e) = Font::from_str(input) {
            return Self::Font(e);
        }
        if let Ok(e) = Image::from_str(input) {
            return Self::Image(e);
        }
        if let Ok(e) = Message::from_str(input) {
            return Self::Message(e);
        }
        if let Ok(e) = Model::from_str(input) {
            return Self::Model(e);
        }
        if let Ok(e) = Multipart::from_str(input) {
            return Self::Multipart(e);
        }
        if let Ok(e) = Text::from_str(input) {
            return Self::Text(e);
        }
        if let Ok(e) = Video::from_str(input) {
            return Self::Video(e);
        }
        Self::Other(input.to_string())
    }
}
