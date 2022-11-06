mod company;
mod requirements_reader;
mod ranked_companies;
mod results_writer;

fn main() {

    let mut ranked = ranked_companies::RankedCompanies::new();
    ranked.update_requirements(Box::new(requirements_reader::YamlReader{path: "requirements.yaml".to_string()}));
    ranked.get_companies();
    // ranked.update_indicators();
    // ranked.write_results(Box::new(results_writer::CsvWriter{path: "results.csv".to_string()}));
    ranked.write_results(Box::new(results_writer::ConsolePrinter{}))
}
