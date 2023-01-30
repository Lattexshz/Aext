use crate::aext::ExtensionType;

#[derive(Clone)]
pub struct ExtensionLock {
    // Plugin details
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: String,
    pub ext_type: ExtensionType
}