use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};

pub fn console_putchar(c: usize) {
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c);
}


pub fn console_getchar() -> usize {
    #[allow(deprecated)]
    return sbi_rt::legacy::console_getchar();
}

pub fn shutdown(failure: bool) -> !{
    if !failure {
        system_reset(Shutdown, NoReason);
    }else{
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
