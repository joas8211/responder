use bincode::{Decode, Encode};
use std::collections::HashMap;

mod manifest_json;
mod manifest_yaml;

#[derive(Encode, Decode)]
pub struct Matcher {
    pub headers: HashMap<String, Vec<String>>,
    pub methods: Vec<String>,
    pub paths: Vec<String>,
    pub response: u32,
}

#[derive(Encode, Decode)]
pub struct Package {
    pub matchers: Vec<Matcher>,
    pub responses: Vec<Vec<u8>>,
}
