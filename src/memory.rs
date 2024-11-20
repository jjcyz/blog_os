use core::sync::atomic::{AtomicBool, Ordering};
use spin::Mutex;
use crate::println;

pub const HEAP_START: usize = 0x80400000;
pub const HEAP_SIZE: usize = 1024 * 1024; // 1MB

// Page size for RISC-V Sv39/Sv48
pub const PAGE_SIZE: usize = 4096;
pub const PAGE_TABLE_ENTRIES: usize = 512;

static INIT: AtomicBool = AtomicBool::new(false);

#[repr(C, align(4096))]
pub struct PageTable {
    entries: [PageTableEntry; PAGE_TABLE_ENTRIES],
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    pub const fn new_empty() -> Self {
        PageTableEntry(0)
    }

    pub fn is_valid(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn set_entry(&mut self, ppn: u64, flags: u64) {
        self.0 = (ppn << 10) | flags | 1;
    }
}

pub struct MemoryManager {
    next_free_page: usize,
}

impl MemoryManager {
    pub const fn new() -> Self {
        MemoryManager {
            next_free_page: HEAP_START + HEAP_SIZE,
        }
    }

    pub fn alloc_page(&mut self) -> Option<*mut u8> {
        let page = self.next_free_page;
        self.next_free_page += PAGE_SIZE;
        Some(page as *mut u8)
    }
}

lazy_static::lazy_static! {
    pub static ref MEMORY_MANAGER: Mutex<MemoryManager> = Mutex::new(MemoryManager::new());
}

pub fn init() {
    if INIT.swap(true, Ordering::SeqCst) {
        return;
    }

    // Initialize root page table
    let root_table = PageTable {
        entries: [PageTableEntry::new_empty(); PAGE_TABLE_ENTRIES],
    };

    // Set up SATP register for Sv39
    unsafe {
        let satp_value = (8 << 60) | // Sv39 mode
                        (0 << 44) | // ASID
                        ((root_table.entries.as_ptr() as usize >> 12) & ((1 << 44) - 1));
        riscv::register::satp::write(satp_value);

        // Flush TLB
        riscv::asm::sfence_vma_all();
    }

    println!("Memory management initialized");
}
