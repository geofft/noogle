use std::{fs, path::PathBuf, rc::Rc};

use serde::{Deserialize, Serialize};

use crate::position::FilePosition;

#[serde_with::skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LambdaMeta {
    #[allow(non_snake_case)]
    pub isPrimop: bool,
    pub name: Option<String>,
    pub position: Option<FilePosition>,
    pub args: Option<Vec<String>>,
    pub experimental: Option<bool>,
    pub arity: Option<usize>,

    // I want to potentially overwrite those two
    pub content: Option<String>,
    #[allow(non_snake_case)]
    pub countApplied: Option<usize>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttrMeta {
    pub position: Option<FilePosition>,
    /// I want to add this
    pub content: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocsMeta {
    pub lambda: Option<LambdaMeta>,
    pub attr: AttrMeta,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Docs {
    pub docs: DocsMeta,
    pub aliases: Option<Vec<Rc<Vec<String>>>>,
    pub path: Rc<Vec<String>>,
}

pub fn read_pasta(path: &PathBuf) -> Vec<Docs> {
    let raw = fs::read_to_string(&path);
    match raw {
        Ok(content) => {
            let data: Vec<Docs> = serde_json::from_str(&content).unwrap();
            data
        }
        Err(e) => {
            panic!("Could not parse input data: {}", e)
        }
    }
}
