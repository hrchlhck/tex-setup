use std::collections::HashMap;

use reqwest::blocking::ClientBuilder;
use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct GistCommit {
    pub files: HashMap<String, File>,
    pub history: Vec<History>,
}


#[derive(Deserialize, Debug)]
pub struct File {
    pub content: String
 }

#[derive(Deserialize, Debug)]
pub struct History {
    pub version: String,
}

pub fn get_info(gist_id: &str, token: &str) -> GistCommit {
    let client = ClientBuilder::new()
        .user_agent("TeX-Setup/0.1.0")
        .build()
        .unwrap();
    let result = client
        .get(format!("https://api.github.com/gists/{gist_id}"))
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .unwrap();

    result.json().unwrap()
}
