

.global __boot

.section .text
.thumb_func
# .cfi_startproc
__boot:
  # Address of stack memory
  ldr r0, =0x81008000
  mov r13, r0
  # Hand execution over to `main`.
  bl main
  svc 0xFF
  # Note: `main` must not return. `bl` is used only because it has a wider range than `b`.
  # .cfi_endproc
  
# To actually test changes made here, do the following: 
# 1. Run ./assemble.sh
# 2. Run cargo build
# 3. In neutron-host run ./hard_rebuild.sh (Hard rebuild seems to occationally be needed to properly re-link stuff)
# 4. In neutron-host run cargo test
# 5. Your changes are now in action and, if test passed, you haven't broken anything! 
  
# Please try to keep entries in same order here as in lib.rs!

# Costack operators

.global __push_costack
.section .text
.thumb_func
__push_costack:
  svc 0x10
  mov pc, lr

.global __pop_costack
.section .text
.thumb_func
__pop_costack:
  svc 0x11
  mov pc, lr

.global __clear_costack
.section .text
.thumb_func
__clear_costack:
  svc 0x14
  mov pc, lr
  
.global __move_input_to_output_costack
.section .text
.thumb_func
__move_input_to_output_costack:
  svc 0x16
  mov pc, lr
  
# Comap operators
  
.global __push_comap
.section .text
.thumb_func
__push_comap:
  svc 0x30
  mov pc, lr
  
.global __push_raw_comap
.section .text
.thumb_func
__push_raw_comap:
  svc 0x31
  mov pc, lr
  
.global __peek_comap
.section .text
.thumb_func
__peek_comap:
  svc 0x32
  mov pc, lr
  
.global __peek_raw_comap
.section .text
.thumb_func
__peek_raw_comap:
  svc 0x33
  mov pc, lr
  
.global __peek_result_comap
.section .text
.thumb_func
__peek_result_comap:
  svc 0x34
  mov pc, lr
  
.global __peek_raw_result_comap
.section .text
.thumb_func
__peek_raw_result_comap:
  svc 0x35
  mov pc, lr
  
# Context info operators

.global __gas_remaining
.section .text
.thumb_func
__gas_remaining:
  svc 0x90
  mov pc, lr
  
.global __self_address
.section .text
.thumb_func
__self_address:
  svc 0x91
  mov pc, lr
  
.global __origin
.section .text
.thumb_func
__origin:
  svc 0x92
  mov pc, lr
  
.global __sender
.section .text
.thumb_func
__sender:
  svc 0x94
  mov pc, lr
  
.global __execution_type
.section .text
.thumb_func
__execution_type:
  svc 0x96
  mov pc, lr
  
# Misc operators
  
.global __exit
.section .text
.thumb_func
__exit:
    svc 0xFF
    # incase of some return from the SVC call, execute it again in an infintie loop
    b __exit 
    
.global __system_call
.section .text
.thumb_func
__system_call:
  svc 0x20
  mov pc, lr

.global __breakpoint
.section .text
.thumb_func
__breakpoint:
  bkpt
  mov pc, lr

