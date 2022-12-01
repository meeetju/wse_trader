mod company;
mod errors;
mod lazy_regexps;
mod ranked_companies;
mod requirements_reader;
mod results_writer;
mod urls_modifier;

use std::env;

use crate::ranked_companies::RankedCompanies;
use crate::requirements_reader::YamlReader;
use crate::results_writer::{ConsolePrinter, CsvWriter, JsonWriter};
use crate::urls_modifier::UrlsModifier;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    let mut ranked = RankedCompanies::new();
    ranked.update_requirements(YamlReader {path: "requirements.yaml".to_string()});
    ranked.update_url_mappings(UrlsModifier::new("links_mapping.yaml".to_string()));
    ranked.get_companies().await;
    ranked.update_indicators().await;
    ranked.filter_best_companies().await;
    // ranked.write_results(CsvWriter{path: "results.csv".to_string()}).await;
    // ranked.write_results(ConsolePrinter{}).await;
    println!("{}", ranked.write_results(JsonWriter {}).await);
}
