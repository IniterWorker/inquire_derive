use proc_macro2::TokenStream;
use quote::quote;
use syn::Error;

use crate::field::FieldSingleContext;

pub struct Context {
    pub fields: Vec<FieldSingleContext>,
}
