//! Совместимый слой для параллельных/последовательных итераций.

#[cfg(not(target_arch = "wasm32"))]
pub use rayon::prelude::*;

#[cfg(target_arch = "wasm32")]
mod wasm_prelude {
    pub trait SliceParIter {
        type Item;
        fn par_iter(&self) -> std::slice::Iter<'_, Self::Item>;
        fn par_iter_mut(&mut self) -> std::slice::IterMut<'_, Self::Item>;
    }

    impl<T> SliceParIter for [T] {
        type Item = T;
        fn par_iter(&self) -> std::slice::Iter<'_, T> { self.iter() }
        fn par_iter_mut(&mut self) -> std::slice::IterMut<'_, T> { self.iter_mut() }
    }

    pub trait IntoParIter {
        type Iter: Iterator;
        fn into_par_iter(self) -> Self::Iter;
    }

    impl<T> IntoParIter for Vec<T> {
        type Iter = std::vec::IntoIter<T>;
        fn into_par_iter(self) -> Self::Iter { self.into_iter() }
    }

    impl IntoParIter for std::ops::Range<usize> {
        type Iter = std::ops::Range<usize>;
        fn into_par_iter(self) -> Self::Iter { self }
    }
}

#[cfg(target_arch = "wasm32")]
pub use wasm_prelude::*;
