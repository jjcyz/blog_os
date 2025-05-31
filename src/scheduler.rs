use alloc::collections::VecDeque;
use crate::task::{Task, ResourceRequirements, TaskStatus};

pub struct Scheduler {
    pub task_queue: VecDeque<Task>,
    available_resources: ResourceRequirements,
}

impl Scheduler {
    pub fn new(available_resources: ResourceRequirements) -> Self {
        Scheduler {
            task_queue: VecDeque::new(),
            available_resources,
        }
    }

    pub fn add_task(&mut self, mut task: Task) {
        task.status = TaskStatus::Queued;
        self.task_queue.push_back(task);
    }

    pub fn schedule_next_task(&mut self) -> Option<Task> {
        // Find the highest priority task that can be executed
        let mut highest_priority_idx = None;
        let mut highest_priority = 0;

        for (idx, task) in self.task_queue.iter().enumerate() {
            if task.priority > highest_priority && self.can_execute_task(task) {
                highest_priority = task.priority;
                highest_priority_idx = Some(idx);
            }
        }

        if let Some(idx) = highest_priority_idx {
            let mut task = self.task_queue.remove(idx).unwrap();
            task.status = TaskStatus::Running;
            self.update_available_resources(&task);
            Some(task)
        } else {
            None
        }
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

    pub fn get_last_task_priority(&self) -> u32 {
        self.task_queue.back().map(|t| t.priority).unwrap_or(0)
    }

    pub fn get_next_task_priority(&self) -> Option<u32> {
        self.task_queue.front().map(|t| t.priority)
    }

    pub fn get_last_task(&self) -> Option<&Task> {
        self.task_queue.back()
    }
}
