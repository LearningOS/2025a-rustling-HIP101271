// No `extern` equals `extern "Rust"`.
#[no_mangle]
pub fn my_demo_function(a: u32) -> u32 {
    a
}
