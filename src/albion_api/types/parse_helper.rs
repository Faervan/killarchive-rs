use serde::{de::Error, Deserializer};
use serde_json::{Map, Value};

pub trait QuickParse<'de> {
    fn get_key<D>(&mut self, key: &'static str) -> Result<String, D::Error>
            where
                Self: Sized,
                D: Deserializer<'de>;
    fn key(&mut self, key: &'static str) -> String;
}

impl<'de> QuickParse<'de> for Map<String, Value> {
    fn get_key<D>(&mut self, key: &'static str) -> Result<String, D::Error>
                where
                    Self: Sized,
                    D: Deserializer<'de> {
        self.remove(key)
            .map(|v| v.to_string().trim_matches('\"').to_string())
            .ok_or_else(|| Error::missing_field(key))
    }
    fn key(&mut self, key: &'static str) -> String {
        self.remove(key)
            .map(|v| v.to_string().trim_matches('\"').to_string())
            .unwrap_or(String::new())
    }
}
