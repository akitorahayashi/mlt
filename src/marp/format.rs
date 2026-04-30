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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Single(Format),
    All,
}

impl Target {
    pub fn formats(self) -> Vec<Format> {
        match self {
            Self::Single(format) => vec![format],
            Self::All => Format::ALL.to_vec(),
        }
    }
}
