use clap::Parser;

const COMPANIES_LIST_URL: &str = "https://www.biznesradar.pl/spolki-rating/akcje_gpw";
const COMPANY_INDICATORS_URL: &str = "https://strefainwestorow.pl/notowania/gpw/";

#[derive(Debug, Parser)]
pub struct Cli {
    /// Own address
    #[clap(long, value_parser)]
    pub oa: Option<String>,
    /// Own port
    #[clap(long, value_parser)]
    pub op: Option<String>,
    /// Companies list url
    #[clap(long, value_parser, default_value = COMPANIES_LIST_URL)]
    pub companies_list_url: String,
    /// Company indicators url
    #[clap(long, value_parser, default_value = COMPANY_INDICATORS_URL)]
    pub company_indicators_url: String,
}
