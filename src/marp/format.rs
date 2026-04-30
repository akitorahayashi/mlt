#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Pdf,
    Html,
    Png,
    Pptx,
}

impl Format {
    pub const ALL: [Format; 4] = [Format::Pdf, Format::Html, Format::Png, Format::Pptx];

    pub fn extension(self) -> &'static str {
        match self {
            Self::Pdf => "pdf",
            Self::Html => "html",
            Self::Png => "png",
            Self::Pptx => "pptx",
        }
    }
}
