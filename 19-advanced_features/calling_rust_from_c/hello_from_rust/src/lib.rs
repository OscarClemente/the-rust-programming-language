#[no_mangle]
pub extern "C" fn hello_from_rust() -> u32{
    let var = 17534;
    println!("Hello from Rust! I have a variable with value {}.", var);
    var
}