use lettre::message::{
    header::{ContentDisposition, ContentType},
    SinglePart,
};

pub enum Attachment {
    Html { file_name: String, data: String },
}

impl From<Attachment> for SinglePart {
    fn from(value: Attachment) -> Self {
        match value {
            Attachment::Html { file_name, data } => SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .header(ContentDisposition::attachment(&file_name))
                .body(data),
        }
    }
}

impl From<&Attachment> for SinglePart {
    fn from(value: &Attachment) -> Self {
        match value {
            Attachment::Html { file_name, data } => SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .header(ContentDisposition::attachment(&file_name))
                .body(data.clone()),
        }
    }
}
