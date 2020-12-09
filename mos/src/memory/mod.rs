mod address;
pub mod config;
pub mod frame;
pub mod heap;
pub mod mapping;
pub mod range;

pub use config::*;

/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;

/// 初始化内存相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    // 允许内核读写用户态内存
    unsafe { riscv::register::sstatus::set_sum() };

    println!("mod memory initialized");
}