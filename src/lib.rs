#![no_std]

#[cfg(feature = "std")]
extern crate std;

pub use wiresafe_derive::Wiresafe;

pub trait Wiresafe: __private::Wiresafe + Sized {
    fn try_from_bytes(bytes: &[u8]) -> Result<&Self, Error> {
        read(bytes).and_then(try_as)
    }

    fn as_bytes(&self) -> &[u8] {
        as_bytes(self)
    }

    fn write_into<const N: usize>(&self, dst: &mut [u8; N]) -> usize {
        write(self.as_bytes(), dst)
    }
}

impl<T: __private::Wiresafe> Wiresafe for T {}

fn read(bytes: &[u8]) -> Result<&[u8], Error> {
    match bytes {
        [content @ .., crc] => {
            if crc_checksum(content) != *crc {
                return Err(Error::Checksum);
            }
            Ok(content)
        }
        _ => Err(Error::SizeMismatch {
            expected: 1,
            actual: bytes.len(),
        }),
    }
}

fn write<const N: usize>(src: &[u8], dst: &mut [u8; N]) -> usize {
    if N < src.len() + 1 {
        0
    } else {
        dst[..src.len()].copy_from_slice(src);
        dst[src.len()] = crc_checksum(src);
        src.len() + 1
    }
}

fn crc_checksum(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0, |x, acc| acc.wrapping_add(x))
}

fn as_bytes<T>(t: &T) -> &[u8] {
    let ptr = t as *const T as *const u8;
    unsafe { core::slice::from_raw_parts(ptr, core::mem::size_of::<T>()) }
}

fn try_as<'a, T>(bytes: &[u8]) -> Result<&'a T, Error> {
    if bytes.len() != core::mem::size_of::<T>() {
        return Err(Error::SizeMismatch {
            expected: core::mem::size_of::<T>(),
            actual: bytes.len(),
        });
    }

    let ptr: *const u8 = bytes.as_ptr();
    Ok(unsafe { &*(ptr as *const T) })
}

#[derive(Debug)]
pub enum Error {
    Checksum,
    SizeMismatch { expected: usize, actual: usize },
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Checksum => f.write_str("CRC checksums don't match, possible data corruption"),
            Error::SizeMismatch { expected, actual } => {
                write!(f, "Size mismatch, expected `{expected}`, got `{actual}`")
            }
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

    impl<T: Wiresafe, const N: usize> Wiresafe for [T; N] { fn check() {} }

    impl Wiresafe for () { fn check() {} }
    impl<T0: Wiresafe> Wiresafe for (T0,) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe> Wiresafe for (T0, T1) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe> Wiresafe for (T0, T1, T2) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe, T3: Wiresafe> Wiresafe for (T0, T1, T2, T3) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe, T3: Wiresafe, T4: Wiresafe> Wiresafe for (T0, T1, T2, T3, T4) { fn check() {} }
    impl<T0: Wiresafe, T1: Wiresafe, T2: Wiresafe, T3: Wiresafe, T4: Wiresafe, T5: Wiresafe> Wiresafe for (T0, T1, T2, T3, T4, T5) { fn check() {} }

    impl<T: Wiresafe> Wiresafe for core::cell::UnsafeCell<T> { fn check() {} }
    impl<T: Wiresafe> Wiresafe for core::cell::Cell<T> { fn check() {} }

    impl<T: Wiresafe> Wiresafe for core::marker::PhantomData<T> { fn check() {} }

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

    impl Wiresafe for core::sync::atomic::AtomicBool { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicI8 { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicU8 { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicI16 { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicU16 { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicI32 { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicU32 { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicI64 { fn check() {} }
    impl Wiresafe for core::sync::atomic::AtomicU64 { fn check() {} }
    impl Wiresafe for core::sync::atomic::Ordering { fn check() {} }

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
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Data {
        x: i32,
        y: u8,
        zero: Zero,
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
    fn roundtrip() {
        let data = Data {
            x: -5,
            y: 10,
            zero: Zero,
        };

        let mut buf = [0u8; 32];
        let written = data.write_into(&mut buf);
        let bytes = &buf[..written];

        let data2 = Data::try_from_bytes(bytes).unwrap();

        assert_eq!(data, *data2);
    }
}
