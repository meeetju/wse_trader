use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotFoundError {
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
