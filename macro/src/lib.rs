use proc_macro::{TokenStream, TokenTree};
use proc_macro2::Ident;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

/// Adds `discriminant()` function to get numeric value of enum variants. Also adds
/// `from_discriminant()` function to create unit type enum variants from discriminants.
///
/// The `discriminant()` function relies on casting, as described in the
/// [Rust language documentation](https://doc.rust-lang.org/std/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant).
/// The `from_discriminant()` function on the other hand is basically a `match` statement of with
/// all the unit type variants.
#[proc_macro_attribute]
pub fn discriminant(arguments: TokenStream, item: TokenStream) -> TokenStream {
    let enum_item = parse_macro_input!(item as syn::ItemEnum);
    let enum_name = &enum_item.ident;
    let (variant_names, discrimnants) = enum_unit_variants(&enum_item);
    let argument_tokens: proc_macro2::TokenStream = arguments.clone().into();
    let Some(repr_type) = get_repr_type(arguments) else {
        return syn::Error::new_spanned(
            argument_tokens,
            "Valid enum representation type expected as argument to discriminant, \
            e.g., #[discriminant(u8)]",
        )
        .to_compile_error()
        .into();
    };

    quote! {
        #[repr(#argument_tokens)]
        #enum_item

        impl #enum_name {
            /// Creates enum variant from discriminant numeric value if there is a unit type variant
            /// with that value.
            fn from_discriminant(discriminant: #repr_type) -> Option<Self> {
                match discriminant {
                    // Match arm guard needed in case discriminant is not a literal but
                    // constant other expression
                    #( discriminant if discriminant == #discrimnants =>
                        Some(#enum_name::#variant_names), )*
                    _ => None,
                }
            }

            /// Returns the discriminant numeric value of an enum variant.
            fn discriminant(&self) -> #repr_type {
                // See https://doc.rust-lang.org/core/mem/fn.discriminant.html
                unsafe {
                    *<*const _>::from(self).cast::<#repr_type>()
                }
            }
        }
    }
    .into()
}

/// Returns the first valid representation type found in the arguments
fn get_repr_type(arguments: TokenStream) -> Option<syn::Path> {
    let alt_repr = [
        "u8", "u16", "u32", "u64", "u128", "usize", "i8", "i16", "i32", "i64", "i128", "isize",
    ];

    arguments
        .into_iter()
        .filter_map(|token_tree| {
            if let TokenTree::Ident(ident) = token_tree {
                let ident_str = ident.to_string();
                if alt_repr.contains(&ident_str.as_str()) {
                    return Some(syn::parse_str::<syn::Path>(&ident_str).unwrap());
                }
            }
            None
        })
        .next()
}

/// Returns a tuple of the names and discriminants of the unit variants of an enum. The
/// discriminants are returned as expressions, since explicit input discriminants can be
/// constant expressions.
fn enum_unit_variants(enum_item: &syn::ItemEnum) -> (Vec<Ident>, Vec<syn::Expr>) {
    let mut previous_expr: Option<syn::Expr> = None;
    enum_item
        .variants
        .iter()
        .filter(|variant| matches!(variant.fields, syn::Fields::Unit))
        .map(|variant| {
            let expr = if let Some(discriminant) = &variant.discriminant {
                discriminant.1.clone()
            } else if let Some(ref old_expr) = previous_expr {
                parse_quote!( 1 + #old_expr )
            } else {
                parse_quote!(0)
            };
            previous_expr = Some(expr.clone());
            (variant.ident.clone(), expr)
        })
        .unzip()
}
