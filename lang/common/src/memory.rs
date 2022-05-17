use std::{
    hash::Hash,
    ops::{Deref, DerefMut},
};

/// An ironic object which is not safe at all
///
/// This object is useful for interior mutability.
pub struct SafePtr<T> {
    ptr: *mut T,
}

impl<T> Drop for SafePtr<T> {
    fn drop(&mut self) {}
}

unsafe impl<T> std::marker::Sync for SafePtr<T> {}

unsafe impl<T> std::marker::Send for SafePtr<T> {}

impl<T> Clone for SafePtr<T> {
    fn clone(&self) -> Self {
        Self { ptr: self.ptr }
    }
}

impl<T> Deref for SafePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for SafePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for SafePtr<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ptr.fmt(f)
    }
}

impl<T> std::borrow::Borrow<T> for SafePtr<T> {
    fn borrow(&self) -> &T {
        self.get()
    }
}

impl<T> SafePtr<T> {
    pub fn new(mut obj: T) -> Self {
        Self {
            ptr: std::ptr::addr_of_mut!(obj),
        }
    }

    pub fn from_ptr(ptr: *mut T) -> Self {
        Self { ptr }
    }

    pub fn get_ptr(&self) -> *mut T {
        self.ptr
    }

    pub fn get(&self) -> &T {
        unsafe {
            if self.ptr.is_null() {
                panic!("null pointer detected!");
            }
            match self.ptr.as_ref() {
                Some(v) => v,
                None => panic!("failed to read pointer"),
            }
        }
    }

    #[allow(clippy::mut_from_ref)]
    pub fn get_mut(&self) -> &mut T {
        unsafe {
            if self.ptr.is_null() {
                panic!("null pointer detected!");
            }
            match self.ptr.as_mut() {
                Some(v) => v,
                None => panic!("failed to read pointer"),
            }
        }
    }
}

impl<T> Hash for SafePtr<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ptr.hash(state)
    }
}

impl<T> AsRef<T> for SafePtr<T> {
    fn as_ref(&self) -> &T {
        self.get()
    }
}

impl<T> AsMut<T> for SafePtr<T> {
    fn as_mut(&mut self) -> &mut T {
        self.get_mut()
    }
}
