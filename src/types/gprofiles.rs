use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GProfilesData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keybinds: Option<HashMap<String, Vec<Keybind>>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keybind {
    pub input: String,
    pub output: String,
    pub virtual_input: i32,
    pub virtual_output: i32,
}

impl Clone for Keybind {
    fn clone(&self) -> Self {
        Keybind {
            input: self.input.clone(),
            output: self.output.clone(),
            virtual_input: self.virtual_input,
            virtual_output: self.virtual_input,
        }
    }
}
