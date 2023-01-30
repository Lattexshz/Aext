use std::fmt;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use serde::Deserialize;
use crate::lock::ExtensionLock;

#[derive(Copy, Clone)]
pub enum ExtensionType {
    Extension,
    Command
}

#[derive(Clone, Debug, Deserialize)]
pub struct Aext {
    pub plugin: Option<PluginConfig>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct PluginConfig {
    pub name: Option<String>,
    pub version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub ext_type: Option<String>
}

#[derive(Clone,Debug,Deserialize)]
pub struct ExecuteConfig {
    pub step: Option<StepConfig>
}

#[derive(Clone,Debug,Deserialize)]
pub struct StepConfig {
    pub program: Option<String>,
    pub command: Option<String>
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
            error: _AextError::RequiredField(str.into()),
        }
    }
}

enum _AextError {
    IllegalArgument(String),
    RequiredField(String),
}

impl _AextError {
    pub fn description(&self) -> &str {
        match self {
            _AextError::IllegalArgument(s) => s.as_str(),
            _AextError::RequiredField(s) => s.as_str(),
        }
    }
}

impl fmt::Display for AextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.error {
            _AextError::IllegalArgument(s) => f.write_str(s),
            _AextError::RequiredField(s) => f.write_str(s),
        }
    }
}

impl Debug for AextError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("{}", self.error.description()))
    }
}

pub fn parse_aext(path: Vec<PathBuf>) -> Vec<ExtensionLock> {
    let mut aexts: Vec<ExtensionLock> = vec![];
    if path.is_empty() {
        return vec![];
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
        let decoded: Aext = toml::from_str(&contents).unwrap();
        check_script(decoded.clone());
        aexts.push(decoded.into())
    }

    aexts
}

fn check_script(aext: Aext) {
    let plugin = match aext.plugin {
        None => {
            println!(
                "error: [project] is not defined.
This field is required
note:Are you using the 'Project' as an upper case?"
            );
            std::process::exit(1);
        }
        Some(p) => p,
    };

    match plugin.name {
        None => {
            println!(
                "error: [project][name] is not defined.
This field is required"
            );
            std::process::exit(1)
        }
        Some(_) => {}
    }
}


impl Into<ExtensionLock> for Aext {
    fn into(self) -> ExtensionLock {
        let config = match self.plugin {
            None => {
                eprintln!("error: [plugin] is not defined.");
                std::process::exit(1)
            },
            Some(c) => c
        };
        ExtensionLock {
            name: match config.name {
                None => std::process::exit(1),
                Some(n) => n
            },
            version: match config.version {
                None =>  {
                    eprintln!("error: version is not defined.\nthis field is required.");
                    std::process::exit(1);
                }
                Some(v) => v
            },
            authors: match config.authors {
                None => {
                    println!("warning:Authors is not defined.");
                    vec![]
                }
                Some(a) => a
            },
            description: match config.description {
                None => {
                    println!("warning:Description is not defined.");
                    String::new()
                }
                Some(d) => d
            },
            ext_type: ExtensionType::Extension,
        }
    }
}