use crate::memory::range::Range;
use alloc::sync::Arc;
use crate::memory::MemoryResult;
use crate::memory::PAGE_SIZE;
use crate::process::VirtualAddress;
use crate::memory::mapping::Flags;
use crate::memory::mapping::Segment;
use crate::memory::mapping::MapType;
use spin::Mutex;

use super::MemorySet;

/// 进程的信息
pub struct Process {
    /// 是否属于用户态
    pub is_user: bool,
    /// 用 `Mutex` 包装一些可变的变量
    pub inner: Mutex<ProcessInner>,
}

pub struct ProcessInner {
    /// 进程中的线程公用页表 / 内存映射
    /// Note(mwish): 一个进程对应一个映射，这个因为关联到更多内存，是需要可变的。
    pub memory_set: MemorySet,
//  /// 打开的文件描述符（实验五）
//  pub descriptors: Vec<Arc<dyn INode>>,
}

#[allow(unused)]
impl Process {
    /// 创建一个内核进程
    pub fn new_kernel() -> MemoryResult<Arc<Self>> {
        Ok(Arc::new(Self {
            is_user: false,
            inner: Mutex::new(ProcessInner {
                memory_set: MemorySet::new_kernel()?,
            }),
        }))
    }

    // /// 创建进程，从文件中读取代码
    // pub fn from_elf(file: &ElfFile, is_user: bool) -> MemoryResult<Arc<Self>> {
    //     Ok(Arc::new(Self {
    //         is_user,
    //         inner: Mutex::new(ProcessInner {
    //             memory_set: MemorySet::from_elf(file, is_user)?,
    //             descriptors: vec![STDIN.clone(), STDOUT.clone()],
    //         }),
    //     }))
    // }

    /// 上锁并获得可变部分的引用
    pub fn inner(&self) -> spin::MutexGuard<ProcessInner> {
        self.inner.lock()
    }

    /// 分配一定数量的连续虚拟空间
    ///
    /// 从 `memory_set` 中找到一段给定长度的未占用虚拟地址空间，分配物理页面并建立映射。返回对应的页面区间。
    ///
    /// `flags` 只需包括 rwx 权限，user 位会根据进程而定。
    pub fn alloc_page_range(
        &self,
        size: usize,
        flags: Flags,
    ) -> MemoryResult<Range<VirtualAddress>> {
        let memory_set = &mut self.inner().memory_set;

        // memory_set 只能按页分配，所以让 size 向上取整页
        let alloc_size = (size + PAGE_SIZE - 1) & !(PAGE_SIZE - 1);
        // 从 memory_set 中找一段不会发生重叠的空间
        let mut range = Range::<VirtualAddress>::from(VirtualAddress(0x1000000)..VirtualAddress(0x1000000 + alloc_size));
        while memory_set.overlap_with(range.into()) {
            range.start += alloc_size;
            range.end += alloc_size;
        }
        // 分配物理页面，建立映射
        memory_set.add_segment(
            Segment {
                map_type: MapType::Framed,
                range,
                flags: flags | Flags::user(self.is_user),
            },
            None,
        )?;
        // 返回地址区间（使用参数 size，而非向上取整的 alloc_size）
        Ok(Range::from(range.start..(range.start + size)))
    }
}
