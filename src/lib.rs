#![no_std]
#![crate_type = "staticlib"]

extern "C"{
    pub fn __exit(code: u32) -> !;
    pub fn __entropy(foo: u32) -> u32;
}


