use num::Num;
use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

#[derive(Debug)]
pub enum Storage {
    Full,
    Packed,
}

#[derive(Debug)]
pub struct RadialArray<T> {
    ptr: NonNull<T>,
    len_total: usize,
    len_sites: usize,
    len_grid: usize,
    storage: Storage,
}

impl<T> RadialArray<T> {
    pub fn new(ngrid: usize, ns1: usize, ns2: usize, storage: Storage) -> Self {
        assert!(mem::size_of::<T>() != 0, "no zero-sized types");

        let nsites = match storage {
            Storage::Full => ns1 * ns2,
            Storage::Packed => {
                assert!(
                    ns1 == ns2,
                    "can only store symmetric matrices in packed storage"
                );
                ns1 * (ns2 - 1) / 2
            }
        };

        let new_layout = Layout::array::<T>(ngrid * nsites).unwrap();

        let new_mem = { unsafe { alloc::alloc(new_layout) } };

        let new_ptr = match NonNull::new(new_mem as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        RadialArray {
            ptr: new_ptr,
            len_total: ngrid * nsites,
            len_sites: nsites,
            len_grid: ngrid,
            storage,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_storage_init() {
        let array: RadialArray<f64> = RadialArray::new(100, 10, 10, Storage::Full);
        println!("{:?}", array)
    }

    #[test]
    fn packed_storage_init() {
        let array: RadialArray<f64> = RadialArray::new(100, 10, 10, Storage::Packed);
        println!("{:?}", array)
    }
}
