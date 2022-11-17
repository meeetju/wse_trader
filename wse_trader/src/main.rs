mod company;
mod requirements_reader;
mod ranked_companies;
mod results_writer;
mod urls_modifier;

use crate::ranked_companies::RankedCompanies;
use crate::requirements_reader::YamlReader;
use crate::results_writer::{CsvWriter, ConsolePrinter};
use crate::urls_modifier::UrlsModifier;

fn main() {

    let mut ranked = RankedCompanies::new();
    ranked.update_requirements(YamlReader{path: "requirements.yaml".to_string()});
    ranked.update_url_mappings(UrlsModifier::new("links_mapping.yaml".to_string()));
    ranked.get_companies();
    ranked.update_indicators();
    ranked.filter_best_companies();
    ranked.write_results(CsvWriter{path: "results.csv".to_string()});
    // ranked.write_results(ConsolePrinter{});
}
