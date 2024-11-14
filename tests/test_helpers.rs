// In tests/test_helpers.rs
#![cfg(test)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(test, allow(unused_imports))]

use super::*;

#[cfg(not(target_os = "none"))]
pub fn init_test_env() {
    // Host system test initialization
}

#[cfg(target_os = "none")]
pub fn init_test_env() {
    // Bare metal test initialization
}

// In your main test file
#[cfg(test)]
mod tests {
    use super::*;

    // Tests that can run on both environments
    mod common_tests {
        use super::*;

        #[test]
        fn test_add_task() {
            let mut scheduler = Scheduler::new(
                ResourceRequirements {
                    cpu: 8,
                    memory: 16_000,
                },
                Arc::new(Mutex::new(ResourceManager::new(ResourceRequirements {
                    cpu: 8,
                    memory: 16_000,
                }))),
            );

            let task = Task {
                executable: "task1".to_string(),
                arguments: vec!["arg1".to_string(), "arg2".to_string()],
                priority: 1,
                resource_requirements: ResourceRequirements {
                    cpu: 2,
                    memory: 2_000,
                },
            };

            scheduler.add_task(task.clone());
            assert_eq!(scheduler.task_queue.front().unwrap(), &task);
        }
    }

    // Host-only tests
    #[cfg(not(target_os = "none"))]
    mod host_tests {
        use super::*;

        #[tokio::test]
        async fn test_concurrent_scheduling() {
            // Your concurrent test code here
        }
    }

    // Bare metal tests
    #[cfg(target_os = "none")]
    mod os_tests {
        use super::*;

        #[test_case]
        fn test_bare_metal_scheduling() {
            // Your bare metal specific test code here
        }
    }
}
