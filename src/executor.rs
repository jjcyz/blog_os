use crate::task::Task;
use crate::println; // Use your custom println macro

pub fn execute_task(task: &Task) {
    println!("Executing task: {:?}", task);

    // Instead of thread::sleep, use a simple spin loop
    for _ in 0..1000000 {
        core::hint::spin_loop();
    }

    println!("Task completed: {:?}", task);
}
