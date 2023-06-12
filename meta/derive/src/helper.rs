use proc_macro::TokenStream;
use quote::format_ident;
use syn::{Field, Ident, ItemStruct, Type};

pub trait DeriveHelper {
  fn get_field_name(&self, field: &str) -> Ident;
  fn get_field_type(&self, field: &str) -> Type;
  fn get_connection(&self) -> Ident;
  fn get_table_name(&self) -> Ident;
  fn get_field_containing(&self, field: &str) -> Field;
}

impl DeriveHelper for ItemStruct {
  fn get_field_name(&self, field: &str) -> Ident {
    self.get_field_containing(field).ident.expect("Could not find ident")
  }

  fn get_field_type(&self, field: &str) -> Type {
    self.get_field_containing(field).ty
  }

  fn get_table_name(&self) -> Ident {
    let item_table_name: String = self
      .attrs
      .iter()
      .map(|x| x.tokens.to_string())
      .find(|attr| attr.contains("table_name"))
      .expect("Attribute with table_name")
      .replace("table_name = ", "")
      .replace("(", "")
      .replace(")", "");
    format_ident!("{}", &item_table_name)
  }

  fn get_connection(&self) -> Ident {
    let item_table_name: String = self
      .attrs
      .iter()
      .map(|x| x.tokens.to_string())
      .find(|attr| attr.contains("connection"))
      .expect("Attribute with connection")
      .replace("connection = ", "")
      .replace("(", "")
      .replace(")", "");
    format_ident!("{}", &item_table_name)
  }

  fn get_field_containing(&self, field: &str) -> Field {
    self
      .fields
      .iter()
      .filter(|item: &&Field| {
        item
          .ident
          .as_ref()
          .filter(|field_ident| field_ident.to_string().contains(field))
          .is_some()
      })
      .next()
      .expect(&format!("Could not find field containing '{}'", field))
      .clone()
  }
}

pub fn concat_impl_proc_macro_attribute(
  attr: &TokenStream,
  item: &TokenStream,
  func_first: fn(&TokenStream, &TokenStream) -> TokenStream,
  func_second: fn(&TokenStream, &TokenStream) -> TokenStream,
) -> TokenStream {
  let first_result_item = func_first(attr, item);
  func_second(attr, &first_result_item)
}
