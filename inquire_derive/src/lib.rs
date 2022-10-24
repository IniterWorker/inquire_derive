//! # Inquire Derive
//!
//!
use crate::structural::InquireInvokeOpts;
use proc_macro::TokenStream;
use syn::DeriveInput;

pub(crate) mod field;
pub(crate) mod prompts;
pub(crate) mod structural;

#[proc_macro_derive(InquireInvoke, attributes(inquire))]
pub fn derive_inquire(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).expect("Error InquireInvoke derive");
    let parsed: Result<InquireInvokeOpts, darling::Error> =
        darling::FromDeriveInput::from_derive_input(&ast);
    // println!("{:?}", parsed);
    parsed.unwrap().gen().unwrap().into()
}
