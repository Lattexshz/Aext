use crate::aext::ExtensionType;

#[derive(Clone)]
pub struct ExtensionLock {
    // Plugin details
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
}

#[derive(Clone)]
pub struct CommandLock {
    pub name: String,
    pub version: String,
    pub description: String,
}
