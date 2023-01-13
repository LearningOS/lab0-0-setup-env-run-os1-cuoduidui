##https://zhuanlan.zhihu.com/p/383729996
OUTPUT_ARCH(riscv)##指定一个特定的输出机器架构
ENTRY(_start) ##设置入口点
BASE_ADDRESS = 0x80200000;
//http://rcore-os.cn/rCore-Tutorial-Book-v3/chapter1/3first-instruction-in-kernel1.html#id6
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