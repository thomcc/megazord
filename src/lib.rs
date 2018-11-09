// Note: happens with or without the `pub`s.
pub extern crate foo;
pub extern crate bar;
pub use foo::*;
pub use bar::*;

#[no_mangle]
pub unsafe extern "C" fn megazord_fn() -> () {
    println!("megazord_fn");
}

#[cfg(feature = "export_fns")]
#[no_mangle]
pub unsafe extern "C" fn _fns() -> std::vec::Vec<*const u8> {
    let mut fns = vec![
        &megazord_fn as *const _ as *const u8,
    ];
    fns.append(&mut foo::fns().clone());
    fns.append(&mut bar::fns().clone());

    fns
}
