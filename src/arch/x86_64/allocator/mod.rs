// Copyright 2023 Quaterno LLC
//
// Author: Mansoor Ahmed Memon <mansoorahmed.one@gmail.com>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use alloc::alloc::Layout;

use spin::{Mutex, MutexGuard};

pub use bump::BumpAllocator;
pub use linked_list::LinkedListAllocator;
pub use pool::PoolAllocator;

use super::meta;

mod bump;
mod linked_list;
mod pool;

#[global_allocator]
pub static ALLOCATOR: Locked<PoolAllocator> = Locked::new(PoolAllocator::new());

pub struct Locked<A> {
    inner: Mutex<A>,
}

impl<A> Locked<A> {
    const fn new(inner: A) -> Self {
        Locked {
            inner: Mutex::new(inner),
        }
    }

    fn lock(&self) -> MutexGuard<A> { self.inner.lock() }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! { panic!("allocation failure: {:?}", layout) }

fn align_up(addr: usize, align: usize) -> usize { (addr + align - 1) & !(align - 1) }

pub fn init() -> Result<(), ()> {
    unsafe { ALLOCATOR.lock().init(meta::heap_begin() as usize, meta::heap_size() as usize) };

    Ok(())
}
