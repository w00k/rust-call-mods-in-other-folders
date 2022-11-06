use crate::pass_core;

pub fn rest_caller() {
    println!("rest_caller() in rest_api/rest.rs called!");
    pass_core::create::create_func();
}