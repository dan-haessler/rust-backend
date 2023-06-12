use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, ItemFn};


pub fn impl_rest_get_by_id(attr: &TokenStream, item: &TokenStream) -> TokenStream {
  let items = item.clone();
  let input = parse_macro_input!(items as ItemFn);

  let stmts = &input.block.stmts;
  let attrs = &input.attrs;
  let vis = &input.vis;
  let model = format_ident!("{}", attr.to_string());
  let sig = &input.sig;

  let gen = quote! {
    #(#attrs)* #vis #sig {
        #(#stmts)*

        let model = &pool
            .interact(move |conn| #model::get_by_id(conn, path.abs()))
            .await
            .expect("Interaction Eror");

        match model {
            Ok(mdl) => HttpResponse::Ok().json(mdl),
            Err(e) => HttpResponse::Ok().body(e.to_string()),
        }
    }
  };

  TokenStream::from(gen)
}

pub fn impl_rest_get_all(attr: &TokenStream, item: &TokenStream) -> TokenStream {
  let items = item.clone();
  let input = parse_macro_input!(items as ItemFn);

  let stmts = &input.block.stmts;
  let attrs = &input.attrs;
  let vis = &input.vis;
  let model = format_ident!("{}", attr.to_string());
  let sig = &input.sig;

  let gen = quote! {
    #(#attrs)* #vis #sig {
        #(#stmts)*

        let model = &pool
            .interact(move |conn| #model::get_all(conn))
            .await
            .expect("Interaction Eror");

        match model {
            Ok(mdl) => HttpResponse::Ok().json(mdl),
            Err(e) => HttpResponse::Ok().body(e.to_string()),
        }
    }
  };

  //println!("{}", gen.to_string());

  TokenStream::from(gen)
}
