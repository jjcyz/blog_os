#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

#[macro_use]
mod vga_buffer;
mod interrupts;
mod batch_system;
mod scheduler;
mod task;
mod resource_manager;
mod executor;

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::panic::PanicInfo;
use bootloader::BootInfo;
use crate::batch_system::BatchSystem;
use crate::task::{Task, ResourceRequirements};

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

pub struct HeapAllocator {
    heap_start: usize,
    heap_end: usize,
}

impl HeapAllocator {
    pub const fn new() -> Self {
        HeapAllocator {
            heap_start: 0,
            heap_end: 0,
        }
    }

    pub fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr
    } else {
        addr - remainder + align
    }
}

unsafe impl GlobalAlloc for Locked<HeapAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut allocator = self.lock();

        let alloc_start = align_up(allocator.heap_start, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return null_mut(),
        };

        if alloc_end <= allocator.heap_end {
            allocator.heap_start = alloc_end;
            alloc_start as *mut u8
        } else {
            null_mut()
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // This is a simple bump allocator, deallocation is not implemented
    }
}

#[global_allocator]
static ALLOCATOR: Locked<HeapAllocator> = Locked::new(HeapAllocator::new());

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

#[no_mangle]
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    // Initialize heap
    let heap_start = 0x_4444_4444_0000;
    let heap_size = 100 * 1024; // 100 KiB
    unsafe {
        ALLOCATOR.lock().init(heap_start, heap_size);
    }

    // Initialize interrupts
    interrupts::init_idt();

    let batch_system = BatchSystem::new(ResourceRequirements {
        cpu: 4,
        memory: 1024,
    });

    // Create test tasks
    let task1 = Task {
        executable: alloc::string::String::from("task1"),
        arguments: alloc::vec![alloc::string::String::from("arg1")],
        priority: 1,
        resource_requirements: ResourceRequirements {
            cpu: 1,
            memory: 256,
        },
    };

    let task2 = Task {
        executable: alloc::string::String::from("task2"),
        arguments: alloc::vec![alloc::string::String::from("arg2")],
        priority: 2,
        resource_requirements: ResourceRequirements {
            cpu: 1,
            memory: 512,
        },
    };

    println!("Batch System Initialized");
    println!("Submitting tasks...");

    batch_system.submit_task(task1);
    batch_system.submit_task(task2);

    println!("Running batch system...");
    batch_system.run();

    println!("All tasks completed");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
