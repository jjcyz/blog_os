use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use core::ptr::{read_volatile, write_volatile};
use crate::println;

// QEMU virt machine UART address
const UART0: u64 = 0x1000_0000;

#[repr(C)]
struct UartRegisters {
    thr_rbr: u8, // Transmit/Receive
    ier: u8,     // Interrupt Enable
    fcr_iir: u8, // FIFO Control
    lcr: u8,     // Line Control
    mcr: u8,     // Modem Control
    lsr: u8,     // Line Status
    msr: u8,     // Modem Status
    spr: u8,     // Scratch Pad
}

lazy_static! {
    pub static ref UART: Mutex<Uart> = Mutex::new(Uart::new());
}

pub struct Uart {
    registers: *mut UartRegisters,
}

// Safe because we only access MMIO registers
unsafe impl Send for Uart {}

impl Uart {
    pub const fn new() -> Self {
        Uart {
            registers: UART0 as *mut UartRegisters,
        }
    }

    pub fn init(&mut self) {
        unsafe {
            // Disable all interrupts
            self.write_reg(1, 0x00);
            println!("UART: Disabled interrupts");

            // Enable DLAB (set baud rate divisor)
            self.write_reg(3, 0x80);
            println!("UART: Enabled DLAB");

            // Set divisor to 1 (115200 baud)
            self.write_reg(0, 0x01);
            self.write_reg(1, 0x00);
            println!("UART: Set baud rate");

            // 8 bits, no parity, one stop bit
            self.write_reg(3, 0x03);
            println!("UART: Set line control");

            // Enable FIFO, clear them, with 14-byte threshold
            self.write_reg(2, 0xC7);
            println!("UART: Enabled FIFO");

            // Mark data terminal ready, signal request to send
            self.write_reg(4, 0x0B);
            println!("UART: Set modem control");

            // Enable received data available interrupt
            self.write_reg(1, 0x01);
            println!("UART: Enabled receive interrupt");
        }
    }

    unsafe fn write_reg(&mut self, offset: usize, value: u8) {
        write_volatile((self.registers as *mut u8).add(offset), value);
    }

    unsafe fn read_reg(&mut self, offset: usize) -> u8 {
        read_volatile((self.registers as *const u8).add(offset))
    }

    pub fn put_byte(&mut self, byte: u8) {
        while !self.is_transmit_empty() {}
        unsafe {
            self.write_reg(0, byte);
        }
    }

    pub fn get_byte(&mut self) -> Option<u8> {
        if self.is_data_ready() {
            Some(unsafe { self.read_reg(0) })
        } else {
            None
        }
    }

    fn is_transmit_empty(&mut self) -> bool {
        unsafe { self.read_reg(5) & (1 << 5) != 0 }
    }

    fn is_data_ready(&mut self) -> bool {
        unsafe { self.read_reg(5) & 1 != 0 }
    }
}

impl fmt::Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.put_byte(byte);
        }
        Ok(())
    }
}

pub fn init() {
    use crate::println;
    UART.lock().init();
    println!("UART initialized");
}
