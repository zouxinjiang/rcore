use core::panic::PanicInfo;
use crate::sbi::shutdown;
use log::*;
//use crate::{println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location(){
        error!("[kernel] Paniced at {}:{} {}",
            location.file(),
            location.line(),
            info.message()
        );
    } else {
        error!("[kernel] Paniced: {}", info.message());
    }
    shutdown(true)
}


