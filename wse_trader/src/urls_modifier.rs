use std::collections::HashMap;
use serde_any;

#[derive(Debug)]
pub struct UrlsModifier {
    pub path: String,
    map: HashMap<String, String>
}

impl UrlsModifier {
    pub fn new(path: String) -> Self {
        Self {
            path,
            map: HashMap::new(),
        }
    }
    pub fn read_map_file(self) -> HashMap<String, String> {
        let map = serde_any::from_file(self.path).unwrap();
        map
    }

}