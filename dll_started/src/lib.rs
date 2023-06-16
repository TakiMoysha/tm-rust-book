#[no_mangle]
pub extern "C" fn foo() -> u32 {
    print!("hello world!");
    return 1;
}
