use crate::helper::DeriveHelper;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Type};

/// This is an implementation query a Identifiable and Queryable Entity
pub fn impl_get_by_id(input: TokenStream) -> TokenStream {
  let item_struct = parse_macro_input!(input as ItemStruct);

  let name = &item_struct.ident;
  let diesel_id_field_type: Type = item_struct.get_field_type("id");
  let diesel_table = &item_struct.get_table_name();
  let diesel_connection = &item_struct.get_connection();
  let (impl_generics, ty_generics, where_clause) = &item_struct.generics.split_for_impl();
  let gen = quote! {
    impl #impl_generics GetById<#name, #diesel_id_field_type> for #name #ty_generics #where_clause {
      fn get_by_id(conn: &mut #diesel_connection, entity_id: #diesel_id_field_type)
        -> Result<#name, diesel::result::Error> {
        use crate::database::schema::#diesel_table::dsl::*;

        #diesel_table.find(entity_id).first::<#name>(conn)
      }
    }
  };
  //println!("{}", gen.to_string());
  TokenStream::from(gen)
}

pub fn impl_get_all(input: TokenStream) -> TokenStream {
  let item_struct = parse_macro_input!(input as ItemStruct);

  let name = &item_struct.ident;
  let diesel_table = &item_struct.get_table_name();
  let diesel_connection = &item_struct.get_connection();
  let (impl_generics, ty_generics, where_clause) = &item_struct.generics.split_for_impl();

  let gen = quote! {
    impl #impl_generics GetAll<#name> for #name #ty_generics #where_clause {
      fn get_all(conn: &mut #diesel_connection) -> Result<Vec<#name>, diesel::result::Error> {
        use crate::database::schema::#diesel_table::dsl::*;

        #diesel_table.load::<#name>(conn)
      }
    }
  };

  TokenStream::from(gen)
}

pub fn impl_insert(input: TokenStream) -> TokenStream {
  let item_struct = parse_macro_input!(input as ItemStruct);

  let name = &item_struct.ident;
  let diesel_table = &item_struct.get_table_name();
  let diesel_connection = &item_struct.get_connection();
  let (impl_generics, ty_generics, where_clause) = &item_struct.generics.split_for_impl();

  let gen = quote! {
    impl #impl_generics Insert<#name> for #name #ty_generics #where_clause {
      fn insert(conn: &mut #diesel_connection, value: &#name) -> Result<usize, diesel::result::Error> {
        use crate::database::schema::#diesel_table::dsl::*;

        diesel::insert_into(#diesel_table).values(value).execute(conn)
      }
    }
  };

  TokenStream::from(gen)
}

pub fn impl_update(input: TokenStream) -> TokenStream {
  let item_struct = parse_macro_input!(input as ItemStruct);

  let name = &item_struct.ident;
  let diesel_table = &item_struct.get_table_name();
  let diesel_connection = &item_struct.get_connection();
  let (impl_generics, ty_generics, where_clause) = &item_struct.generics.split_for_impl();

  let gen = quote! {
    impl #impl_generics Update<#name> for #name #ty_generics #where_clause {
      fn update(conn: &mut #diesel_connection, value: &#name) -> Result<usize, diesel::result::Error> {
        use crate::database::schema::#diesel_table::dsl::*;

        diesel::update(#diesel_table).set(value).execute(conn)
      }
    }
  };

  TokenStream::from(gen)
}
