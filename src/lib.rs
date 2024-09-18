use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Fields, ItemStruct, Result, Token, Type,
};

#[proc_macro_attribute]
pub fn newtyperef(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as Attrs);
    let item_struct = parse_macro_input!(item as ItemStruct);

    let struct_name = &item_struct.ident;
    let struct_vis = &item_struct.vis;

    let inner_ty = {
        let Fields::Unnamed(fields) = &item_struct.fields else {
            return syn::Error::new_spanned(&item_struct, "Expected tuple struct")
                .to_compile_error()
                .into();
        };

        if fields.unnamed.len() != 1 {
            return syn::Error::new_spanned(&item_struct, "Expected a single field in the struct")
                .to_compile_error()
                .into();
        }

        &fields.unnamed[0].ty
    };

    let ref_name = format_ident!("{}Ref", struct_name);
    let refmut_name = format_ident!("{}RefMut", struct_name);

    let ref_ty = attrs.ref_ty.unwrap_or_else(|| inner_ty.clone());
    let mut_ty = attrs.mut_ty.unwrap_or_else(|| inner_ty.clone());

    quote! {
        #item_struct

        impl #struct_name {
            pub fn as_ref<'a>(&'a self) -> #ref_name<'a> {
                #ref_name(&self.0)
            }

            pub fn as_mut<'a>(&'a mut self) -> #refmut_name<'a> {
                #refmut_name(&mut self.0)
            }
        }

        #struct_vis struct #ref_name<'a>(&'a #ref_ty);
        #struct_vis struct #refmut_name<'a>(&'a mut #mut_ty);

        impl<'a> std::ops::Deref for #ref_name<'a> {
            type Target = #ref_ty;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        impl<'a> std::ops::Deref for #refmut_name<'a> {
            type Target = #mut_ty;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        impl<'a> std::ops::DerefMut for #refmut_name<'a> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.0
            }
        }
    }
    .into()
}

struct Attrs {
    ref_ty: Option<Type>,
    mut_ty: Option<Type>,
}

impl Parse for Attrs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ref_ty = None;
        let mut mut_ty = None;

        while !input.is_empty() {
            if input.peek(Token![ref]) {
                input.parse::<Token![ref]>()?;
                input.parse::<Token![=]>()?;
                let ty: Type = input.parse()?;
                ref_ty = Some(ty);
            } else if input.peek(Token![mut]) {
                input.parse::<Token![mut]>()?;
                input.parse::<Token![=]>()?;
                let ty: Type = input.parse()?;
                mut_ty = Some(ty);
            } else if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "Unexpected token in newtyperef attribute",
                ));
            }
        }

        Ok(Attrs { ref_ty, mut_ty })
    }
}
