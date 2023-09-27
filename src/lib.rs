#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");

#[no_mangle]
pub extern "C" fn add_from_rust(a : i32, b : i32) -> i32 {
    return a + b
}

#[no_mangle]
pub unsafe extern "C" fn gcd_two_nat(a : *mut lean_object, b : *mut lean_object) -> *mut lean_object {
    lean_nat_gcd(a, b)
}