#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub use wiresafe_derive::Wiresafe;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Message<T: Wiresafe> {
    pub content: T,
    crc: u32,
}

impl<T: __private::Wiresafe> __private::Wiresafe for Message<T> {
    fn check() {}
}

impl<T: Wiresafe> From<T> for Message<T> {
    fn from(content: T) -> Self {
        let crc = crc32fast::hash(as_bytes(&content));
        Message { content, crc }
    }
}

impl<'a, T: Wiresafe> From<&'a Message<T>> for &'a [u8] {
    fn from(value: &'a Message<T>) -> Self {
        as_bytes(value)
    }
}

impl<T: Wiresafe> Message<T> {
    #[cfg(feature = "std")]
    /// Attempts to read a message from the given reader, checking for validity using a CRC32
    /// checksum.
    pub fn read_from<R: std::io::Read>(mut reader: R) -> std::io::Result<Self> {
        let mut msg = core::mem::MaybeUninit::<Self>::uninit();

        let ptr = msg.as_mut_ptr() as *mut u8;
        let len = core::mem::size_of::<Self>();
        let buf = unsafe { core::slice::from_raw_parts_mut(ptr, len) };

        reader.read_exact(buf)?;

        let msg = unsafe { msg.assume_init() };

        let crc = crc32fast::hash(as_bytes(&msg.content));
        if crc == msg.crc {
            Ok(msg)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Checksum verification failed",
            ))
        }
    }

    #[cfg(feature = "std")]
    /// Attempts to write the message to the given writer.
    pub fn write_into<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(self.into())
    }

    /// Converts a [Message] to a byte lice.
    pub fn as_bytes(&self) -> &[u8] {
        as_bytes(self)
    }

    // TODO: get rid of the N when `generic_const_exprs` is stabilized in some form.
    /// Initialized a zeroed array with the same size and memory alignment as [Message].
    pub const fn uninit<const N: usize>() -> AlignedBytes<N, Self> {
        if N != core::mem::size_of::<Self>() {
            panic!("Requested size isn't equal to `size_of::<Self>()`");
        }
        AlignedBytes::zeroed()
    }

    /// Attempts to convert the given [AlignedBytes] without copying. Validity is checked using a
    /// CRC32 checksum.
    ///
    /// # Safety
    /// The bytes in the array must correspond to valid byte patterns for the fields of `T`.
    pub unsafe fn try_from_aligned<const N: usize>(
        bytes: &AlignedBytes<N, Self>,
    ) -> Result<&Self, Error> {
        // Skip size check since the only way to create `AlignedBytes` is through `Self::uninit`
        let ptr = bytes.as_ref().as_ptr() as *const Self;
        let msg = &*ptr;

        let crc = crc32fast::hash(as_bytes(&msg.content));
        if crc == msg.crc {
            Ok(msg)
        } else {
            Err(Error::Checksum)
        }
    }
}

impl<T: Wiresafe + Copy> Message<T> {
    // TODO: get rid of the Copy requirement when `generic_const_exprs` is stabilized in some form.
    /// Attempts to convert the given [AlignedBytes]. Validity is checked using a CRC32 checksum.
    ///
    /// # Safety
    /// The bytes in the array must correspond to valid byte patterns for the fields of `T`.
    pub unsafe fn try_from_aligned_copy<const N: usize>(
        bytes: AlignedBytes<N, Self>,
    ) -> Result<Self, Error> {
        // Skip size check since the only way to create `AlignedBytes` is through `Self::uninit`
        let ptr = bytes.as_ref().as_ptr() as *const Self;
        let msg = &*ptr;

        let crc = crc32fast::hash(as_bytes(&msg.content));
        if crc == msg.crc {
            Ok(*msg)
        } else {
            Err(Error::Checksum)
        }
    }
}

const fn as_bytes<T>(t: &T) -> &[u8] {
    let ptr = t as *const T as *const u8;
    unsafe { core::slice::from_raw_parts(ptr, core::mem::size_of::<T>()) }
}

/// Array of bytes that are force into `T`'s memory alignment.
pub struct AlignedBytes<const N: usize, T> {
    _align: [T; 0],
    value: [u8; N],
}

impl<const N: usize, T> AlignedBytes<N, T> {
    const fn zeroed() -> Self {
        Self {
            value: [0u8; N],
            _align: [],
        }
    }
}

impl<const N: usize, T> AsRef<[u8; N]> for AlignedBytes<N, T> {
    fn as_ref(&self) -> &[u8; N] {
        &self.value
    }
}

impl<const N: usize, T> AsMut<[u8; N]> for AlignedBytes<N, T> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.value
    }
}

