mod company;
mod requirements;
mod ranked_companies;
mod results_writer;

fn main() {
    let requirements = requirements::read_requirements("stock_requirements.toml".to_string()).unwrap();
    let mut ranked = ranked_companies::RankedCompanies::new();
    ranked.update_requirements(requirements);
    ranked.get_companies();
    ranked.update_indicators();
    // ranked.print_results()
    ranked.store_results_to_csv();
}
