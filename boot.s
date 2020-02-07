/* Constants for the multiboot header. */
.set ALIGN,   1<<0              /* Align loaded modules on page boundaries */
.set MEMINFO, 1<<1              /* Provide the memory map */
.set FLAGS,   ALIGN | MEMINFO   /* The multiboot "flag" field */
.set MAGIC,   0x1BADB002        /* "Magic number" that lets the bootloader find the header */
.set CHECKSUM, -(MAGIC + FLAGS) /* Checksum of the above, proves we are multiboot */

/*
We need to declare a multiboot header that marks this program as a kernel. The following values
are magic values documented in the multiboot standard. The bootloader will search for this
stuff in the first 8 KiB of the kernel file, aligned on a 32-bit boundary. The signature is in its
own section so the header can be forced to be within those first 8 KiB of the kernel file.
*/
.section .multiboot
.align 4
.long MAGIC
.long FLAGS
.long CHECKSUM

/*
Alright, so another (but bigger) info dump. The multiboot standard doesn't define the value of the
stack pointer register (esp) and it is up to the kernel to provide a stack. The following allocates
room for a small stack by creating a symbol at the bottom of it, then allocating 16384 bytes for it,
and finally creating a symbol at the top. The stack grows downwards (towards 0x0) on x86(_64). It's
in its own section so it can be marked nobits, which means the kernel file is smaller because it
doesn't contain an uninitialized stack. The stack on x86 must be 16-byte aligned, according to the
System V ABI standard and de-facto extensions. The compiler will assume the stack is properly
aligned, and failure to align the stack will result in undefined behaviour.
*/
.section .bss
.align 16
stack_bottom:
.skip 16384 # 16 KiB
stack_top:

/*
The linker script specifies _start as the entry point to the kernel, and the bootloader will jump to
__this__ position once the kernel has been loaded. It doesn't make sense to return from this
function as the bootloader is gone.
*/
.section .text
.global _start
.type _start, @function
_start:
    /*
    The bootloader has loaded us into 32-bit protected mode on an x86 machine. Interrupts are
    disabled. Paging is disabled. The processor state is as defined in the multiboot standard. The
    kernel can only make use of hardware features and any code it provides as part of itself.

    To set up a stack, we set the esp register to point to the top of the stack (it grows downwards
    on x86 systems). This must be done in Assembly, as langs like Rust can't function without a
    stack.
    */
    mov $stack_top, %esp

    /*
    MORE INFO
    This is a good place to initialize crucial processor state before the high-level kernel is
    entered. It's best to minimize the early environment where crucial features are offline. Note
    that the processor is not fully initialized yet; features such as floating point instructions
    and instruction set extensions are not initialized yet. The global descriptor table (GDT) should
    be loaded here. Paging should be enabled here. etc. etc.
    */
    /* Aforementioned things go here */
    /*
    And now we enter the kernel proper. The ABI requires 16-byte alignment for the stack at the time
    of the call instruction (which afterwards pushes te return pointer of size 4 bytes). The stack
    was originally 16-byte aligned above, and we've pushed a multiple of 16 bytes to the stack since
    (0), so the alignment has been preserved and the following call is well defined.
    */
    call kernel_main

    /*
    If the system has nothing more to do, put the computer into an infinite loop. To do that, we:
    1. Disable interrupts with `cli` (clear interrupt enable in eflags). They are already disabled
       by the bootloader, so this is not needed. However, you might later enable interrupts and
       return from `kernel_main` (but that doesn't really make sense).
    2. Wait for the next interrupt to arrive with `hlt` (halt instruction). Since they are disabled,
       this will lock up the computer.
    3. Jump to the `hlt` instruction if it ever wakes due to a non-maskable interrupt occurring or
       due to system management mode.
    */
    cli
1:  hlt
    jmp 1b

/*
Set the size of the _start symbol to the current location '.' minus its start. This is useful for
debugging or when we implement call tracing.
*/
.size _start, . - _start
