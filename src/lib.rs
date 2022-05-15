//! # 🦀 `const_split_str`
//!
//! _Macros to split `&'static str` at compile time._

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

/// Split a `&'static str` at compile time.
///
/// # Syntax
///
/// ```ignore
/// split_str!(INPUT, DELIMITER);
/// ```
///
/// Where both `INPUT` and `DELIMITER` are `&'static str`.
///
/// # Semantics
///
/// For a given input and delimiter, the macro split the input and returns an array of `&'static str`.
///
/// # Example
///
/// ```rust
/// let [head, tail] = const_split_str::split!("head-tail", "-");
///
/// assert_eq!(head, "head");
/// assert_eq!(tail, "tail");
/// ```
#[proc_macro]
pub fn split(input: TokenStream) -> TokenStream {
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

/// Like [`str::split_once`], but at compile time.
///
/// # Syntax
///
/// ```ignore
/// split_once!(INPUT, DELIMITER);
/// ```
///
/// Where both `INPUT` and `DELIMITER` are `&'static str`.
///
/// # Semantics
///
/// For a given input and delimiter, the macro split the input at the first occurence of the
/// delimiter and returns a `(&'static str, &'static str)`.
///
/// # Example
///
/// ```rust
/// let (head, tail) = const_split_str::split_once!("head-tail", "-");
///
/// assert_eq!(head, "head");
/// assert_eq!(tail, "tail");
/// ```
#[proc_macro]
pub fn split_once(input: TokenStream) -> TokenStream {
    let SplitStrInput { string, delimiter } = parse_macro_input!(input as SplitStrInput);

    let expanded = if let Some((head, tail)) = string.value().split_once(&delimiter.value()) {
        quote! {
            (#head, #tail)
        }
    } else {
        quote_spanned! { delimiter.span() =>
            compile_error!("delimiter is not contained in the input string")
        }
    };

    TokenStream::from(expanded)
}
