#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use blog_os::{println, HEAP_START, HEAP_SIZE};
use blog_os::batch_system::BatchSystem;
use blog_os::task::{Task, ResourceRequirements};
use blog_os::ALLOCATOR;
use riscv::register::{mhartid, marchid, mimpid, mvendorid};

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Initialize early console
    blog_os::uart::init();

    // Print boot banner
    println!("\n==========================================");
    println!("RISC-V Kernel Booting on Hart {}", mhartid::read());
    println!("==========================================\n");

    // Print hardware info
    println!("Hardware Information:");
    println!("  Vendor ID: {:?}", mvendorid::read());
    println!("  Architecture ID: {:?}", marchid::read());
    println!("  Implementation ID: {:?}", mimpid::read());
    println!("");

    // Initialize core subsystems
    println!("Initializing Core Subsystems:");

    println!("→ Initializing interrupt handling...");
    blog_os::interrupts::init();
    println!("  [OK] Interrupts initialized");

    println!("→ Initializing memory management...");
    blog_os::memory::init();
    println!("  [OK] Memory management initialized");

    println!("→ Initializing heap allocator...");
    ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);

    println!("  [OK] Heap initialized at 0x{:x} with size {}KB", HEAP_START, HEAP_SIZE / 1024);
    println!("");

    // Initialize batch system
    println!("Initializing Batch System:");
    let batch_system = BatchSystem::new(ResourceRequirements {
        cpu: 4,
        memory: 1024,
    });
    println!("  [OK] Batch system initialized with 4 CPUs and 1024KB memory\n");

    // Create test tasks
    println!("Creating Test Tasks:");
    let task1 = Task {
        executable: alloc::string::String::from("task1"),
        arguments: alloc::vec![alloc::string::String::from("arg1")],
        priority: 1,
        resource_requirements: ResourceRequirements {
            cpu: 1,
            memory: 256,
        },
    };
    println!("  [+] Created Task 1 (Priority: 1, Memory: 256KB)");

    let task2 = Task {
        executable: alloc::string::String::from("task2"),
        arguments: alloc::vec![alloc::string::String::from("arg2")],
        priority: 2,
        resource_requirements: ResourceRequirements {
            cpu: 1,
            memory: 512,
        },
    };
    println!("  [+] Created Task 2 (Priority: 2, Memory: 512KB)\n");

    // Submit and run tasks
    println!("Submitting tasks to batch system...");
    batch_system.submit_task(task1);
    batch_system.submit_task(task2);
    println!("  [OK] Tasks submitted successfully\n");

    println!("Starting batch system execution...");
    batch_system.run();
    println!("  [OK] All tasks completed successfully\n");

    // Enter main loop
    println!("Kernel initialization complete!");
    println!("Entering main loop...\n");

    loop {
        unsafe {
            // Wait for interrupts, then handle them
            riscv::asm::wfi();
        }
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n!!! KERNEL PANIC !!!");
    println!("Error: {}\n", info);
    println!("System halted.");

    loop {
        unsafe {
            // Wait for debugger or reset
            riscv::asm::wfi();
        }
    }
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    println!("All tests passed!");
}
