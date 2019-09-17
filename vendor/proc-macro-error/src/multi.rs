//! **This module is deprecated! Come back in v0.3.**
#![doc(hidden)]

use crate::ResultExt;
use crate::{MacroError, Payload};
use proc_macro2::{Span, TokenStream};
use std::iter::FromIterator;
use std::ops::{Deref, DerefMut};

/// This type represents a container for multiple errors. Each error has it's own span
/// location.
pub struct MultiMacroErrors(Vec<MacroError>);

impl MultiMacroErrors {
    /// Create an empty errors storage.
    pub fn new() -> Self {
        MultiMacroErrors(Vec::new())
    }

    /// Test if the storage contains no errors.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Add an error to the list of errors.
    pub fn add(&mut self, err: MacroError) {
        self.0.push(err)
    }

    /// Shortcut for `.add(MacroError::new(span, msg))`
    pub fn add_span_msg(&mut self, span: Span, msg: String) {
        self.add(MacroError::new(span, msg))
    }

    /// Convert this storage into [`TokenStream`] consisting of `compile_error!(msg1); compile_error!(msg2);`...
    /// sequence. Each `compile_error!` invocation gets the span attached to the particular message.
    pub fn into_compile_errors(self) -> TokenStream {
        let errs = self.0.into_iter().flat_map(MacroError::into_compile_error);
        TokenStream::from_iter(errs)
    }

    /// Abort the proc-macro's execution and display the errors contained.
    ///
    /// ### Note:
    ///
    /// This function aborts even no errors are present! It's very much possibly that
    /// [`trigger_on_error`][MultiMacroErrors::trigger_on_error] is what you really want.
    pub fn trigger(self) -> ! {
        panic!(Payload(self))
    }

    /// If this storage is not empty abort the proc-macro's execution and display the errors contained.
    pub fn trigger_on_error(self) {
        if !self.is_empty() {
            self.trigger()
        }
    }
}

impl<T> ResultExt for Result<T, MultiMacroErrors> {
    type Ok = T;

    fn unwrap_or_exit(self) -> T {
        match self {
            Ok(res) => res,
            Err(e) => e.trigger(),
        }
    }

    fn expect_or_exit(self, message: &str) -> T {
        match self {
            Ok(res) => res,
            Err(mut e) => {
                for MacroError { msg, .. } in e.0.iter_mut() {
                    *msg = format!("{}: {}", message, msg);
                }
                e.trigger()
            }
        }
    }
}

impl From<MultiMacroErrors> for TokenStream {
    fn from(errors: MultiMacroErrors) -> Self {
        errors.into_compile_errors()
    }
}

impl From<MultiMacroErrors> for proc_macro::TokenStream {
    fn from(errors: MultiMacroErrors) -> Self {
        errors.into_compile_errors().into()
    }
}

impl FromIterator<MacroError> for MultiMacroErrors {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = MacroError>,
    {
        MultiMacroErrors(Vec::from_iter(iter))
    }
}

impl IntoIterator for MultiMacroErrors {
    type Item = MacroError;
    type IntoIter = <Vec<MacroError> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for MultiMacroErrors {
    type Target = [MacroError];

    fn deref(&self) -> &[MacroError] {
        &self.0
    }
}

impl DerefMut for MultiMacroErrors {
    fn deref_mut(&mut self) -> &mut [MacroError] {
        &mut self.0
    }
}
