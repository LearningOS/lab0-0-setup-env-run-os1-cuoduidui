    .section .text.entry ##在声明了符号 _start 的地址放置在名为 .text.entry 的段中之后, 我们便可以指示编译器将这个段放到最低的地址上, 故这段指令能被最先执行.
    .globl _start
_start:
    la sp, boot_stack_top    ## 我们在 entry.asm 中分配启动栈空间，并在控制权被转交给 Rust 入口之前将栈指针 sp 设置为栈顶的位置。
    call rust_main
    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: