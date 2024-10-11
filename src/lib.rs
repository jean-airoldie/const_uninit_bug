#![feature(const_ptr_write)]

use static_assertions::assert_eq_size;

struct Buf<const N: usize> {
    bytes: [u8; N],
    cursor: usize,
}

impl<const N: usize> Buf<N> {
    const fn new() -> Self {
        Self {
            bytes: [0u8; N],
            cursor: 0,
        }
    }

    const fn push_frame_slice(&mut self, slice: &[Frame]) {
        if self.bytes.len() - self.cursor < slice.len() {
            panic!("exceeded capacity");
        }

        let mut i = 0;
        while i < slice.len() {
            let ptr = unsafe { self.bytes.as_mut_ptr().add(self.cursor) as *mut Frame };
            unsafe { ptr.write(slice[i]) };
            self.cursor += std::mem::size_of::<Frame>();
            i += 1;
        }
        self.cursor += slice.len();
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
enum Frame {
    First(u16),
    Second,
}

const FRAMES: &[Frame] = &[Frame::First(8), Frame::Second];
const NB_BYTES: usize = FRAMES.len() * std::mem::size_of::<Frame>();
const SERIALIZED_FRAMES: [u8; NB_BYTES] = {
    let mut buf = Buf::<NB_BYTES>::new();
    buf.push_frame_slice(FRAMES);
    buf.bytes
};
