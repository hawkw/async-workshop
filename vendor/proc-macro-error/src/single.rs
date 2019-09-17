//! This module contains data types and functions to be used for single-error reporting.
//!
//! These are supposed to be used through [`span_error!`] and [`call_site_error!`],
//! see [crate level documentation](crate).

use crate::Payload;
use crate::ResultExt;
use proc_macro2::{Span, TokenStream};
use quote::quote_spanned;
use std::convert::{AsMut, AsRef};
use std::fmt::{Display, Formatter};

/// An single error in a proc-macro. This struct preserves
/// the given span so `rustc` can highlight the exact place in user code
/// responsible for the error.
///
/// You're not supposed to use this type directly, use [`span_error!`] and [`call_site_error!`].
#[derive(Debug)]
pub struct MacroError {
    pub(crate) span: Span,
    pub(crate) msg: String,
}

impl MacroError {
    /// Create an error with the span and message provided.
    pub fn new(span: Span, msg: String) -> Self {
        MacroError { span, msg }
    }

    /// A shortcut for `MacroError::new(Span::call_site(), message)`
    pub fn call_site(msg: String) -> Self {
        MacroError::new(Span::call_site(), msg)
    }

    /// Convert this error into a [`TokenStream`] containing these tokens: `compiler_error!(<message>);`.
    /// All these tokens carry the span this error contains attached.
    ///
    /// There are `From<[MacroError]> for proc_macro/proc_macro2::TokenStream` implementations
    /// so you can use `error.into()` instead of this method.
    ///
    /// [compl_err]: https://doc.rust-lang.org/std/macro.compile_error.html
    pub fn into_compile_error(self) -> TokenStream {
        let MacroError { span, msg } = self;
        quote_spanned! { span=> compile_error!(#msg); }
    }

    /// Abandon the old span and replace it with the given one.
    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }

    /// Get the span contained.
    pub fn span(&self) -> Span {
        self.span.clone()
    }

    /// Trigger single error, aborting the proc-macro's execution.
    ///
    /// You're not supposed to use this function directly.
    /// Use [`span_error!`] or [`call_site_error!`] instead.
    pub fn trigger(self) -> ! {
        panic!(Payload(self))
    }
}

impl From<syn::Error> for MacroError {
    fn from(e: syn::Error) -> Self {
        MacroError::new(e.span(), e.to_string())
    }
}

impl From<String> for MacroError {
    fn from(msg: String) -> Self {
        MacroError::call_site(msg)
    }
}

impl From<&str> for MacroError {
    fn from(msg: &str) -> Self {
        MacroError::call_site(msg.into())
    }
}

impl Display for MacroError {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        Display::fmt(&self.msg, f)
    }
}

impl<T, E: Into<MacroError>> ResultExt for Result<T, E> {
    type Ok = T;

    fn unwrap_or_exit(self) -> T {
        match self {
            Ok(res) => res,
            Err(e) => e.into().trigger(),
        }
    }

    fn expect_or_exit(self, message: &str) -> T {
        match self {
            Ok(res) => res,
            Err(e) => {
                let MacroError { msg, span } = e.into();
                let msg = format!("{}: {}", message, msg);
                MacroError::new(span, msg).trigger()
            }
        }
    }
}

impl From<MacroError> for TokenStream {
    fn from(err: MacroError) -> Self {
        err.into_compile_error()
    }
}

impl From<MacroError> for proc_macro::TokenStream {
    fn from(err: MacroError) -> Self {
        err.into_compile_error().into()
    }
}

impl AsRef<String> for MacroError {
    fn as_ref(&self) -> &String {
        &self.msg
    }
}

impl AsMut<String> for MacroError {
    fn as_mut(&mut self) -> &mut String {
        &mut self.msg
    }
}
