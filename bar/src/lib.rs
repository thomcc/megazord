#[no_mangle]
pub unsafe extern "C" fn bar_fn() -> () {
    println!("bar_fn");
}


#[cfg(feature = "export_fns")]
pub fn fns() -> std::vec::Vec<*const u8> {
    vec![
        &bar_fn as *const _ as *const u8,
    ]
}
