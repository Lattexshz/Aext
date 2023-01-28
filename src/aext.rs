use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Aext {
    pub project:Option<ProjectConfig>
}

#[derive(Debug,Deserialize)]
pub struct ProjectConfig {
    pub name: Option<String>
}

pub struct AextError {
    error: _AextError,
}

impl AextError {
    pub fn illegal_argument(str: impl Into<String>) -> Self {
        Self {
            error: _AextError::IllegalArgument(str.into()),
        }
    }

    pub fn required_field(str: impl Into<String>) -> Self {
        Self {
            error: _AextError::RequiredField(str.into())
        }
    }
}

enum _AextError {
    IllegalArgument(String),
    RequiredField(String)
}

impl _AextError {
    pub fn description(&self) -> &str {
        match self {
            _AextError::IllegalArgument(s) => s.as_str(),
            _AextError::RequiredField(s) => s.as_str()
        }
    }
}

impl fmt::Display for AextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error {
            _AextError::IllegalArgument(s) => f.write_str(s),
            _AextError::RequiredField(s) => f.write_str(s)
        }
    }
}

impl Debug for AextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.error.description()))
    }
}

pub fn parse_aext(path:Vec<PathBuf>) -> Vec<Aext> {
    let mut aexts:Vec<Aext> = vec!();
    if path.len() == 0 {
        return vec!();
    }

    for p in path {
        let mut f = match File::open(p) {
            Ok(f) => f,
            Err(_) => {
                println!("Warning: Could not open file for some reason");
                continue;
            }
        };

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");

        // テキストは\n{}です
        println!("With text:\n{}", contents);
        let decoded:Aext = toml::from_str(&*contents).unwrap();
        println!("{:#?}", decoded);
        aexts.push(decoded)
    }

    aexts
}