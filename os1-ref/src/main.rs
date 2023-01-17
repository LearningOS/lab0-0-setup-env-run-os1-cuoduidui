#![no_std]
#![no_main]
#![feature(panic_info_message)]

use log::*;


#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

core::arch::global_asm!(include_str!("entry.asm"));

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

#[no_mangle]//no_mangle属性可用于任何项目以禁用标准符号名称重整。项目的符号将是项目名称的标识符。 需要通过宏将 rust_main 标记为 #[no_mangle] 以避免编译器对它的名字进行混淆，不然在链接的时候， entry.asm 将找不到 main.rs 提供的外部符号 rust_main 从而导致链接失败。
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    println!("Hello, world!");
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    error!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    panic!("Shutdown machine!");
}

// fn all_path(root_path: &String) -> Result<Vec<String>, Box<dyn Error>> {
//     let mut path_list = vec![String::from(root_path)];
//     let mut start_index = 0;
//     loop {
//         let list_len = path_list.len();
//         for index in start_index..path_list.len() {
//             let path = &path_list[index];
//             if metadata(path)?.is_dir() {
//                 for child_dir in read_dir(&path)? {
//                     path_list.push(String::from(child_dir?.path().as_os_str().to_str().expect("")));
//                 }
//             }
//         }
//         if list_len == start_index { break; }
//         start_index = list_len;
//     }
//     return Ok(path_list);
// } 
