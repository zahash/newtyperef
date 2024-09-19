use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Fields, GenericParam, ItemStruct, Lifetime, LifetimeParam, Result, Token,
    Type,
};

#[proc_macro_attribute]
pub fn newtyperef(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attrs as Attrs);
    let item_struct = parse_macro_input!(item as ItemStruct);

    let struct_name = &item_struct.ident;
    let struct_vis = &item_struct.vis;
    let generics = &item_struct.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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

    let mut ref_generics = generics.clone();
    ref_generics.params.insert(
        0,
        GenericParam::Lifetime(LifetimeParam::new(Lifetime::new("'__a", Span::call_site()))),
    );
    let (ref_impl_generics, ref_ty_generics, ref_where_clause) = ref_generics.split_for_impl();

    quote! {
        #item_struct

        impl #impl_generics #struct_name #ty_generics #where_clause {
            pub fn as_ref<'__a>(&'__a self) -> #ref_name #ref_impl_generics {
                #ref_name(&self.0)
            }

            pub fn as_mut<'__a>(&'__a mut self) -> #refmut_name #ref_impl_generics {
                #refmut_name(&mut self.0)
            }
        }

        #struct_vis struct #ref_name #ref_impl_generics (&'__a #ref_ty) #ref_where_clause;
        #struct_vis struct #refmut_name #ref_impl_generics (&'__a mut #mut_ty) #ref_where_clause;

        impl #ref_impl_generics std::ops::Deref for #ref_name #ref_ty_generics #ref_where_clause {
            type Target = #ref_ty;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        impl #ref_impl_generics std::ops::Deref for #refmut_name #ref_ty_generics #ref_where_clause {
            type Target = #mut_ty;

            fn deref(&self) -> &Self::Target {
                self.0
            }
        }

        impl #ref_impl_generics std::ops::DerefMut for #refmut_name #ref_ty_generics #ref_where_clause {
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
