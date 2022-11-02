mod company;
mod requirements;
mod ranked_companies;
mod results_writer;

fn main() {

    let requirements = requirements::StockRequirements::new();
    let requirements = requirements.update("requirements.yaml".to_string());
    // println!("{:?}", requirements);
    let mut ranked = ranked_companies::RankedCompanies::new(requirements);
    ranked.get_companies();
    // ranked.update_indicators();
    ranked.print_results();
    // ranked.store_results_to_csv();
}
