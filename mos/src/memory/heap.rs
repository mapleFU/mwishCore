//! 虽然加上这段之后我们的代码就可以运行 `Vec` 之类的了，但是实际上这个在 `.bss` 段分配的内存，
//! 然后并不是用户用的 memory. 所以还需要给用户的 alloc 提供对应的接口。
//!
//! 至于为何堆在 .bss 字段，实际上这也不是必须的——我们完全可以随意指定一段可以访问的内存空间。
//! 不过，在代码中用全局变量来表示堆并将其放在 .bss 字段，是一个很简单的实现：
//! 这样堆空间就包含在内核的二进制数据之中了，而自 KERNEL_END_ADDRESS 以后的空间就都可以给进程使用。
use super::KERNEL_HEAP_SIZE;

use buddy_system_allocator::LockedHeap;

/// 进行动态内存分配所用的堆空间
///
/// 大小为 [`KERNEL_HEAP_SIZE`]  
/// 这段空间编译后会被放在操作系统执行程序的 bss 段
static mut HEAP_SPACE: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];

/// 堆，动态内存分配器
///
/// ### `#[global_allocator]`
/// [`LockedHeap`] 实现了 [`alloc::alloc::GlobalAlloc`] trait，
/// 可以为全局需要用到堆的地方分配空间。例如 `Box` `Arc` 等
#[global_allocator]
static HEAP: LockedHeap = LockedHeap::empty();

/// 初始化操作系统运行时堆空间
pub fn init() {
    // 告诉分配器使用这一段预留的空间作为堆
    unsafe {
        HEAP.lock()
            .init(HEAP_SPACE.as_ptr() as usize, KERNEL_HEAP_SIZE)
    }
}

/// 空间分配错误的回调，直接 panic 退出
#[alloc_error_handler]
fn alloc_error_handler(_: alloc::alloc::Layout) -> ! {
    panic!("alloc error")
}
