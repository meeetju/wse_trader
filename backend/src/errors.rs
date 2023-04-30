use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Generic(String),
    #[error("I/O error {0}")]
    File(#[from] std::io::Error),
    #[error("Yaml error {0}")]
    YamlError(#[from] serde_yaml::Error),
    #[error("TickerError: Searched value not found")]
    TickerError(),
    #[error("NameError: Searched value not found")]
    NameError(),
    #[error("AltmanError: Searched value not found")]
    AltmanError(),
    #[error("FScoreError: Searched value not found")]
    FScoreError(),
    #[error("FloatError: Searched value not found")]
    FloatError(),
    #[error("WrongLinkError: Wrong link {0}, Identify correct link for the company, and put the diff between links into links_mapping.yaml")]
    WrongLinkError(String),
}
