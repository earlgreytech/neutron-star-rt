#![no_std]
#![crate_type = "staticlib"]

extern "C"{
    //SVC 0xFF: exit_execution(status) -> noreturn
    pub fn __exit(code: u32) -> !;

    //SVC 0x10: push_costack (buffer: pointer, size: u32)
    pub fn __push_costack(buffer: *const u8, size: usize);
    //SVC 0x11: pop_costack (buffer: pointer, max_size: u32) -> actual_size: u32 -- note: if buffer and max_size is 0, then the item will be popped without copying the item to memory and only the actual_size will be returned
    pub fn __pop_costack(buffer: *mut u8, max_size: usize) -> usize;
    //SVC 0x14: clear_costack() -- Will clear the stack completely, without giving any information about what was held on the stack
    pub fn __clear_costack();
    
    //SVC 0x40: push_raw_output(key: stack, data: stack)
    pub fn __push_raw_output();
    
    //SVC 0x42: peek_raw_input(key: stack, max_size: u32) -> data: stack [u8]
    pub fn __peek_raw_input(max_size: usize);
    
    //SVC 0x44: peek_raw_result(key: stack, max_size: u32) -> data: stack [u8]
    pub fn __peek_raw_result(max_size: usize);

    //SVC 0x20: system_call(feature, function):variable -> error:u32 -- will call into the NeutronCallSystem
    pub fn __system_call(element: u32, function: u32) -> u32;

    //Executes BKPT instruction. Has no effect outside of a debugging instance of Neutron
    pub fn __breakpoint();
}