/// Trait that marks a type as being safe to reinterpret cast to/from bytes. Mostly this means
/// Plain Old Data types.
pub trait Wiresafe: __private::Wiresafe + Sized {
    #[cfg(feature = "std")]
    /// Convenience method for calling [Message::read_from] and extracting the content.
    fn read_from<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Message::<Self>::read_from(reader).map(|msg| msg.content)
    }
}

impl<T: __private::Wiresafe> Wiresafe for T {}

#[derive(Debug)]
pub enum Error {
    Checksum,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Checksum => f.write_str("CRC checksums don't match, possible data corruption"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}

#[doc(hidden)]
#[rustfmt::skip]
pub mod __private {
    pub trait Wiresafe { fn check(); }

    impl Wiresafe for i8 { fn check() {} }
    impl Wiresafe for u8 { fn check() {} }
    impl Wiresafe for i16 { fn check() {} }
    impl Wiresafe for u16 { fn check() {} }
    impl Wiresafe for i32 { fn check() {} }
    impl Wiresafe for u32 { fn check() {} }
    impl Wiresafe for i64 { fn check() {} }
    impl Wiresafe for u64 { fn check() {} }
    impl Wiresafe for i128 { fn check() {} }
    impl Wiresafe for u128 { fn check() {} }
    impl Wiresafe for isize { fn check() {} }
    impl Wiresafe for usize { fn check() {} }
    impl Wiresafe for f32 { fn check() {} }
    impl Wiresafe for f64 { fn check() {} }
    impl Wiresafe for char { fn check() {} }
    impl Wiresafe for bool { fn check() {} }

    #[cfg(target_arch = "x86")] impl Wiresafe for std::arch::x86::__m128 { fn check() {} }
    #[cfg(target_arch = "x86")] impl Wiresafe for std::arch::x86::__m128d { fn check() {} }
    #[cfg(target_arch = "x86")] impl Wiresafe for std::arch::x86::__m128i { fn check() {} }
    #[cfg(target_arch = "x86")] impl Wiresafe for std::arch::x86::__m256 { fn check() {} }
    #[cfg(target_arch = "x86")] impl Wiresafe for std::arch::x86::__m256d { fn check() {} }
    #[cfg(target_arch = "x86")] impl Wiresafe for std::arch::x86::__m256i { fn check() {} }

    #[cfg(target_arch = "x86_64")] impl Wiresafe for std::arch::x86_64::__m128 { fn check() {} }
    #[cfg(target_arch = "x86_64")] impl Wiresafe for std::arch::x86_64::__m128d { fn check() {} }
    #[cfg(target_arch = "x86_64")] impl Wiresafe for std::arch::x86_64::__m128i { fn check() {} }
    #[cfg(target_arch = "x86_64")] impl Wiresafe for std::arch::x86_64::__m256 { fn check() {} }
    #[cfg(target_arch = "x86_64")] impl Wiresafe for std::arch::x86_64::__m256d { fn check() {} }
    #[cfg(target_arch = "x86_64")] impl Wiresafe for std::arch::x86_64::__m256i { fn check() {} }

    impl<T: Wiresafe, const N: usize> Wiresafe for [T; N] { fn check() {} }

    impl Wiresafe for () { fn check() {} }
    impl<T0: Wiresafe> Wiresafe for (T0,) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe> Wiresafe for (T0, T1) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe> Wiresafe for (T0, T1, T2) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe, T3: Wiresafe> Wiresafe for (T0, T1, T2, T3) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe, T3: Wiresafe, T4: Wiresafe> Wiresafe for (T0, T1, T2, T3, T4) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe, T3: Wiresafe, T4: Wiresafe, T5: Wiresafe> Wiresafe for (T0, T1, T2, T3, T4, T5) { fn check() {} }

    impl<T: Wiresafe> Wiresafe for core::marker::PhantomData<T> { fn check() {} }
    impl Wiresafe for core::marker::PhantomPinned { fn check() {} }

    impl<T: Wiresafe> Wiresafe for core::mem::ManuallyDrop<T> { fn check() {} }

    impl Wiresafe for core::num::NonZeroI8 { fn check() {} }
    impl Wiresafe for core::num::NonZeroU8 { fn check() {} }
    impl Wiresafe for core::num::NonZeroI16 { fn check() {} }
    impl Wiresafe for core::num::NonZeroU16 { fn check() {} }
    impl Wiresafe for core::num::NonZeroI32 { fn check() {} }
    impl Wiresafe for core::num::NonZeroU32 { fn check() {} }
    impl Wiresafe for core::num::NonZeroI64 { fn check() {} }
    impl Wiresafe for core::num::NonZeroU64 { fn check() {} }
    impl Wiresafe for core::num::NonZeroI128 { fn check() {} }
    impl Wiresafe for core::num::NonZeroU128 { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::num::Wrapping<T> { fn check() {} }
    impl Wiresafe for core::num::FpCategory { fn check() {} }

