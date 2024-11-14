use crate::task::ResourceRequirements;

pub struct ResourceManager {
    pub total_resources: ResourceRequirements,
    pub available_resources: ResourceRequirements,
}

impl ResourceManager {
    pub fn new(total_resources: ResourceRequirements) -> Self {
        ResourceManager {
            total_resources: total_resources.clone(),
            available_resources: total_resources,
        }
    }

    pub fn allocate_resources(&mut self, requirements: &ResourceRequirements) -> bool {
        if requirements.cpu <= self.available_resources.cpu
            && requirements.memory <= self.available_resources.memory
        {
            self.available_resources.cpu -= requirements.cpu;
            self.available_resources.memory -= requirements.memory;
            true
        } else {
            false
        }
    }

    pub fn release_resources(&mut self, requirements: &ResourceRequirements) {
        self.available_resources.cpu += requirements.cpu;
        self.available_resources.memory += requirements.memory;
    }

    pub fn get_available_resources(&self) -> &ResourceRequirements {
        &self.available_resources
    }
}
