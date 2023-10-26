//! SBI 终端输出
use crate::sbi::console_putchar;
use core::fmt::{Write, Arguments, Result};


struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: Arguments) {
    Stdout.write_fmt(args).unwrap();
}

/// Print! to the host console using the format string and arguments.
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

/// Println! to the host console using the format string and arguments.
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}



