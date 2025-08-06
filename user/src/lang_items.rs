
#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let err = panic_info.message();
    if let Some(loc) = panic_info.location() {
        println!(
            "Paniced at {}:{} {}",
            loc.file(), loc.line(), err
        );
    } else {
        println!("Panic: {}", err);
    }
    loop{}
}

