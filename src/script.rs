use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Script {
    keys: Vec<String>,
    args: Vec<String>,
    script: String,
}

impl Script {
    pub fn keys(&self) -> &Vec<String> {
        &self.keys
    }

    pub fn args(&self) -> &Vec<String> {
        &self.args
    }

    pub fn script(&self) -> &String {
        &self.script
    }
}
