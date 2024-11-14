pub mod common {
	use super::*;

	pub fn setup_test_scheduler() -> (Scheduler, Arc<Mutex<ResourceManager>>) {
			let resource_manager = Arc::new(Mutex::new(ResourceManager::new(ResourceRequirements {
					cpu: 8,
					memory: 16_000,
			})));

			let scheduler = Scheduler::new(
					ResourceRequirements {
							cpu: 8,
							memory: 16_000,
					},
					Arc::clone(&resource_manager),
			);

			(scheduler, resource_manager)
	}

	pub fn create_test_task(id: &str, priority: u32, cpu: u32, memory: u64) -> Task {
			Task {
					executable: format!("task{}", id),
					arguments: vec![format!("arg{}_1", id), format!("arg{}_2", id)],
					priority,
					resource_requirements: ResourceRequirements {
							cpu,
							memory,
					},
			}
	}
}
