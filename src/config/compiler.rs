use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Compiler {
    pub cc: String,
    pub cflags: Option<String>,
}
