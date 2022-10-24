use darling::{ast, util, FromDeriveInput};
use proc_macro2::TokenStream;
use quote::quote;

use crate::field::{FieldMultiContext, FieldSingleContext};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(inquire), supports(struct_named))]
pub struct InquireInvokeOpts {
    /// Current struct ident
    pub ident: syn::Ident,
    /// Option to set inquire_#fieldname as private
    pub private: Option<bool>,
    /// All fields
    pub data: ast::Data<util::Ignored, FieldMultiContext>,
}

impl InquireInvokeOpts {
    pub fn gen(self) -> Result<TokenStream, Vec<syn::Error>> {
        let ident = &self.ident;
        let fields = self
            .data
            .take_struct()
            .take()
            .unwrap()
            .fields
            .into_iter()
            .map(|field| FieldSingleContext {
                ident: field.ident.clone().unwrap(),
                private: self.private,
                ty: field.ty.clone(),
                field_type: field.parse().unwrap(),
            })
            .collect::<Vec<_>>();

        // Generate Methods' impls
        let methods = fields.iter().fold(Vec::new(), |mut acc, elem| {
            let method = elem.generate_inquire_method().unwrap();
            acc.push(method);
            acc
        });

        let methods = quote! {
            #(#methods)*
        };

        // Generate Methods' calls
        let methods_calls = fields.iter().fold(Vec::new(), |mut acc, elem| {
            let method = elem.generate_inquire_method_call().unwrap();
            acc.push(method);
            acc
        });

        let inquire = quote! {
                /// Will invoke inquire prompts on mutable `Self`
                pub fn inquire_mut(&mut self) -> inquire::error::InquireResult<()> {
                    #(#methods_calls)*
                    Ok(())
                }

                /// Will invoke inquire prompts on new instance from Default
                pub fn from_inquire() -> inquire::error::InquireResult<Self> {
                    let mut s = Self::default();
                    s.inquire_mut()?;
                    Ok(s)
                }
        };

        // General implementation
        Ok(quote! {
            impl #ident {
                #methods
                #inquire
            }
        })
    }
}
