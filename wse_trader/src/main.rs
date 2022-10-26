mod company;
mod file_reader;
mod ranked_companies;

fn main() {
    let requirements = file_reader::read_requirements("stock_requirements.toml".to_string()).unwrap();
    let mut ranked = ranked_companies::RankedCompanies::new();
    ranked.get_companies("https://www.biznesradar.pl/spolki-rating/akcje_gpw");
    ranked.update_indicators();
}
