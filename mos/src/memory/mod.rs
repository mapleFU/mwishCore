mod address;
pub mod config;
pub mod frame;
pub mod heap;
pub mod range;

pub use config::*;
pub use heap::init;

/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;
