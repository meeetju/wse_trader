mod company;
mod requirements;
mod ranked_companies;
mod results_writer;

fn main() {

    let requirements = requirements::StockRequirements::new();
    let requirements = requirements.update("requirements.yaml".to_string());
    let results_writer = results_writer::CsvWriter::new("results.csv".to_string());
    // println!("{:?}", requirements);
    let mut ranked = ranked_companies::RankedCompanies::new(requirements, results_writer);
    ranked.get_companies();
    // ranked.update_indicators();
    // ranked.print_results();
    ranked.write_results();
}
