use alloc::sync::Arc;
use spin::Mutex;
use crate::task::{Task, ResourceRequirements};
use crate::resource_manager::ResourceManager;
use crate::scheduler::Scheduler;
use crate::println;

pub struct BatchSystem {
    scheduler: Mutex<Scheduler>,
    resource_manager: Arc<Mutex<ResourceManager>>,
}

impl BatchSystem {
    pub fn new(total_resources: ResourceRequirements) -> Self {
        let resource_manager = Arc::new(Mutex::new(ResourceManager::new(total_resources.clone())));
        BatchSystem {
            scheduler: Mutex::new(Scheduler::new(total_resources, Arc::clone(&resource_manager))),
            resource_manager,
        }
    }

    pub fn submit_task(&self, task: Task) {
        let mut scheduler = self.scheduler.lock();
        scheduler.add_task(task);
    }

    pub fn run(&self) {
        loop {
            let mut scheduler = self.scheduler.lock();
            if let Some(task) = scheduler.schedule_next_task() {
                drop(scheduler);

                println!("Executing task: {:?}", task);
                let mut resource_manager = self.resource_manager.lock();
                if resource_manager.allocate_resources(&task.resource_requirements) {
                    drop(resource_manager);

                    self.execute_task(&task);

                    let mut resource_manager = self.resource_manager.lock();
                    resource_manager.release_resources(&task.resource_requirements);
                } else {
                    println!("Unable to allocate resources for task: {:?}", task);
                }
            } else {
                break;
            }
        }
    }

    fn execute_task(&self, task: &Task) {
        println!("Starting task execution: {:?}", task);
        for _ in 0..1_000_000 {
            core::hint::spin_loop();
        }
        println!("Task completed: {:?}", task);
    }

    pub fn get_status(&self) -> BatchSystemStatus {
        let scheduler = self.scheduler.lock();
        let resource_manager = self.resource_manager.lock();

        BatchSystemStatus {
            tasks_queued: scheduler.get_queue_length(),
            resources_available: resource_manager.get_available_resources().clone(),
        }
    }
}

#[derive(Debug)]
pub struct BatchSystemStatus {
    pub tasks_queued: usize,
    pub resources_available: ResourceRequirements,
}

impl BatchSystemStatus {
    pub fn print(&self) {
        println!("Batch System Status:");
        println!("Tasks in queue: {}", self.tasks_queued);
        println!("Available CPU: {}", self.resources_available.cpu);
        println!("Available Memory: {}", self.resources_available.memory);
    }
}

