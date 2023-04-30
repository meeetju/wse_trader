use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Own address
    #[clap(long, value_parser)]
    pub oa: Option<String>,
    /// Own port
    #[clap(long, value_parser)]
    pub op: Option<String>,
    /// Remote address
    #[clap(long, value_parser)]
    pub ra: Option<String>,
    /// Repote port
    #[clap(long, value_parser)]
    pub rp: Option<String>,
}
