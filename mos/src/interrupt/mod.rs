//! RISC-V 提供了一些 csr, 以供处理异常：
//! stvec 它保存发生异常时处理器需要跳转到的地址
//! scause 它指示发生异常的种类。
//! sepc 指向发生异常的指令
//! sie 它指出处理器目前能处理和必须忽略的中断。
//! sip 它列出目前正准备处理的中断。
//! stval 它保存了陷入(trap)的附加信息:地址例外中出错的地址、发生非法指令例外的指令本身，对于其他异常，它的值为 0。
//! scratch 它暂时存放一个字大小的数据。(好像实现线程的地方可以用到)。
//! sstatus 它保存全局中断使能，以及许多其他的状态
mod context;
mod handler;
mod timer;

/// 初始化中断相关的子模块
///
/// - [`handler::init`]
/// - [`timer::init`]
pub fn init() {
    handler::init();
    timer::init();
    println!("mod interrupt initialized");
}
