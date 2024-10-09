.set ALIGN,    1<<0             # align loaded modules on page boundaries
.set MEMINFO,  1<<1             # provide memory map
.set FLAGS,    ALIGN | MEMINFO
.set MAGIC,    0x1BADB002
.set CHECKSUM, -(MAGIC + FLAGS)

.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

.section .bss
.align 16
stack_bottom:
.skip 16384    # 16 KiB
stack_top:

.section .text
.global _start
.type _start, @function
_start:
    mov esp, OFFSET stack_top
    push eax
    push ebx
    call kernel_main
    cli
kernel_hlt:
    hlt
    jmp kernel_hlt
