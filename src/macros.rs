//! Macros for Kernel-Mode drivers.

/// Find field offset in a struct
macro_rules! offset_of {
    ($ty:ty, $field:ident) => {
        //  Undefined Behavior: dereferences a null pointer.
        //  Undefined Behavior: accesses field outside of valid memory area.
        #[allow(unused_unsafe)]
        unsafe { &(*(0 as *const $ty)).$field as *const _ as usize }
    }
}

/// Macro to send a message to the kernel debugger.
///
/// # Example
///
/// ```no_run
/// KdPrint!("NTSTATUS is 0x%X\n", status);
/// ```
#[macro_export]
macro_rules! KdPrint {
	($msg:expr $(, $arg:expr)*) => { #[allow(unused_unsafe)] unsafe { $crate::debug::DbgPrint( concat!($msg, "\n\0").as_ptr() $(, $arg )* )} };
}

/// Macro to send a message to the kernel debugger for unsafe blocks.
///
/// Used in `unsafe {}` blocks.
#[macro_export]
macro_rules! KdPrint_u {
	($msg:expr $(, $arg:expr)*) => { $crate::debug::DbgPrint( concat!($msg, "\n\0").as_ptr() $(, $arg )* ) };
}

#[macro_export]
macro_rules! check_unsafe {
	($expr:expr) => {{
		let st: $crate::status::Status = unsafe { $expr };
		if st.is_err() {
			KdPrint!("[km] error: status 0x%X\n", st);
			return st;
		} else {
			st
		}
	}}
}

macro_rules! UNION {
    ($(#[$attrs:meta])* union $name:ident {
        [$stype:ty; $ssize:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])*
        pub struct $name([$stype; $ssize]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
    ($(#[$attrs:meta])* union $name:ident {
        [$stype32:ty; $ssize32:expr] [$stype64:ty; $ssize64:expr],
        $($variant:ident $variant_mut:ident: $ftype:ty,)+
    }) => (
        #[repr(C)] $(#[$attrs])* #[cfg(target_arch = "x86")]
        pub struct $name([$stype32; $ssize32]);
        #[repr(C)] $(#[$attrs])* #[cfg(target_pointer_width = "64")]
        pub struct $name([$stype64; $ssize64]);
        impl Copy for $name {}
        impl Clone for $name {
            #[inline]
            fn clone(&self) -> $name { *self }
        }
        #[cfg(feature = "impl-default")]
        impl Default for $name {
            #[inline]
            fn default() -> $name { unsafe { $crate::_core::mem::zeroed() } }
        }
        impl $name {$(
            #[inline]
            pub unsafe fn $variant(&self) -> &$ftype {
                &*(self as *const _ as *const $ftype)
            }
            #[inline]
            pub unsafe fn $variant_mut(&mut self) -> &mut $ftype {
                &mut *(self as *mut _ as *mut $ftype)
            }
        )+}
    );
}
