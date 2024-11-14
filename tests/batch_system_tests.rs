#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::string::String;
use alloc::vec;
use alloc::sync::Arc;
use spin::Mutex;
use core::panic::PanicInfo;
use blog_os::{serial_print, serial_println};
use blog_os::scheduler::Scheduler;
use blog_os::resource_manager::ResourceManager;
use blog_os::task::{Task, ResourceRequirements};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn test_add_task() {
    serial_print!("test_add_task... ");

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
        executable: String::from("task1"),
        arguments: vec![String::from("arg1"), String::from("arg2")],
        priority: 1,
        resource_requirements: ResourceRequirements {
            cpu: 2,
            memory: 2_000,
        },
    };

    scheduler.add_task(task.clone());
    assert_eq!(scheduler.task_queue.front().unwrap(), &task);
    serial_println!("[ok]");
}

#[test_case]
fn test_schedule_task() {
    serial_print!("test_schedule_task... ");

    let resource_manager = Arc::new(Mutex::new(ResourceManager::new(ResourceRequirements {
        cpu: 8,
        memory: 16_000,
    })));

    let mut scheduler = Scheduler::new(
        ResourceRequirements {
            cpu: 8,
            memory: 16_000,
        },
        Arc::clone(&resource_manager),
    );

    let task1 = Task {
        executable: String::from("task1"),
        arguments: vec![String::from("arg1"), String::from("arg2")],
        priority: 1,
        resource_requirements: ResourceRequirements {
            cpu: 2,
            memory: 2_000,
        },
    };

    let task2 = Task {
        executable: String::from("task2"),
        arguments: vec![String::from("arg3"), String::from("arg4")],
        priority: 2,
        resource_requirements: ResourceRequirements {
            cpu: 4,
            memory: 4_000,
        },
    };

    scheduler.add_task(task1.clone());
    scheduler.add_task(task2.clone());

    let scheduled_task = scheduler.schedule_next_task().unwrap();
    assert_eq!(scheduled_task, task1);

    let mut resource_manager = resource_manager.lock();
    assert!(resource_manager.allocate_resources(&task1.resource_requirements));
    resource_manager.release_resources(&task1.resource_requirements);
    serial_println!("[ok]");
}

#[test_case]
fn test_resource_allocation() {
    serial_print!("test_resource_allocation... ");

    let mut resource_manager = ResourceManager::new(ResourceRequirements {
        cpu: 8,
        memory: 16_000,
    });

    let requirements = ResourceRequirements {
        cpu: 4,
        memory: 4_000,
    };

    assert!(resource_manager.allocate_resources(&requirements));
    assert_eq!(
        resource_manager.available_resources,
        ResourceRequirements {
            cpu: 4,
            memory: 12_000
        }
    );

    resource_manager.release_resources(&requirements);
    assert_eq!(
        resource_manager.available_resources,
        ResourceRequirements {
            cpu: 8,
            memory: 16_000
        }
    );
    serial_println!("[ok]");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
