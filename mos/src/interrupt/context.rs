use core::mem::zeroed;
use riscv::register::sstatus::Sstatus;

#[repr(C)]
#[derive(Debug)]
pub struct Context {
    pub x: [usize; 32], // 32 个通用寄存器
    pub sstatus: Sstatus,
    pub sepc: usize,
}

/// 创建一个用 0 初始化的 Context
///
/// 这里使用 [`core::mem::zeroed()`] 来强行用全 0 初始化。
/// 因为在一些类型中，0 数值可能不合法（例如引用），所以 [`zeroed()`] 是 unsafe 的
impl Default for Context {
    fn default() -> Self {
        unsafe { zeroed() }
    }
}
