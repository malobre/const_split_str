//! Split a `&'static str` at compile time.
//!
//! # Syntax
//!
//! ```ignore
//! split_str!(INPUT, DELIMITER);
//! ```
//!
//! Where both `INPUT` and `DELIMITER` are `&'static str`.
//!
//! # Semantics
//!
//! For a given input and delimiter, the macro split the input and returns an array of `&'static str`.
//!
//! # Example
//!
//! ```rust
//! let [head, tail] = const_split_str::split_str!("head-tail", "-");
//!
//! assert_eq!(head, "head");
//! assert_eq!(tail, "tail");
//! ```

use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse::Parse, parse_macro_input, LitStr, Token};

struct SplitStrInput {
    string: LitStr,
    delimiter: LitStr,
}

impl Parse for SplitStrInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let string = input.parse::<LitStr>()?;
        input.parse::<Token![,]>()?;
        let delimiter = input.parse::<LitStr>()?;

        Ok(Self { string, delimiter })
    }
}

#[proc_macro]
pub fn split_str(input: TokenStream) -> TokenStream {
    let SplitStrInput { string, delimiter } = parse_macro_input!(input as SplitStrInput);

    let string_value = string.value();
    let delimiter_value = delimiter.value();

    let expanded = if !string_value.contains(&delimiter_value) {
        quote_spanned! { delimiter.span() =>
            compile_error!("delimiter is not contained in the input string")
        }
    } else {
        let substrings = string_value.split(&delimiter_value);

        quote! {
            [#(#substrings),*]
        }
    };

    TokenStream::from(expanded)
}
