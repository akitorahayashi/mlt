use clap::ValueEnum;

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

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Target {
    Pdf,
    Html,
    Png,
    Pptx,
    All,
}

impl Target {
    pub fn formats(self) -> Vec<Format> {
        match self {
            Self::Pdf => vec![Format::Pdf],
            Self::Html => vec![Format::Html],
            Self::Png => vec![Format::Png],
            Self::Pptx => vec![Format::Pptx],
            Self::All => Format::ALL.to_vec(),
        }
    }
}
