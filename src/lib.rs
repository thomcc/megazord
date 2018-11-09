extern crate foo;
extern crate bar;

#[no_mangle]
pub unsafe extern "C" fn megazord_fn() -> () {
    println!("megazord_fn");
}
