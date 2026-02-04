use crate::std::{
    alloc::Layout,
    marker::PhantomData,
    num::NonZeroU32,
    ptr::{self, NonNull},
};
use crate::sys::coreinit::mem;

/// 32 MiB 1T SRAM (Graphics buffers)
pub struct MEM1;

/// 2 GiB DDR3 (General memory)
///
/// Prefer to use the normal global allocator where possible.
pub struct MEM2;

/// 40 MiB foreground bucket (only available if application in foreground)
pub struct FG;

pub trait Allocator {
    const MEM: mem::Arena;

    fn base_handle() -> Result<NonNull<mem::HeapHeader>, ()> {
        let handle = unsafe { mem::get_base_handle(Self::MEM) };

        match NonNull::new(handle) {
            None => Err(()),
            Some(handle) => Ok(handle),
        }
    }

    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, ()> {
        let handle = Self::base_handle()?;

        let size = layout.size() as u32;
        let align = layout.align() as i32;

        let ptr = unsafe {
            match handle.as_ref().tag {
                mem::HeapTag::ExpandedHeap => mem::alloc_exp_heap(handle.as_ptr(), size, align),
                mem::HeapTag::FrameHeap => mem::alloc_frm_heap(handle.as_ptr(), size, align),
                _ => return Err(()),
            }
        };

        let ptr = ptr::slice_from_raw_parts_mut(ptr as *mut u8, layout.size());

        match NonNull::new(ptr) {
            None => Err(()),
            Some(ptr) => Ok(ptr),
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>) {
        let handle = match Self::base_handle() {
            Ok(handle) => handle,
            Err(_) => return,
        };

        unsafe {
            match handle.as_ref().tag {
                mem::HeapTag::ExpandedHeap => {
                    mem::free_exp_heap(handle.as_ptr(), ptr.as_ptr().cast())
                }
                _ => (),
            }
        }

        todo!()
    }
}

impl Allocator for MEM1 {
    const MEM: mem::Arena = mem::Arena::Mem1;
}

impl Allocator for MEM2 {
    const MEM: mem::Arena = mem::Arena::Mem2;
}

impl Allocator for FG {
    const MEM: mem::Arena = mem::Arena::Foreground;
}

#[derive(Debug)]
pub struct Snapshot<MEM: Allocator> {
    tag: NonZeroU32,
    _marker: PhantomData<MEM>,
}

/// Records the current memory use state of a frame heap. It can be restored to the recorded state by calling [restore].
///
/// Not restoring the snapshot at a later point does nothing (expect using up some memory on the heap).
#[must_use]
pub fn record<MEM: Allocator>(tag: NonZeroU32) -> Result<Snapshot<MEM>, ()> {
    let handle = MEM::base_handle()?;

    let success = unsafe { mem::record_state_frm_heap(handle.as_ptr(), tag.get()) } != 0;

    match success {
        false => Err(()),
        true => Ok(Snapshot {
            tag,
            _marker: PhantomData,
        }),
    }
}

/// Restores the memory use state of a frame heap to a previous state. The state must have been previously recorded by [record].
///
/// # Safety
///
/// This function breaks Rusts memory safety in many ways. It can easily create dangling references / pointers, leading to undefined behavior.
///
/// The recommended general usecase is to call [record] and  [restore] either
/// * at the beginning and end of a scope
/// * in the constructor and destructor of an object
///
pub unsafe fn restore<MEM: Allocator>(snapshot: Snapshot<MEM>) -> Result<(), ()> {
    let handle = MEM::base_handle()?;

    let success = unsafe { mem::free_state_frm_heap(handle.as_ptr(), snapshot.tag.get()) } != 0;

    match success {
        false => Err(()),
        true => Ok(()),
    }
}
