use syn::{parse::Parse, ItemFn, Pat, PatType, ReturnType};

use crate::{
  file::export_ts,
  ty::{get_string_attribute_value, type_to_string},
};

pub struct Command {
  pub item: ItemFn,
}

fn lowercase(value: String) -> String {
  return value
    .chars()
    .enumerate()
    .map(|(i, c)| {
      if i == 0 {
        c.to_lowercase().next().unwrap()
      } else {
        c
      }
    })
    .collect();
}

impl Parse for Command {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let mut item: ItemFn = input.parse()?;

    let syn::Signature {
      ident,
      inputs,
      output,
      ..
    } = &mut item.sig;

    let mut string = String::new();

    string.push_str("export type ");
    string.push_str(&ident.to_string());

    if inputs.len() == 0 {
      string.push_str("_args = undefined;\n");
    } else {
      let mut args_string = String::new();

      for input in inputs {
        let PatType { pat, ty, attrs, .. } = match input {
          syn::FnArg::Typed(input) => input,
          _ => panic!("unexpected fn arg"),
        };

        let path = get_string_attribute_value(attrs, "path")?;

        let type_str = if let Some((path, index)) = path {
          attrs.remove(index);
          path
        } else {
          type_to_string(ty)
        };

        if &type_str == "tauri.State" {
          // Skipping the "tauri.State" type, as it's an injection from Tauri
          continue;
        }

        args_string.push_str("  ");

        match &**pat {
          Pat::Ident(ident) => {
            args_string.push_str(&ident.ident.to_string());
          }
          Pat::Struct(s) => {
            let last = s.path.segments.last().unwrap().ident.to_string();
            args_string.push_str(&lowercase(last));
          }
          Pat::TupleStruct(ts) => {
            let last = ts.path.segments.last().unwrap().ident.to_string();
            args_string.push_str(&lowercase(last));
          }
          Pat::Tuple(_ts) => {
            args_string.push_str("unknown");
          }
          _ => unimplemented!("Unsupported pattern type"),
        }
        args_string.push_str(": ");

        args_string.push_str(&type_str);

        args_string.push_str(";\n");
      }

      if !args_string.is_empty() {
        string.push_str("_args = {\n");
        string.push_str(&args_string);
        string.push_str("};\n");
      } else {
        string.push_str("_args = undefined;\n");
      }
    }

    string.push_str("export type ");
    string.push_str(&ident.to_string());
    string.push_str("_return = ");

    let path = get_string_attribute_value(&item.attrs, "return_path")?;
    if let Some((path, index)) = path {
      string.push_str(&path);
      item.attrs.remove(index);
    } else {
      match &*output {
        ReturnType::Default => {
          string.push_str(" void");
        }
        ReturnType::Type(_, ty) => {
          string.push_str(&type_to_string(ty));
        }
      }
    }

    string.push_str(";\n\n");

    export_ts(&string)?;

    Ok(Command { item })
  }
}
