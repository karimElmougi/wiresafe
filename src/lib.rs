pub use wiresafe_derive::Wiresafe;

pub trait Wiresafe: __private::Wiresafe {}

impl<T: __private::Wiresafe> Wiresafe for T {}

// #[derive(Wiresafe)]
// struct MyStruct<T> {
//     foo: i32,
//     bar: (UnitStruct,),
//     t: T,
// }
//
// #[derive(Wiresafe)]
// struct TupleStruct<T>(T);
//
// #[derive(Wiresafe)]
// struct UnitStruct;
//
// #[derive(Wiresafe)]
// enum Enum {
//     Unit,
//     Tuple(i32),
//     Struct{x: i32}
// }
// //
// #[derive(Wiresafe)]
// union Union {
//     int: i32,
//     float: f32,
//     arr: [u8; 8],
// }

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
    // ...
}
