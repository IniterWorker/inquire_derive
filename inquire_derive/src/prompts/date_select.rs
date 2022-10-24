use darling::FromMeta;
use proc_macro2::TokenStream;

use crate::field::FieldSingleContext;

use super::FieldInvokeInquire;

#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub struct DateSelect {
    /// Main message when prompting the user for input, "What is your name?" in the example above.
    pub prompt_message: Option<String>,
    /// Message displayed at the line below the prompt.
    pub help_message: Option<String>,
    ///  Default value selected when the calendar is displayed and the one select if the user submits without any previous actions. Current date by default.
    pub default_value: Option<String>,
    /// Custom validators to the user's selected date, displaying an error message if the date does not pass the requirements.
    pub validators: Option<String>,
    /// Custom formatter in case you need to pre-process the user input before showing it as the final answer.
    /// Formats to "Month Day, Year" by default.
    pub formatter: Option<String>,
    /// Which day of the week should be displayed in the first column of the calendar, Sunday by default.
    pub week_start: Option<String>,
    /// Inclusive boundaries of allowed dates in the interactive calendar. If any boundary is set, the user will not be able to move past them, consequently not being able to select any dates out of the allowed range.
    pub min_date: Option<String>,
    /// Inclusive boundaries of allowed dates in the interactive calendar. If any boundary is set, the user will not be able to move past them, consequently not being able to select any dates out of the allowed range.
    pub max_date: Option<String>,
}

impl FieldInvokeInquire for DateSelect {
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
