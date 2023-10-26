use core::arch::asm;

const EXIT_SUCCESS: u32 = 0x5555; // Equals `exit(0)`. qemu successful exit

const EXIT_FAILURE_FLAG: u32 = 0x3333;
const EXIT_FAILURE: u32 = exit_code_encode(1);
// Equals `exit(1)`. qemu failed exit
const EXIT_RESET: u32 = 0x7777; // qemu reset

pub trait QEMUExit {
    fn exit(&self, code: u32) -> !;

    fn exit_success(&self) -> !;

    fn exit_failure(&self) -> !;
}

pub struct RISCV64 {
    /// Address of the sifive_test mapped device.
    addr: u64,
}

/// Encode the exit code using EXIT_FAILURE_FLAG.
const fn exit_code_encode(code: u32) -> u32 {
    (code << 16) | EXIT_FAILURE_FLAG
}

impl RISCV64 {
    pub const fn new(addr: u64) -> Self {
        RISCV64 { addr: addr }
    }
}


impl QEMUExit for RISCV64 {
    fn exit(&self, code: u32) -> ! {
        let code_new = match code {
            EXIT_SUCCESS | EXIT_FAILURE | EXIT_RESET => code,
            _ => exit_code_encode(code),
        };

        unsafe {
            asm!(
            "sw {0}, 0({1})",
            in(reg)code_new, in(reg)self.addr
            );

            loop {
                asm!("wfi", options(nomem, nostack))
            }
        }
    }

    fn exit_success(&self) -> ! {
        self.exit(EXIT_SUCCESS);
    }

    fn exit_failure(&self) -> ! {
        self.exit(EXIT_FAILURE);
    }
}


const VIRT_TEST: u64 = 0x100000;

pub const QEMU_EXIT_HANDLE: RISCV64 = RISCV64::new(VIRT_TEST);



