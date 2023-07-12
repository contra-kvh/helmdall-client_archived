use crate::models::script_config::ScriptConfig;
use serde::{
    Serialize,
    Deserialize
};

#[derive(Deserialize, Serialize, Debug)]
pub struct ScriptGroup {
    name: String,
    scripts: Vec<ScriptConfig>
}