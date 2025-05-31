use alloc::sync::Arc;
use spin::Mutex;
use crate::task::{Task, ResourceRequirements, TaskStatus};
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
            scheduler: Mutex::new(Scheduler::new(total_resources)),
            resource_manager,
        }
    }

    pub fn submit_task(&self, task: Task) {
        let mut scheduler = self.scheduler.lock();
        scheduler.add_task(task);
        println!("[BATCH] Task submitted: {:?}", scheduler.get_last_task());
    }

    pub fn run(&self) {
        println!("\n[BATCH] Starting batch system execution...");
        let mut completed_tasks = 0;
        let mut failed_tasks = 0;

        loop {
            // Print current status
            self.get_status().print();

            let mut scheduler = self.scheduler.lock();
            if let Some(task) = scheduler.schedule_next_task() {
                drop(scheduler);

                println!("\n[BATCH] Executing task: {:?}", task);
                let mut resource_manager = self.resource_manager.lock();
                if resource_manager.allocate_resources(&task.resource_requirements) {
                    drop(resource_manager);

                    let status = self.execute_task(&task);
                    match status {
                        TaskStatus::Completed => completed_tasks += 1,
                        TaskStatus::Failed => failed_tasks += 1,
                        _ => {}
                    }

                    let mut resource_manager = self.resource_manager.lock();
                    resource_manager.release_resources(&task.resource_requirements);
                } else {
                    println!("[BATCH] Failed to allocate resources for task: {:?}", task);
                    println!("[BATCH] Required resources: CPU={}, Memory={}KB",
                        task.resource_requirements.cpu,
                        task.resource_requirements.memory);
                    failed_tasks += 1;
                }
            } else {
                println!("\n[BATCH] No more tasks to execute.");
                println!("[BATCH] Summary:");
                println!("  - Completed tasks: {}", completed_tasks);
                println!("  - Failed tasks: {}", failed_tasks);
                println!("  - Total tasks: {}", completed_tasks + failed_tasks);
                break;
            }
        }
    }

    fn execute_task(&self, task: &Task) -> TaskStatus {
        println!("[TASK] Starting execution: {}", task.executable);
        println!("[TASK] Priority: {}", task.priority);
        println!("[TASK] Resources: CPU={}, Memory={}KB",
            task.resource_requirements.cpu,
            task.resource_requirements.memory);

        // Simulate work based on task priority
        let iterations = task.priority * 50_000; // Reduced for faster execution
        for i in 0..iterations {
            if i % 5_000 == 0 {
                println!("[TASK] Progress: {}%", (i * 100) / iterations);
            }
            core::hint::spin_loop();
        }

        println!("[TASK] Execution completed: {}", task.executable);
        TaskStatus::Completed
    }

    pub fn get_status(&self) -> BatchSystemStatus {
        let scheduler = self.scheduler.lock();
        let resource_manager = self.resource_manager.lock();

        BatchSystemStatus {
            tasks_queued: scheduler.get_queue_length(),
            resources_available: resource_manager.get_available_resources().clone(),
            next_task_priority: scheduler.get_next_task_priority(),
        }
    }
}

#[derive(Debug)]
pub struct BatchSystemStatus {
    pub tasks_queued: usize,
    pub resources_available: ResourceRequirements,
    pub next_task_priority: Option<u32>,
}

impl BatchSystemStatus {
    pub fn print(&self) {
        println!("\n[BATCH] Current System Status:");
        println!("  - Tasks in queue: {}", self.tasks_queued);
        println!("  - Available CPU: {}", self.resources_available.cpu);
        println!("  - Available Memory: {}KB", self.resources_available.memory);
        if let Some(priority) = self.next_task_priority {
            println!("  - Next task priority: {}", priority);
        }
    }
}

