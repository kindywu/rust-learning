#![allow(unused)]

use std::{marker::PhantomPinned, ptr::NonNull};
fn main() {}

pub struct InlineBuf {
    data: [u8; 64],
    slice: NonNull<[u8]>,
    // _pinned: PhantomPinned,
}

impl InlineBuf {
    pub fn new() -> Self {
        Self {
            data: [0; 64],
            slice: NonNull::from(&[]),
        }
    }

    pub fn set_content(&mut self, buf: &[u8]) -> bool {
        let buf_len = buf.len();
        if buf_len > self.data.len() {
            return false;
        }

        self.data[0..buf_len].copy_from_slice(buf);
        self.slice = NonNull::from(&self.data[0..buf_len]);

        true
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe { &*self.slice.as_ptr() }
    }
}

impl Default for InlineBuf {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Default)]
struct AddrTracker(usize);

impl AddrTracker {
    pub fn show_address(&self) {
        println!("{:?}", self as *const _)
    }
}

fn take_addr_tracker(tracker: AddrTracker) {
    tracker.show_address()
}

#[cfg(test)]
mod tests {
    use super::{take_addr_tracker, AddrTracker, InlineBuf};

    #[test]
    fn test_addr_tracker() {
        let tracker = AddrTracker::default();
        tracker.show_address();
        take_addr_tracker(tracker)
    }

    #[test]
    fn test_inline_buf() {
        let mut buf = InlineBuf::new();

        {
            let mut tmp_buf = InlineBuf::new();
            tmp_buf.set_content(b"hello");

            buf = tmp_buf;
            println!("{:?}", buf.as_bytes());

            tmp_buf = InlineBuf::new();
            println!("{:?}", buf.as_bytes());

            tmp_buf.set_content(b"world");
            println!("{:?}", buf.as_bytes());
        }

        assert_eq!(buf.as_bytes(), b"hello")
    }
}
