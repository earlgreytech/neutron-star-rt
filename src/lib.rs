#![no_std]
#![crate_type = "staticlib"]

extern "C"{
    /*
    For docs see https://neutron.earlgrey.tech/spec/neutron-arm-vm
    
    To actually test changes made here, do the following: 
    1. Run ./assemble.sh
    2. Run cargo build
    3. In neutron-host run ./hard_rebuild.sh (Hard rebuild seems to occationally be needed to properly re-link stuff)
    4. In neutron-host run cargo test
    5. Your changes are now in action and, if test passed, you haven't broken anything! 

    Please try to keep entries in same order here as in asm.s!
    */
    
    // Costack operators

    //SVC 0x10: push_costack (buffer: pointer, size: u32)
    pub fn __push_costack(buffer: *const u8, size: usize);
    
    //SVC 0x11: pop_costack (buffer: pointer, max_size: u32) -> actual_size: u32 -- note: if buffer and max_size is 0, then the item will be popped without copying the item to memory and only the actual_size will be returned
    pub fn __pop_costack(buffer: *mut u8, max_size: usize) -> usize;
    
    //SVC 0x14: clear_costack() -- Will clear the stack completely, without giving any information about what was held on the stack
    pub fn __clear_costack();
    
    //SVC 0x16: forward_input_costack() -- Overwriting copy of input stack onto output stack. Simplifies using this context's input as input to a following call
    pub fn __forward_input_costack();
    
    // Comap operators
    
    //SVC 0x30: push_comap(key: stack [u8], abi_data: u32, value: stack [u8])
    pub fn __push_comap(abi_data: u32);
    
    //SVC 0x31: push_raw_comap(key: stack [u8], raw_value: stack [u8])
    pub fn __push_raw_comap();
    
    //SVC 0x32: peek_comap(key: stack [u8], begin: u32, max_length: u32) -> (abi_data: u32, value: stack [u8])
    pub fn __peek_comap(begin: usize, max_size: usize) -> u32;
    
    //SVC 0x33: peek_raw_comap(key: stack [u8], begin: u32, max_length: u32) -> raw_value: stack [u8]
    pub fn __peek_raw_comap(begin: usize, max_size: usize);
    
    //SVC 0x34: peek_result_comap(key: stack [u8], begin: u32, max_length: u32) -> (abi_data: u32, value: stack [u8])
    pub fn __peek_result_comap(begin: usize, max_size: usize) -> u32;
    
    //SVC 0x35: peek_raw_result_comap(key: stack [u8], begin: u32, max_length: u32) -> raw_value: stack [u8]
    pub fn __peek_raw_result_comap(begin: usize, max_size: usize);
    
    // Context info operators
    
    //SVC 0x90: gas_remaining() -> limit: u64 (TODO: Investigate if this has to be split into two u32 or not)
    pub fn __gas_remaining() -> u64;
    
    //SVC 0x91: self_address() -> address: stack NeutronAddress
    pub fn __self_address();
    
    //SVC 0x92: origin() -> address: stack NeutronAddress
    pub fn __origin();
    
    //SVC 0x94: sender() -> address: stack NeutronAddress
    pub fn __sender();
    
    //SVC 0x96: execution_type() -> type: u32
    pub fn __execution_type() -> u32;

    // Misc operators

    //SVC 0xFF: exit_execution(status) -> noreturn
    pub fn __exit(code: u32) -> !;

    //SVC 0x20: system_call(feature, function):variable -> error:u32 -- will call into the NeutronCallSystem
    pub fn __system_call(element: u32, function: u32) -> u32;

    //Executes BKPT instruction. Has no effect outside of a debugging instance of Neutron
    pub fn __breakpoint();
}


