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

    const fn push_u8_slice(&mut self, slice: &[u8]) {
        if self.bytes.len() - self.cursor < slice.len() {
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
        // SAFETY We know the slice is valid and casting to bytes should
        // always be valid, even if repr(rust) isn't stable yet.
        unsafe { std::mem::transmute(slice) }
    }
}

const FRAMES: &[Frame] = &[Frame::First(8), Frame::Second];
const NB_BYTES: usize = FRAMES.len() * std::mem::size_of::<Frame>();
const SERIALIZED_FRAMES: [u8; NB_BYTES] = {
    let mut buf = Buf::<NB_BYTES>::new();
    let bytes = Frame::cast_slice(FRAMES);
    buf.push_u8_slice(&bytes);
    buf.bytes
};
