#![allow(unused)]

const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_SHUTDOWN: usize = 8;

#[inline(always)]//https://qastack.cn/programming/37639276/when-should-inline-be-used-in-rust
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        //https://doc.rust-lang.org/stable/reference/inline-assembly.html
                 // inlateout(<reg>) <expr>/inlateout(<reg>) <in expr> => <out expr>
            // inout除了寄存器分配器可以重用分配给 an 的寄存器in（如果编译器知道 与 具有相同的初始值，就会发生这种情况in）inlateout。
            // 您应该只在读取所有输入后才写入寄存器，否则您可能会破坏输入。
                //in(<reg>) <expr>
            // <reg>可以引用寄存器类或显式寄存器。分配的寄存器名称被替换到 asm 模板字符串中。
            // 分配的寄存器将包含<expr>asm 代码开头的值。
            // 分配的寄存器必须在 asm 代码末尾包含相同的值（除非 alateout分配给同一寄存器）。
        core::arch::asm!(
            "li x16, 0",//用于加载立即值的伪指令
            "ecall",//(环境\系统调用）申请内核服务
            inlateout("x10") arg0 => ret,
            in("x11") arg1,
            in("x12") arg2,
            in("x17") which,
        );
    }
    ret
}

pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}
