##https://zhuanlan.zhihu.com/p/383729996
OUTPUT_ARCH(riscv)##指定一个特定的输出机器架构
ENTRY(_start) ##设置入口点
BASE_ADDRESS = 0x80200000;
##http://rcore-os.cn/rCore-Tutorial-Book-v3/chapter1/3first-instruction-in-kernel1.html#id6
SECTIONS ##描述输出文件的内存布局
{
    . = BASE_ADDRESS; ##位置计数器。如果没有以其他方式指定输出节的地址(其他方式将在后面描述)，地址将被设置为位置计数器的当前值
    skernel = .;

    stext = .;
    .text : { ##定义了一个输出节
        *(.text.entry) ##输入节的名称
        *(.text .text.*)
    }

    . = ALIGN(8K);##返回向上对齐到指定值的当前位置计数器  4k的空间
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(8K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(8K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(8K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
}

##已初始化数据段保存程序中那些已初始化的全局数据，分为 .rodata 和 .data 两部分。前者存放只读的全局数据，通常是一些常数或者是 常量字符串等；而后者存放可修改的全局数据。

##未初始化数据段 .bss 保存程序中那些未初始化的全局数据，通常由程序的加载者代为进行零初始化，即将这块区域逐字节清零；

##堆 （heap）区域用来存放程序运行时动态分配的数据，如 C/C++ 中的 malloc/new 分配到的数据本体就放在堆区域，它向高地址增长；

##栈 （stack）区域不仅用作函数调用上下文的保存与恢复，每个函数作用域内的局部变量也被编译器放在它的栈帧内，它向低地址增长。