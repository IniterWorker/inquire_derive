use darling::FromMeta;
use proc_macro2::TokenStream;

use crate::field::FieldSingleContext;

use super::FieldInquireForm;

/// Confirm is a prompt to ask the user for simple yes/no questions, commonly known by asking the user displaying the (y/n) text.
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub struct Confirm {
    /// Required when creating the prompt.
    pub prompt_message: Option<String>,
    /// Default value returned when the user submits an empty response.
    pub default_value: Option<String>,
    /// Short hint that describes the expected value of the input.
    pub placeholder: Option<String>,
    /// Message displayed at the line below the prompt.
    pub help_message: Option<String>,
    /// Custom formatter in case you need to pre-process the user input before showing it as the final answer.
    /// Formats true to "Yes" and false to "No", by default.
    pub formatter: Option<String>,
    /// Custom parser for user inputs.
    /// The default bool parser returns true if the input is either "y" or "yes", in a case-insensitive comparison. Similarly, the parser returns false if the input is either "n" or "no".
    pub parser: Option<String>,
    /// Function that formats how the default value is displayed to the user.
    /// By default, displays "y/n" with the default value capitalized, e.g. "y/N".
    pub default_value_formatter: Option<String>,
    /// Error message to display when a value could not be parsed from the input.
    /// Set to "Invalid answer, try typing 'y' for yes or 'n' for no" by default.
    pub error_message: Option<String>,
}

impl FieldInquireForm for Confirm {
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
