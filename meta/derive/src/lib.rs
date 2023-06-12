#![allow(dead_code)]
#![allow(unused_variables)]
use proc_macro::TokenStream;

mod helper;
mod impl_dao;
mod impl_rest;

#[proc_macro_derive(GetById)]
pub fn get_by_id(input: TokenStream) -> TokenStream {
  impl_dao::impl_get_by_id(input)
}

#[proc_macro_derive(GetAll)]
pub fn get_all(input: TokenStream) -> TokenStream {
  impl_dao::impl_get_all(input)
}

#[proc_macro_derive(Insert)]
pub fn insert(input: TokenStream) -> TokenStream {
  impl_dao::impl_insert(input)
}

#[proc_macro_derive(Update)]
pub fn update(input: TokenStream) -> TokenStream {
  impl_dao::impl_update(input)
}

#[proc_macro_attribute]
pub fn backend(_attr: TokenStream, item: TokenStream) -> TokenStream {
  item
}

#[proc_macro_attribute]
pub fn rest_get_by_id(attr: TokenStream, item: TokenStream) -> TokenStream {
  impl_rest::impl_rest_get_by_id(&attr, &item)
}

#[proc_macro_attribute]
pub fn rest_get_all(attr: TokenStream, item: TokenStream) -> TokenStream {
  impl_rest::impl_rest_get_all(&attr, &item)
}
