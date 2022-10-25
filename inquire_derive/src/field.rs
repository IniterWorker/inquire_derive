use darling::FromField;
use proc_macro2::TokenStream;

use crate::prompts::{
    Confirm, CustomType, DateSelect, Editor, FieldInquireForm, MultiSelect, Password, Select, Text,
};

#[derive(Debug)]
pub enum FieldType {
    Text(Text),
    DateSelect(DateSelect),
    Select(Select),
    MultiSelect(MultiSelect),
    Editor(Editor),
    Password(Password),
    CustomType(CustomType),
    Confirm(Confirm),
}

#[derive(Debug)]
pub enum Error {
    /// Too many field type in this attribute definition
    TooManyFieldType,
    /// Not implemented
    NotImplemented,
}

#[derive(Debug, FromField)]
#[darling(attributes(inquire))]
pub struct FieldMultiContext {
    pub ident: Option<syn::Ident>,
    pub vis: syn::Visibility,
    pub ty: syn::Type,
    pub text: Option<Text>,
    pub date_select: Option<DateSelect>,
    pub select: Option<Select>,
    pub multi_select: Option<MultiSelect>,
    pub editor: Option<Editor>,
    pub password: Option<Password>,
    pub custom_type: Option<CustomType>,
    pub confirm: Option<Confirm>,
    #[darling(default)]
    pub skip: Option<bool>,
}

impl FieldMultiContext {
    pub fn parse(self) -> Result<FieldType, Error> {
        let count = self.text.is_some() as i32
            + self.date_select.is_some() as i32
            + self.select.is_some() as i32
            + self.multi_select.is_some() as i32
            + self.editor.is_some() as i32
            + self.password.is_some() as i32
            + self.custom_type.is_some() as i32
            + self.confirm.is_some() as i32;

        // too many definition
        if count > 1 {
            return Err(Error::TooManyFieldType);
        }

        if let Some(ft) = self.text {
            Ok(FieldType::Text(ft))
        } else if let Some(ft) = self.date_select {
            Ok(FieldType::DateSelect(ft))
        } else if let Some(ft) = self.select {
            Ok(FieldType::Select(ft))
        } else if let Some(ft) = self.multi_select {
            Ok(FieldType::MultiSelect(ft))
        } else if let Some(ft) = self.editor {
            Ok(FieldType::Editor(ft))
        } else if let Some(ft) = self.password {
            Ok(FieldType::Password(ft))
        } else if let Some(ft) = self.custom_type {
            Ok(FieldType::CustomType(ft))
        } else if let Some(ft) = self.confirm {
            Ok(FieldType::Confirm(ft))
        } else {
            Err(Error::NotImplemented)
        }
    }
}

#[derive(Debug)]
pub struct FieldSingleContext {
    pub ident: syn::Ident,
    pub ty: syn::Type,
    pub private: Option<bool>,
    pub field_type: FieldType,
}

impl FieldSingleContext {
    pub fn generate_inquire_method_call(&self) -> Result<TokenStream, Vec<syn::Error>> {
        match &self.field_type {
            crate::field::FieldType::Text(ft) => ft.generate_inquire_method_call(self),
            crate::field::FieldType::DateSelect(ft) => ft.generate_inquire_method_call(self),
            crate::field::FieldType::Select(ft) => ft.generate_inquire_method_call(self),
            crate::field::FieldType::MultiSelect(ft) => ft.generate_inquire_method_call(self),
            crate::field::FieldType::Editor(ft) => ft.generate_inquire_method_call(self),
            crate::field::FieldType::Password(ft) => ft.generate_inquire_method_call(self),
            crate::field::FieldType::CustomType(ft) => ft.generate_inquire_method_call(self),
            crate::field::FieldType::Confirm(ft) => ft.generate_inquire_method_call(self),
        }
    }

    pub fn generate_inquire_method(&self) -> Result<TokenStream, Vec<syn::Error>> {
        match &self.field_type {
            crate::field::FieldType::Text(ft) => ft.generate_inquire_method(self),
            crate::field::FieldType::DateSelect(ft) => ft.generate_inquire_method(self),
            crate::field::FieldType::Select(ft) => ft.generate_inquire_method(self),
            crate::field::FieldType::MultiSelect(ft) => ft.generate_inquire_method(self),
            crate::field::FieldType::Editor(ft) => ft.generate_inquire_method(self),
            crate::field::FieldType::Password(ft) => ft.generate_inquire_method(self),
            crate::field::FieldType::CustomType(ft) => ft.generate_inquire_method(self),
            crate::field::FieldType::Confirm(ft) => ft.generate_inquire_method(self),
        }
    }
}
