use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Task {
    pub executable: String,
    pub arguments: Vec<String>,
    pub priority: u32,
    pub resource_requirements: ResourceRequirements,
}

#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub cpu: u32,
    pub memory: u32,
}
