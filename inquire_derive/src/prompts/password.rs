use darling::FromMeta;
use proc_macro2::TokenStream;

use crate::field::FieldSingleContext;

use super::FieldInquireForm;

/// Password prompts are meant for secretive text inputs.
#[derive(Debug, FromMeta, Default)]
#[darling(default)]
pub struct Password {
    /// Set the display mode of the text input among hidden, masked and full via the PasswordDisplayMode enum.
    /// * Hidden: default behavior, no UI indicators.
    /// * Masked: behaves like a normal text input, except that all characters of the input are masked to a special character, which is '*' by default but can be customized via RenderConfig.
    /// * Full: behaves like a normal text input, no modifications.
    pub standard_display_mode: Option<String>, // TODO: make an enum Hidden, Masked, Full
    /// By enabling this feature by calling the with_display_toggle_enabled(), you allow the user to toggle between the standard display mode set and the full display mode.
    /// * If you have set the standard display mode to hidden (which is also the default) or masked, the user can press Ctrl+R to change the display mode to Full, and Ctrl+R again to change it back to the standard one.
    /// * Obviously, if you have set the standard display mode to Full, pressing Ctrl+R won't cause any changes.
    pub toggle_display_mode: Option<bool>,
    /// Message displayed at the line below the prompt.
    pub help_message: Option<String>,
    /// Custom formatter in case you need to pre-process the user input before showing it as the final answer.
    /// * By default, it prints eight asterisk characters: ********.
    pub formatter: Option<String>,
    /// Custom validators to make sure a given submitted input pass the specified requirements, e.g. not allowing empty inputs or requiring special characters.
    /// * No validators are on by default.
    pub validators: Option<String>,
}

impl FieldInquireForm for Password {
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
