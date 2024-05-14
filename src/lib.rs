use num::Num;
use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut, Index};
use std::ptr::NonNull;

#[derive(Debug)]
pub enum Storage {
    Full,
    Packed,
}

pub struct RadialArray<T: Num> {
    ptr: *mut T,
    len_total: usize,
    len_sites: usize,
    ngrid: usize,
    ns1: usize,
    ns2: usize,
    storage: Storage,
}

impl<T: Num> RadialArray<T> {
    pub fn new(ngrid: usize, ns1: usize, ns2: usize, storage: Storage) -> Self {
        assert!(mem::size_of::<T>() != 0, "no zero-sized types");

        let nsites = match storage {
            Storage::Full => ns1 * ns2,
            Storage::Packed => {
                assert!(
                    ns1 == ns2,
                    "can only store symmetric matrices in packed storage"
                );
                ns1 * (ns2 + 1) / 2
            }
        };

        let new_layout = Layout::array::<T>(ngrid * nsites).unwrap();

        let new_mem = { unsafe { alloc::alloc(new_layout) } };

        let new_ptr = new_mem as *mut T;
        if new_ptr.is_null() {
            alloc::handle_alloc_error(new_layout);
        }

        for i in 0..(ngrid * nsites) {
            unsafe {
                std::ptr::write(new_ptr.add(i), T::zero());
            }
        }

        RadialArray {
            ptr: new_ptr,
            len_total: ngrid * nsites,
            len_sites: nsites,
            ngrid,
            ns1,
            ns2,
            storage,
        }
    }

    pub fn write_to_1d_idx(&mut self, value: T, idx: usize) {
        unsafe { std::ptr::write(self.ptr.add(idx), value) }
    }

    pub fn write_to_idx(&mut self, value: T, idx: (usize, usize, usize)) {
        let (i, j, k) = idx;
        let ijk = match &self.storage {
            Storage::Packed => {
                let jk = if j > k {
                    j * (j + 1) / 2 + k
                } else {
                    k + (j + 1) / 2 + j
                };
                i * self.ngrid + jk
            }
            Storage::Full => j * self.ns1 + i * self.ngrid + k,
        };

        println!("{}", ijk);

        unsafe { std::ptr::write(self.ptr.add(ijk), value) }
    }
}

impl<T: Num> Deref for RadialArray<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr, self.len_total) }
    }
}

impl<T: Num> DerefMut for RadialArray<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len_total) }
    }
}

impl<T: std::fmt::Debug + Num> std::fmt::Debug for RadialArray<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(
            format!(
                "{:?}, len_grid: {}, len_sites: {}, len_total: {}",
                &**self, self.ngrid, self.len_sites, self.len_total
            )
            .as_str(),
            f,
        )
    }
}

// impl<T: Num> Index<(usize, usize, usize)> for RadialArray<T> {
//     type Output = T;
//
//     fn index(&self, idx: (usize, usize, usize)) -> &Self::Output {
//         let (i, j, k) = idx;
//         let jk = if j > k {
//             j * (j + 1) / 2 + k
//         } else {
//             k + (j + 1) / 2 + j
//         };
//         let ijk = i * self.len_grid + jk;
//         unsafe {
//             let result = self.ptr.add(ijk).read();
//             &result
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_storage_init() {
        let array: RadialArray<f64> = RadialArray::new(3, 2, 2, Storage::Full);
        println!("{:?}", array)
    }

    #[test]
    fn packed_storage_init() {
        let array: RadialArray<f64> = RadialArray::new(3, 2, 2, Storage::Packed);
        println!("{:?}", array)
    }

    #[test]
    fn packed_storage_write() {
        let mut array: RadialArray<f64> = RadialArray::new(3, 2, 2, Storage::Packed);
        array.write_to_idx(1000.0, (2, 0, 1));
        println!("{:?}", array)
    }

    #[test]
    fn full_storage_write() {
        let mut array: RadialArray<f64> = RadialArray::new(3, 2, 2, Storage::Full);
        array.write_to_idx(1000.0, (0, 1, 1));
        println!("{:?}", array)
    }
}
