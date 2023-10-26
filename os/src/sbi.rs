// SBI 系统调用接口

use core::arch::asm;

// 标准数据std 1
const SBI_CONSOLE_PUTCHAR: usize = 1;

// sbi系统调用
/// 在Rust中，`asm!`宏用于嵌入汇编代码。它允许你直接在Rust代码中编写汇编指令，以便与Rust代码进行混合编程。
//
// 以下是`asm!`宏的基本语法：
//
// ```rust
// asm!(assembly_template
//     : output_operands
//     : input_operands
//     : clobbers
//     : options
// );
// ```
//
// - `assembly_template`是包含汇编指令的字符串模板。你可以在模板中使用占位符来引用输入和输出操作数。
// - `output_operands`是一个逗号分隔的输出操作数列表。这些操作数将被写入到Rust变量中。
// - `input_operands`是一个逗号分隔的输入操作数列表。这些操作数将从Rust变量中读取。
// - `clobbers`是一个逗号分隔的寄存器列表，表示在汇编代码中被修改的寄存器。
// - `options`是一个逗号分隔的选项列表，用于指定汇编代码的行为。
//
// 下面是一个简单的示例，展示了如何使用`asm!`宏在Rust中嵌入汇编代码：
//
// ```rust
// fn main() {
//     let input = 42;
//     let output: u32;
//
//     unsafe {
//         asm!(
//             "mov $0, $1"
//             : "=r"(output)
//             : "r"(input)
//         );
//     }
//
//     println!("Output: {}", output);
// }
// ```
//
// 在这个示例中，我们使用`asm!`宏将输入变量`input`的值复制到输出变量`output`中。`"mov $0, $1"`是汇编指令模板，`$0`和`$1`是占位符，分别对应输出和输入操作数的位置。`"=r"(output)`和`"r"(input)`分别指定了输出和输入操作数的约束。
//
// 请注意，使用`asm!`宏需要使用`unsafe`块，因为它涉及到直接操作底层的硬件和寄存器。确保在使用`asm!`宏时要小心，并遵循正确的汇编语法和约束规则。



///这段代码是使用`asm!`宏在Rust中嵌入汇编代码，并将返回值存储在`ret`变量中。根据代码中的汇编指令，是在RISC-V架构上运行的。
// 在这段代码中，`li x16, 0`指令将立即数0加载到寄存器`x16`中，`ecall`指令触发一个系统调用。
// `inlateout("x10") arg0 => ret`表示将`arg0`的值加载到寄存器`x10`中，并将`ret`的值存储回`arg0`。
// `in("x11") arg1`表示将`arg1`的值加载到寄存器`x11`中。
// `in("x12") arg2`表示将`arg2`的值加载到寄存器`x12`中。
// `in("x17") which`表示将`which`的值加载到寄存器`x17`中。
// 最后，`ret`变量将作为整个`asm!`块的返回值。
// 请注意，使用`unsafe`块是因为`asm!`宏涉及到直接操作底层的硬件和寄存器。确保在使用`asm!`宏时要小心，并遵循正确的汇编语法和约束规则。
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        asm!(
        "li x16, 0",
        "ecall",
        inlateout("x10") arg0 => ret,
        in("x11") arg1,
        in("x12") arg2,
        in("x17") which,
        );
    }
    ret
}


pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}


use crate::board::qemu::QEMUExit;

pub fn shutdown() -> ! {
    crate::board::qemu::QEMU_EXIT_HANDLE.exit_failure();
}

