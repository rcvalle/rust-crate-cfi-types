//! CFI types for cross-language LLVM CFI support.
//!
//! The cfi_types crate provides a new set of C types as user-defined types
//! using the cfi_encoding attribute and repr(transparent) to be used for
//! cross-language LLVM CFI support. This new set of C types allows the Rust
//! compiler to identify and correctly encode C types in extern "C" function
//! types indirectly called across the FFI boundary when CFI is enabled.
//!
//! The use of these types are optional and are recommended for when enforcement
//! and explicitness of types used across the FFI boundary and no loss of
//! granularity for cross-language LLVM CFI are preferred.
//!
//! Alternatively, the `-Zsanitizer-cfi-normalize-integers` option may be used
//! with the Clang `-fsanitize-cfi-icall-experimental-normalize-integers` option
//! for cross-language LLVM CFI support.

#![feature(cfg_sanitizer_cfi)]
#![feature(cfi_encoding)]
#![allow(non_camel_case_types)]

/// CFI type equivalent to Rust's core::ffi::c_char type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_char(pub core::ffi::c_char);

/// CFI type equivalent to Rust's core::ffi::c_char type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "c"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_char(pub core::ffi::c_char);

/// CFI type equivalent to Rust's core::ffi::c_int type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_int(pub core::ffi::c_int);

/// CFI type equivalent to Rust's core::ffi::c_int type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "i"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_int(pub core::ffi::c_int);

/// CFI type equivalent to Rust's core::ffi::c_long type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_long(pub core::ffi::c_long);

/// CFI type equivalent to Rust's core::ffi::c_long type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "l"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_long(pub core::ffi::c_long);

/// CFI type equivalent to Rust's core::ffi::c_longlong type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_longlong(pub core::ffi::c_longlong);

/// CFI type equivalent to Rust's core::ffi::c_longlong type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "x"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_longlong(pub core::ffi::c_longlong);

/// CFI type equivalent to Rust's core::ffi::c_schar type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_schar(pub core::ffi::c_schar);

/// CFI type equivalent to Rust's core::ffi::c_schar type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "a"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_schar(pub core::ffi::c_schar);

/// CFI type equivalent to Rust's core::ffi::c_short type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_short(pub core::ffi::c_short);

/// CFI type equivalent to Rust's core::ffi::c_short type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "s"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_short(pub core::ffi::c_short);

/// CFI type equivalent to Rust's core::ffi::c_uchar type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_uchar(pub core::ffi::c_uchar);

/// CFI type equivalent to Rust's core::ffi::c_uchar type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "h"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_uchar(pub core::ffi::c_uchar);

/// CFI type equivalent to Rust's core::ffi::c_uint type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_uint(pub core::ffi::c_uint);

/// CFI type equivalent to Rust's core::ffi::c_uint type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "j"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_uint(pub core::ffi::c_uint);

/// CFI type equivalent to Rust's core::ffi::c_ulong type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_ulong(pub core::ffi::c_ulong);

/// CFI type equivalent to Rust's core::ffi::c_ulong type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "m"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_ulong(pub core::ffi::c_ulong);

/// CFI type equivalent to Rust's core::ffi::c_ulonglong type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_ulonglong(pub core::ffi::c_ulonglong);

/// CFI type equivalent to Rust's core::ffi::c_ulonglong type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "y"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_ulonglong(pub core::ffi::c_ulonglong);

/// CFI type equivalent to Rust's core::ffi::c_ushort type alias.
#[cfg(sanitizer_cfi_normalize_integers)]
#[allow(dead_code)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_ushort(pub core::ffi::c_ushort);

/// CFI type equivalent to Rust's core::ffi::c_ushort type alias.
#[cfg(not(sanitizer_cfi_normalize_integers))]
#[allow(dead_code)]
#[cfi_encoding = "t"]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct c_ushort(pub core::ffi::c_ushort);
