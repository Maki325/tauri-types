use std::{fs::File, io::Write};

use syn::{
  parse::Parse,
  ItemFn, Pat, PatType, ReturnType,
  __private::{quote::quote, ToTokens},
};

use crate::{file::export_ts, ty::type_to_string};

pub struct Command {
  pub item: ItemFn,
}

impl Parse for Command {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let item: ItemFn = input.parse()?;

    let syn::Signature {
      ident,
      inputs,
      output,
      ..
    } = &item.sig;

    let mut string = String::new();

    string.push_str("export type ");
    string.push_str(&ident.to_string());

    if inputs.len() == 0 {
      string.push_str("_args = undefined;\n");
    } else {
      string.push_str("_args = {\n");

      for input in inputs {
        string.push_str("  ");
        let PatType { pat, ty, .. } = match input {
          syn::FnArg::Typed(input) => input,
          _ => panic!("unexpected fn arg"),
        };
        // Indent
        // Struct
        // TupleStruct
        // Wild
        // Slice
        // Rest? (Never got it, but maybe)

        match &**pat {
          Pat::Ident(ident) => {
            string.push_str(&ident.ident.to_string());
          }
          _ => unimplemented!("Unsupported pattern type"),
        }
        string.push_str(": ");

        string.push_str(&type_to_string(ty));
        // match &**ty {
        //   Type::Path(path) => {
        //     string.push_str(rust_type_to_js(&path.path.to_token_stream().to_string()));
        //   }
        //   Type::Array(array) => {
        //     string.push_str(rust_type_to_js(&array.elem.to_token_stream().to_string()));
        //     string.push_str("[]");
        //   }
        //   Type::Slice(slice) => {
        //     string.push_str(rust_type_to_js(&slice.elem.to_token_stream().to_string()));
        //     string.push_str("[]");
        //   }
        //   Type::Never(_) => {
        //     string.push_str("never");
        //   }
        //   _ => unimplemented!("Unsupported Type"),
        // }

        string.push_str(";\n");

        println!("input: {}", input.into_token_stream());
        match **pat {
          Pat::Const(_) => println!("Const"),
          Pat::Ident(_) => println!("Ident"),
          Pat::Lit(_) => println!("Lit"),
          Pat::Macro(_) => println!("Macro"),
          Pat::Or(_) => println!("Or"),
          Pat::Paren(_) => println!("Paren"),
          Pat::Path(_) => println!("Path"),
          Pat::Range(_) => println!("Range"),
          Pat::Reference(_) => println!("Reference"),
          Pat::Rest(_) => println!("Rest"),
          Pat::Slice(_) => println!("Slice"),
          Pat::Struct(_) => println!("Struct"),
          Pat::Tuple(_) => println!("Tuple"),
          Pat::TupleStruct(_) => println!("TupleStruct"),
          Pat::Type(_) => println!("Type"),
          Pat::Verbatim(_) => println!("Verbatim"),
          Pat::Wild(_) => println!("Wild"),
          _ => println!("Unknown pat"),
        }
      }

      string.push_str("};\n");
    }

    string.push_str("export type ");
    string.push_str(&ident.to_string());
    string.push_str("_return = ");

    match &*output {
      ReturnType::Default => {
        string.push_str(" void");
      }
      ReturnType::Type(_, ty) => {
        string.push_str(&type_to_string(ty));
      }
      // ReturnType::Type(_, ty) => match &**ty {
      //   Type::Path(path) => {
      //     string.push_str(rust_type_to_js(&path.path.to_token_stream().to_string()));
      //   }
      //   Type::Array(array) => {
      //     string.push_str(rust_type_to_js(&array.elem.to_token_stream().to_string()));
      //     string.push_str("[]");
      //   }
      //   Type::Slice(slice) => {
      //     string.push_str(rust_type_to_js(&slice.elem.to_token_stream().to_string()));
      //     string.push_str("[]");
      //   }
      //   Type::Never(_) => {
      //     string.push_str("never");
      //   }
      //   _ => unimplemented!("Unsupported Type"),
      // },
    }

    string.push_str(";\n\n");

    println!("{}", string);
    export_ts(&string)?;

    // println!(
    //   r#"
    //   type {:?} {{

    //   }}
    // "#,
    //   ident
    // );

    Ok(Command { item })
  }
}
