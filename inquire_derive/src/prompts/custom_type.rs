use darling::FromMeta;
use proc_macro2::TokenStream;

use crate::field::FieldSingleContext;

use super::FieldInquireForm;

/// CustomType prompts are generic prompts suitable for when you need to parse the user input into a specific type, for example an f64 or a rust_decimal, maybe even an uuid.
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub struct CustomType {
    /// Main message when prompting the user for input, "What is your name?" in the example above.
    pub prompt_message: Option<String>,
    /// Custom formatter in case you need to pre-process the user input before showing it as the final answer.
    pub formatter: Option<String>,
    /// Default value returned when the user submits an empty response.
    pub default_value: Option<String>,
    /// Message displayed at the line below the prompt.
    pub help_message: Option<String>,
    /// Error message to display when a value could not be parsed from the input.
    pub error_message: Option<String>,
    /// The default parser calls the str.parse method, which means that T must implement the FromStr trait. When the parsing fails for any reason, a default error message "Invalid input" is displayed to the user.
    pub parser: Option<String>,
}

impl FieldInquireForm for CustomType {
    fn generate_inquire_method(
        &self,
        _ctx: &FieldSingleContext,
    ) -> Result<TokenStream, Vec<syn::Error>> {
        unimplemented!()
    }

    fn generate_inquire_method_call(
        &self,
        _ctx: &FieldSingleContext,
    ) -> Result<TokenStream, Vec<syn::Error>> {
        unimplemented!()
    }
}
