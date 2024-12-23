macro_rules! success {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("\x1b[32m[SUCCESS]\x1b[0m {}", format!($($arg)*));
    }};
}

macro_rules! info {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        const INFO_NAME: &str = "mmu";
        println!("\x1b[34m[{}]\x1b[0m {}", INFO_NAME, format!($($arg)*));
    }};
}

macro_rules! warning {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("\x1b[33m[WARNING]\x1b[0m {}", format!($($arg)*));
    }};
}

macro_rules! fatal {
    () => {
        $crate::print!("\n")
    };
    ($($arg:tt)*) => {{
        println!("\x1b[31m\x1b[1m[FATAL] {}\x1b[0m ", format!($($arg)*));
    }};
}

pub(crate) use success;
pub(crate) use info;
pub(crate) use warning;
pub(crate) use fatal;
