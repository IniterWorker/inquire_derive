use darling::FromMeta;
use proc_macro2::TokenStream;

use crate::field::FieldSingleContext;

use super::FieldInvokeInquire;

/// Select prompts are suitable for when you need the user to select one option among many.
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub struct Select {
    /// Required when creating the prompt.
    pub prompt_message: Option<String>,
    /// Options displayed to the user. must be non-empty.
    pub options_list: Option<String>,
    /// Index of the cursor when the prompt is first rendered. default is 0 (first option). if the index is out-of-range of the option list, the prompt will fail with an inquireerror::invalidconfiguration error.
    pub starting_cursor: Option<String>,
    /// Message displayed at the line below the prompt.
    pub help_message: Option<String>,
    /// Custom formatter in case you need to pre-process the user input before showing it as the final answer.
    /// Prints the selected option string value by default.
    pub formatter: Option<String>,
    /// Number of options displayed at once, 7 by default.
    pub page_size: Option<String>,
    /// On long lists, it might be helpful to display the indexes of the options to the user. via the renderconfig, you can set the display mode of the indexes as a prefix of an option. the default configuration is none, to not render any index when displaying the options.
    pub display_option_indexes: Option<String>,
    /// Function that defines if an option is displayed or not based on the current filter input.
    pub filter_function: Option<String>,
}

impl FieldInvokeInquire for Select {
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
