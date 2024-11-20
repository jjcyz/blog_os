use riscv::register::{
	mtvec, mstatus, mie,
	mcause::{self, Trap, Exception, Interrupt},
	mtval,
};
use crate::println;

#[repr(C)]
pub struct TrapFrame {
	pub regs: [usize; 32],  // x0-x31
	pub fregs: [usize; 32], // f0-f31
	pub pc: usize,          // program counter
}

#[no_mangle]
#[link_section = ".text.trap_vector"]
pub extern "C" fn trap_handler(trap_frame: &mut TrapFrame) {
	let cause = mcause::read();
	let epc = trap_frame.pc;

	match cause.cause() {
			Trap::Exception(exception) => {
					handle_exception(exception, epc, trap_frame);
			}
			Trap::Interrupt(interrupt) => {
					handle_interrupt(interrupt);
			}
	}
}

fn handle_exception(exception: Exception, epc: usize, trap_frame: &mut TrapFrame) {
	match exception {
			Exception::InstructionMisaligned => {
					println!("InstructionMisaligned at {:#x}", epc);
					panic!("InstructionMisaligned exception");
			}
			Exception::InstructionFault => {
					println!("InstructionFault at {:#x}", epc);
					panic!("InstructionFault exception");
			}
			Exception::IllegalInstruction => {
					println!("IllegalInstruction at {:#x}: {:#x}", epc, mtval::read());
					panic!("IllegalInstruction exception");
			}
			Exception::Breakpoint => {
					println!("Breakpoint at {:#x}", epc);
					trap_frame.pc = epc + 2;
			}
			Exception::LoadFault => {
					println!("LoadFault at {:#x}: accessing {:#x}", epc, mtval::read());
					panic!("LoadFault exception");
			}
			Exception::StoreFault => {
					println!("StoreFault at {:#x}: accessing {:#x}", epc, mtval::read());
					panic!("StoreFault exception");
			}
			_ => {
					println!("Unhandled exception: {:?} at {:#x}", exception, epc);
					panic!("Unhandled exception");
			}
	}
}

fn handle_interrupt(interrupt: Interrupt) {
	match interrupt {
			Interrupt::MachineTimer => {
					// Clear the timer interrupt
					unsafe {
							mie::clear_mtimer();
					}
					println!("Timer interrupt");
			}
			Interrupt::MachineSoft => {
					// Clear software interrupt
					unsafe {
							mie::clear_msoft();
					}
					println!("Software interrupt");
			}
			Interrupt::MachineExternal => {
					println!("External interrupt");
			}
			_ => {
					println!("Unhandled interrupt: {:?}", interrupt);
			}
	}
}

pub fn init() {
	unsafe {
			// Set up trap vector
			mtvec::write(trap_handler as usize, mtvec::TrapMode::Direct);

			// Enable machine-mode interrupts
			mstatus::set_mie();

			// Enable specific interrupts
			mie::set_mext();    // external
			mie::set_mtimer();  // timer
			mie::set_msoft();   // software
	}

	println!("RISC-V interrupt handling initialized");
}
