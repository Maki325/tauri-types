use syn::{parse::Parse, ItemFn, Pat, PatType, ReturnType};

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

        string.push_str(";\n");
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
    }

    string.push_str(";\n\n");

    export_ts(&string)?;

    Ok(Command { item })
  }
}
