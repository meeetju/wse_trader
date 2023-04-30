use crate::errors::Error;
use common::types::shared_types::StockRequirements;

pub trait Read {
    fn read(&self) -> Result<StockRequirements, Error>;
}

pub struct YamlReader {
    pub path: String,
}

impl Read for YamlReader {
    fn read(&self) -> Result<StockRequirements, Error> {
        let f = std::fs::File::open(&self.path)?;
        let requirements: StockRequirements = serde_yaml::from_reader(f)?;
        Ok(requirements)
    }
}

pub struct WebJsonReader {
    pub in_requirements: StockRequirements,
}

impl Read for WebJsonReader {
    fn read(&self) -> Result<StockRequirements, Error> {
        Ok(self.in_requirements.clone())
    }
}
