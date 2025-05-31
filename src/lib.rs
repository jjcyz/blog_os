#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(alloc_error_handler)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

pub mod scheduler;
pub mod resource_manager;
pub mod task;
pub mod uart;
pub mod interrupts;
pub mod memory;
pub mod batch_system;
pub mod executor;

use spin::Mutex;
use core::alloc::GlobalAlloc;
use core::alloc::Layout;
use core::ptr::null_mut;

// Add this near the top of lib.rs
pub fn init_uart() {
    uart::init();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::uart::UART.lock(), $($arg)*);
    });
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// Heap allocator
pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
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

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
pub static ALLOCATOR: Locked<HeapAllocator> = Locked::new(HeapAllocator::new());

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub const HEAP_START: usize = 0x_4444_4444_0000;
pub const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

/// Initialize the heap allocator
pub unsafe fn init_heap() {
    ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    println!("All tests passed!");
}
