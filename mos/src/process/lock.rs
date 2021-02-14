//! 这个似乎是在 spin 基础上实现了一套 RAII，然后把 sstatus 的逻辑 hack 进去了. 在其基础上进一步关闭了中断。这是因为我们（以后）在内核线程中也有可能访问 PROCESSOR，但是此时我们不希望它被时钟打断，这样在中断处理中就无法访问 PROCESSOR 了，因为它已经被锁住。
//! 一个关闭中断的互斥锁 [`Lock`]

use spin::{Mutex, MutexGuard};

/// 关闭中断的互斥锁
#[derive(Default)]
pub struct Lock<T>(pub(self) Mutex<T>);

/// 封装 [`MutexGuard`] 来实现 drop 时恢复 sstatus
pub struct LockGuard<'a, T> {
    /// 在 drop 时需要先 drop 掉 [`MutexGuard`] 再恢复 sstatus
    guard: Option<MutexGuard<'a, T>>,
    /// 保存的关中断前 sstatus
    sstatus: usize,
}

impl<T> Lock<T> {
    /// 创建一个新对象
    pub fn new(obj: T) -> Self {
        Self(Mutex::new(obj))
    }

    /// 获得上锁的对象
    pub fn lock(&self) -> LockGuard<'_, T> {
        let sstatus: usize;
        unsafe {
            llvm_asm!("csrrci $0, sstatus, 1 << 1" : "=r"(sstatus) ::: "volatile");
        }
        LockGuard {
            guard: Some(self.0.lock()),
            sstatus,
        }
    }
}

/// 释放时，先释放内部的 MutexGuard，再恢复 sstatus 寄存器
impl<'a, T> Drop for LockGuard<'a, T> {
    fn drop(&mut self) {
        self.guard.take();
        unsafe { llvm_asm!("csrs sstatus, $0" :: "r"(self.sstatus & 2) :: "volatile") };
    }
}

impl<'a, T> core::ops::Deref for LockGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.guard.as_ref().unwrap().deref()
    }
}

impl<'a, T> core::ops::DerefMut for LockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.guard.as_mut().unwrap().deref_mut()
    }
}
