#[no_mangle]
pub unsafe extern "C" fn foo_fn() -> () {
    println!("foo_fn");
}

#[cfg(feature = "export_fns")]
pub fn fns() -> std::vec::Vec<*const u8> {
    vec![
        &foo_fn as *const _ as *const u8,
    ]
}
