#![allow(unused_imports)]

#[cfg(feature = "dynamic")]
use serde_dynamic;

use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct Cat {
    name: String,
}

impl Cat {
    pub fn new() -> Self {
        Self {
            name: "kitty".into(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
}
