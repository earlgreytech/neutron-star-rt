

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

.global __exit
.section .text
.thumb_func
__exit:
    svc 0xFF
    # incase of some return from the SVC call, execute it again in an infintie loop
    b __exit 

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

.global __system_call
.section .text
.thumb_func
__system_call:
  svc 0x20
  mov pc, lr
  
.global __push_raw_output
.section .text
.thumb_func
__push_raw_output:
  svc 0x40
  mov pc, lr
  
.global __peek_raw_input
.section .text
.thumb_func
__peek_raw_input:
  svc 0x42
  mov pc, lr
  
.global __peek_raw_result
.section .text
.thumb_func
__peek_raw_result:
  svc 0x44
  mov pc, lr

.global __breakpoint
.section .text
.thumb_func
__breakpoint:
  bkpt
  mov pc, lr

