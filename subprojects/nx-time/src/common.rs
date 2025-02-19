//! Common traits for the sys crate

// Code borrowed from: https://github.com/rust-lang/rust/blob/ed49386d3aa3a445a9889707fd405df01723eced/library/std/src/sys_common/mod.rs
// Licensed under: Apache-2.0 OR MIT

/// A trait for viewing representations from std types
#[allow(dead_code)]
pub trait AsInner<Inner: ?Sized> {
    fn as_inner(&self) -> &Inner;
}

/// A trait for viewing representations from std types
#[allow(dead_code)]
pub trait AsInnerMut<Inner: ?Sized> {
    fn as_inner_mut(&mut self) -> &mut Inner;
}

/// A trait for extracting representations from types
pub trait IntoInner<Inner> {
    fn into_inner(self) -> Inner;
}

/// A trait for creating types from internal representations
pub trait FromInner<Inner> {
    fn from_inner(inner: Inner) -> Self;
}
