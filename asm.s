

.global __boot

.section .text
.thumb_func
# .cfi_startproc
__boot:
  # Address of stack memory
  ldr r0, =0x81000000
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

.global __entropy
.section .text
.thumb_func
__entropy:
    svc 0x00