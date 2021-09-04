// SPDX-License-Identifier: Apache-2.0

use core::mem::{align_of, align_of_val, size_of, size_of_val};

/// A single page of memory
///
/// This type is page-aligned and page-sized.
#[derive(Copy, Clone)]
#[repr(C, align(4096))]
pub struct Page([[u64; 32]; 16]);

#[cfg(feature = "const-default")]
impl const_default::ConstDefault for Page {
    const DEFAULT: Self = Self([[0; 32]; 16]);
}

impl Default for Page {
    fn default() -> Self {
        Self([[0; 32]; 16])
    }
}

impl AsRef<[u8]> for Page {
    fn as_ref(&self) -> &[u8] {
        unsafe { self.0.align_to().1 }
    }
}

impl AsMut<[u8]> for Page {
    fn as_mut(&mut self) -> &mut [u8] {
        unsafe { self.0.align_to_mut().1 }
    }
}

impl Page {
    /// Returns the size of the page in bytes
    pub const fn size() -> usize {
        core::mem::size_of::<Self>()
    }

    /// Returns a Page full of zeroes
    pub const fn zeroed() -> Self {
        Self([[0; 32]; 16])
    }

    /// Copy a value into the start of a page
    ///
    /// All unused bytes are zero.
    ///
    /// # Panics
    ///
    /// This function panics if any of these constraints are false:
    ///   1. `size_of::<Page>() >= size_of_val(&value)`
    ///   2. `align_of::<Page>() >= align_of_val(&value)`
    ///   3. `align_of::<Page>() % align_of_val(&value) == 0`
    pub fn copy<T: Copy>(value: T) -> Page {
        assert!(size_of::<Page>() >= size_of_val(&value));
        assert!(align_of::<Page>() >= align_of_val(&value));
        assert!(align_of::<Page>() % align_of_val(&value) == 0);

        let mut pages = [Page::default()];
        let bytes = unsafe { pages.align_to_mut::<u8>().1 };
        let typed = unsafe { bytes.align_to_mut().1 };
        typed[0] = value;
        pages[0]
    }
}
