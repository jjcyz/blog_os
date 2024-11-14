use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::Mutex;
use crate::task::{Task, ResourceRequirements};
use crate::resource_manager::ResourceManager;

pub struct Scheduler {
    pub task_queue: VecDeque<Task>,
    available_resources: ResourceRequirements,
    resource_manager: Arc<Mutex<ResourceManager>>,
}

impl Scheduler {
    pub fn new(available_resources: ResourceRequirements, resource_manager: Arc<Mutex<ResourceManager>>) -> Self {
        Scheduler {
            task_queue: VecDeque::new(),
            available_resources,
            resource_manager,
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.task_queue.push_back(task);
    }

    pub fn schedule_next_task(&mut self) -> Option<Task> {
        while let Some(task) = self.task_queue.pop_front() {
            if self.can_execute_task(&task) {
                self.update_available_resources(&task);
                return Some(task);
            } else {
                self.task_queue.push_back(task);
            }
        }
        None
    }

    fn can_execute_task(&self, task: &Task) -> bool {
        task.resource_requirements.cpu <= self.available_resources.cpu
            && task.resource_requirements.memory <= self.available_resources.memory
    }

    fn update_available_resources(&mut self, task: &Task) {
        self.available_resources.cpu -= task.resource_requirements.cpu;
        self.available_resources.memory -= task.resource_requirements.memory;
    }

    pub fn get_queue_length(&self) -> usize {
        self.task_queue.len()
    }
}