    impl<T: Wiresafe> Wiresafe for core::ops::Range<T> { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::ops::RangeFrom<T> { fn check() {} }
    impl Wiresafe for core::ops::RangeFull { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::ops::RangeInclusive<T> { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::ops::RangeTo<T> { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::ops::RangeToInclusive<T> { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::ops::Bound<T> { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::ops::ControlFlow<T> { fn check() {} }

    impl Wiresafe for core::time::Duration { fn check() {} }

    impl Wiresafe for core::cmp::Ordering { fn check() {} }

    impl<T: Wiresafe> Wiresafe for core::option::Option<T> { fn check() {} }
    impl<T: Wiresafe, E: Wiresafe> Wiresafe for core::result::Result<T, E> { fn check() {} }

    #[cfg(feature = "std")] impl Wiresafe for std::net::IpAddr { fn check() {} }
    #[cfg(feature = "std")] impl Wiresafe for std::net::Ipv4Addr { fn check() {} }
    #[cfg(feature = "std")] impl Wiresafe for std::net::Ipv6Addr { fn check() {} }
    #[cfg(feature = "std")] impl Wiresafe for std::net::Shutdown { fn check() {} }
    #[cfg(feature = "std")] impl Wiresafe for std::net::SocketAddr { fn check() {} }
    #[cfg(feature = "std")] impl Wiresafe for std::net::SocketAddrV4 { fn check() {} }
    #[cfg(feature = "std")] impl Wiresafe for std::net::SocketAddrV6 { fn check() {} }

    #[cfg(feature = "std")] impl<T: Wiresafe> Wiresafe for std::io::Cursor<T> { fn check() {} }

    #[cfg(feature = "arrayvec")] impl<T: Wiresafe, const N: usize> Wiresafe for arrayvec::ArrayVec<T, N> { fn check() {} }
    #[cfg(feature = "arrayvec")] impl<const N: usize> Wiresafe for arrayvec::ArrayString<N> { fn check() {} }
    // ...
}

#[cfg(test)]
mod tests {
    use core::borrow::Borrow;

    use super::*;

    #[repr(C, align(1))]
    #[derive(Debug, PartialEq, Eq)]
    struct Data {
        x: i32,
        y: u8,
        zero: Zero,
        z: u8,
    }

    impl __private::Wiresafe for Data {
        fn check() {}
    }

    #[derive(Debug, PartialEq, Eq)]
    struct Zero;

    impl __private::Wiresafe for Zero {
        fn check() {}
    }

    #[test]
    fn roundtrip_reader() {
        let data = Data {
            x: -5,
            y: 10,
            zero: Zero,
            z: 1,
        };

        let msg = Message::from(data);
        let bytes: &[u8] = msg.borrow().into();
        let msg2 = Message::<Data>::read_from(bytes).unwrap();

        assert_eq!(msg.content, msg2.content);
    }

    #[test]
    fn roundtrip_array() {
        let data = Data {
            x: -5,
            y: 10,
            zero: Zero,
            z: 1,
        };
        let msg = Message::from(data);

        let mut aligned = Message::<Data>::uninit::<12>();
        aligned.as_mut().copy_from_slice(msg.as_bytes());

        let msg2 = unsafe { Message::<Data>::try_from_aligned(&aligned).unwrap() };
        assert_eq!(msg, *msg2);
    }

    macro_rules! assert_aligned {
        ($ty:ty) => {
            assert_eq!(
                core::mem::align_of::<AlignedBytes<0, $ty>>(),
                core::mem::align_of::<$ty>()
            );
        };
    }

    #[test]
    #[allow(dead_code)]
    fn align() {
        /// 2-byte alignment
        #[repr(align(2))]
        struct A2;

        /// 4-byte alignment
        #[repr(align(4))]
        struct A4;

        /// 8-byte alignment
        #[repr(align(8))]
        struct A8;

        /// 16-byte alignment
        #[repr(align(16))]
        struct A16;

        /// 32-byte alignment
        #[repr(align(32))]
        struct A32;

        /// 64-byte alignment
        #[repr(align(64))]
        struct A64;

        assert_aligned!(Data);
        assert_aligned!(u8);
        assert_aligned!(A2);
        assert_aligned!(A4);
        assert_aligned!(A8);
        assert_aligned!(A16);
        assert_aligned!(A32);
        assert_aligned!(A64);
    }
}
