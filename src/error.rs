use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, IncludeError>;

#[derive(Error, Debug)]
pub enum IncludeError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("UTF-8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("XML writer error: {0}")]
    XmlWriteError(#[from] xml::writer::Error),
    #[error("XML reader error: {0}")]
    XmlReadError(#[from] xml::reader::Error),
    #[error("XML reference error: {0}")]
    XmlReferenceError(&'static str),
}
