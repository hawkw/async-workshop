//! `compile_error!` does not interrupt compilation right away. This means
//! `rustc` doesn't just show you the error and abort, it carries on the
//! compilation process, looking for other errors to report.
//!
//! Let's consider an example:
//!
//! ```rust,ignore
//! trait MyTrait {
//!     fn do_thing();
//! }
//!
//! // this proc macro is supposed to generate MyTrait impl
//! #[proc_macro_derive(MyTrait)]
//! fn example(input: TokenStream) -> TokenStream {
//!     // somewhere deep inside
//!     span_error!(span, "something's wrong");
//!
//!     // this implementation will be generated if no error happened
//!     quote! {
//!         impl MyTrait for #name {
//!             fn do_thing() {/* whatever */}
//!         }
//!     }
//! }
//!
//! // ================
//! // in main.rs
//!
//! // this derive triggers an error
//! #[derive(MyTrait)] // first BOOM!
//! struct Foo;
//!
//! fn main() {
//!     Foo::do_thing(); // second BOOM!
//! }
//! ```
//!
//! The problem is: the generated token stream contains only `compile_error!`
//! invocation, the impl was not generated. That means user will see two compilation
//! errors:
//! ```text
//! error: set_dummy test
//!  --> $DIR/probe.rs:9:10
//!   |
//! 9 |#[proc_macro_derive(MyTrait)]
//!   |                    ^^^^^^^
//!
//! error[E0277]: the trait bound `Foo: Default` is not satisfied
//!
//!   --> $DIR/probe.rs:14:10
//!
//!    |
//! 98 |     #[derive(MyTrait)]
//!    |              ^^^^^^^ the trait `Default` is not implemented for `Foo`
//!
//! ```
//!
//! But the second error is meaningless! We definitely need to fix this.
//!
//! Most used approach in cases like this is "dummy implementation" -
//! omit `impl MyTrait for #name` and fill functions bodies with `unimplemented!()`.
//!
//! This is how you do it:
//! ```rust,ignore
//!  trait MyTrait {
//!      fn do_thing();
//!  }
//!
//!  // this proc macro is supposed to generate MyTrait impl
//!  #[proc_macro_derive(MyTrait)]
//!  fn example(input: TokenStream) -> TokenStream {
//!      // first of all - we set a dummy impl which will be appended to
//!      // `compile_error!` invocations in case a trigger does happen
//!      proc_macro_error::set_dummy(Some(quote! {
//!          impl MyTrait for #name {
//!              fn do_thing() { unimplemented!() }
//!          }
//!      }));
//!
//!      // somewhere deep inside
//!      span_error!(span, "something's wrong");
//!
//!      // this implementation will be generated if no error happened
//!      quote! {
//!          impl MyTrait for #name {
//!              fn do_thing() {/* whatever */}
//!          }
//!      }
//!  }
//!
//!  // ================
//!  // in main.rs
//!
//!  // this derive triggers an error
//!  #[derive(MyTrait)] // first BOOM!
//!  struct Foo;
//!
//!  fn main() {
//!      Foo::do_thing(); // no more errors!
//!  }
//! ```

use proc_macro2::TokenStream;
use std::cell::RefCell;

thread_local! {
    pub(crate) static DUMMY_IMPL: RefCell<Option<TokenStream>> = RefCell::new(None);
}

pub(crate) fn take_dummy() -> Option<TokenStream> {
    DUMMY_IMPL.with(|dummy| dummy.replace(None))
}

/// Sets dummy token stream which will be appended to `compile_error!(msg);...`
/// invocations, should a trigger happen. Returns an old dummy, if set.
///
/// # Warning:
/// If you do `set_dummy(Some(ts))` you **must** do `set_dummy(None)`
/// before macro execution completes (`filer_macro_errors!` will do that for you)!
/// Otherwise `rustc` will fail with creepy
/// ```text
/// thread 'rustc' panicked at 'use-after-free in `proc_macro` handle', src\libcore\option.rs:1166:5
/// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
/// error: proc-macro derive panicked
/// ```
pub fn set_dummy(dummy: Option<TokenStream>) -> Option<TokenStream> {
    DUMMY_IMPL.with(|old_dummy| old_dummy.replace(dummy))
}
