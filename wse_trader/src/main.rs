mod company;
mod requirements;
mod ranked_companies;

fn main() {
    let requirements = requirements::read_requirements("stock_requirements.toml".to_string()).unwrap();
    let mut ranked = ranked_companies::RankedCompanies::new();
    ranked.update_requirements(requirements);
    ranked.get_companies("https://www.biznesradar.pl/spolki-rating/akcje_gpw");
    ranked.update_indicators();
    ranked.print_results()
}
