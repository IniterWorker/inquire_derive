use darling::FromMeta;
use proc_macro2::TokenStream;

use crate::field::FieldSingleContext;

use super::FieldInvokeInquire;

/// Editor prompts are meant for cases where you need the user to write some text that might not fit in a single line, such as long descriptions or commit messages.
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub struct Editor {
    /// Main message when prompting the user for input, "What is your name?" in the example above.
    pub prompt_message: Option<String>,
    /// Message displayed at the line below the prompt.
    pub help_message: Option<String>,
    /// If you want to override the selected editor, you can pass over the command and additional args.
    pub editor_command: Option<String>,
    /// Custom extension for the temporary file, useful as a proxy for proper syntax highlighting for example.
    pub file_extension: Option<String>,
    /// Pre-defined text to be written to the temporary file before the user is allowed to edit it.
    pub predefined_text: Option<String>,
    /// Custom validators to the user's input, displaying an error message if the input does not pass the requirements.
    pub validators: Option<String>,
    /// Custom formatter in case you need to pre-process the user input before showing it as the final answer.
    /// By default, a successfully submitted answer is displayed to the user simply as <received>.
    pub formatter: Option<String>,
}

impl FieldInvokeInquire for Editor {
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
