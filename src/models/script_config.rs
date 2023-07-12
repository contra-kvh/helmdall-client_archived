use std::collections::{HashMap};
use serde::{
    Serialize,
    Deserialize
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ScriptConfig{
    name: String,
    path: String,
    args: Vec<String>,
    env: HashMap<String, String>,
    timeout: Option<String>
}