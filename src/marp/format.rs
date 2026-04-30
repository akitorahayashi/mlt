#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Pdf,
    Html,
    Pptx,
}

impl Format {
    pub const ALL: [Format; 3] = [Format::Pdf, Format::Html, Format::Pptx];

    pub fn extension(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Html => "html",
            Self::Pptx => "pptx",
        }
    }
}
