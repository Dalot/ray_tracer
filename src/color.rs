#[derive(Debug)]
pub struct RGB<T> {
    r: T,
    g: T,
    b: T,
}

/// 8-bit RGB
///
/// The colorspace is techincally undefined, but generally sRGB is assumed.
pub type RGB8 = RGB<u8>;

/// 16-bit RGB in machine's native endian
///
/// Be careful to perform byte-swapping when reading from files.
pub type RGB16 = RGB<u16>;

impl<T> RGB<T> {
    /// Convenience function for creating a new pixel
    /// The order of arguments is R,G,B
    #[inline(always)]
    pub const fn new(r: T, g: T, b: T) -> Self {
        Self { r, g, b }
    }
}

impl<T: PartialEq> PartialEq for RGB<T> {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }
}