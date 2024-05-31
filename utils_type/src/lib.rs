extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    bracketed, parse::Parse, parse_macro_input, punctuated::Punctuated, token, Error, Fields,
    ItemStruct, Token,
};

struct IdentArray {
    _bracket_token: token::Bracket,
    ident_list: Punctuated<Ident, Token![,]>,
}

impl Parse for IdentArray {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let content;
        Ok(IdentArray {
            _bracket_token: bracketed!(content in input),
            ident_list: content.parse_terminated(Ident::parse, Token![,])?,
        })
    }
}

struct AttributeArgs {
    name: Ident,
    ident_array: Punctuated<IdentArray, Token![,]>,
}

impl Parse for AttributeArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        if input.peek(token::Comma) {
            let _comma: token::Comma = input.parse()?;
        }
        let ident_array: Punctuated<IdentArray, Token![,]> =
            input.parse_terminated(IdentArray::parse, Token![,])?;

        Ok(AttributeArgs { name, ident_array })
    }
}

/// Constructs a struct by picking fields which not in the set from the original struct.
///
/// ### Example
///
/// ```no_run
/// # use utility_types::omit;
/// #[omit(AuthorLikedComments, [content, link], [Debug])]
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
/// ```
///
/// The code above will become this:
///
/// ```no_run
/// struct Article<T> {
///     author: String,
///     content: String,
///     liked: usize,
///     comments: T,
///     link: Option<String>
/// }
///
/// #[derive(Debug)]
/// struct AuthorLikedComments<T> {
///     author: String,
///     liked: usize,
///     comments: T,
/// }
/// ```
///
/// ### Notice
///
/// Currently, generics are not analyzed. So rustc will complain if the field with generic is not included in the generated struct.
///

#[proc_macro_attribute]
pub fn omit(attr: TokenStream, source: TokenStream) -> TokenStream {
    let attr_parsed = parse_macro_input!(attr as AttributeArgs);
    let source_parsed = parse_macro_input!(source as ItemStruct);

    let AttributeArgs { name, ident_array } = attr_parsed;

    let mut attrs = &Punctuated::new();

    let array_first = ident_array.first();
    if let Some(ident_first) = array_first {
        attrs = &ident_first.ident_list;
    }

    let array_last = ident_array.last();
    let mut temp = Punctuated::new();
    temp.push(Ident::new("Debug", Span::call_site()));

    let mut derives = &temp;

    if let Some(ident_last) = array_last {
        derives = &ident_last.ident_list
    }

    let mut tokens = Error::new_spanned(&source_parsed, "Must define on a struct with named field")
        .to_compile_error();

    let vis = &source_parsed.vis;
    let fields = &source_parsed.fields;
    let generics = &source_parsed.generics;

    if let Fields::Named(fields) = fields {
        // 从当前struct字段中过滤omit的属性
        let fields = fields
            .named
            .iter()
            .filter(|f| attrs.iter().all(|attr| *attr != *f.ident.as_ref().unwrap()));

        let derives_ident = derives.iter();

        tokens = quote! {
            #source_parsed

            #[derive(#(#derives_ident),*)]
            #vis struct #name #generics {
                #(#fields),* // 遍历展示
            }
        };
    }

    tokens.into()
}
