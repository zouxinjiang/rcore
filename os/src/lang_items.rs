use core::panic::PanicInfo;
use crate::sbi::shutdown;
use crate::{println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(loc) = info.location(){
        println!("panic at {}:{} {}", loc.file(), loc.line(), info.message());
    } else {
        println!("panic message: {}", info.message());
    }
    shutdown(true)
}


