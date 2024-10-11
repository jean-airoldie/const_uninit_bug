use std::mem;

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

    const fn capacity(&self) -> usize {
        self.bytes.len() - self.cursor
    }

    const fn push_u8_slice(&mut self, slice: &[u8]) {
        if self.capacity() < slice.len() {
            panic!("exceeded capacity");
        }

        let mut i = 0;
        while i < slice.len() {
            self.bytes[self.cursor + i] = slice[i];
            i += 1;
        }
        self.cursor += slice.len();
    }
}

#[repr(u8)]
enum Frame {
    First(u16),
    Second,
}

impl Frame {
    const fn cast_slice(slice: &[Frame]) -> &[u8] {
        // SAFETY We know the slice is valid, and the src and dst are
        // guranteed to have the same repr.
        unsafe { mem::transmute(slice) }
    }
}

struct Frames(&'static [Frame]);

impl Frames {
    const fn new(slice: &'static [Frame]) -> Self {
        Self(slice)
    }

    const fn serialize<const N: usize>(&self, buf: &mut Buf<N>) {
        let slice: &[u8] = Frame::cast_slice(self.0);
        buf.push_u8_slice(slice);
    }

    const fn serialized_size(&self) -> usize {
        mem::size_of::<u16>() + self.0.len() * mem::size_of::<Frame>()
    }
}

const FRAMES: Frames = Frames::new(&[Frame::First(8), Frame::Second]);
const SERIALIZED_FRAMES: [u8; FRAMES.serialized_size()] = {
    let mut buf = Buf::<{ FRAMES.serialized_size() }>::new();
    FRAMES.serialize(&mut buf);
    buf.bytes
};
