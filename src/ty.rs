use quote::ToTokens;
use syn::{
  parse::{Parse, ParseStream},
  Fields, Item, ItemStruct, Type,
};

use crate::file::export_ts;

pub fn rust_type_to_js(ty: &str) -> String {
  if ty.contains("Vec") {
    return ty.replace("Vec <", "Array <");
  }
  let value = match ty {
    "u8" => "number",
    "u16" => "number",
    "u32" => "number",
    "u64" => "number",
    "u128" => "number",
    "usize" => "number",
    "i8" => "number",
    "i16" => "number",
    "i32" => "number",
    "i64" => "number",
    "i128" => "number",
    "isize" => "number",
    "f32" => "number",
    "f64" => "number",
    "bool" => "boolean",
    "char" => "string",
    "str" => "string",
    "String" => "string",
    _ => ty,
  };

  return value.to_string();
}

pub fn type_to_string(ty: &Type) -> String {
  match ty {
    Type::Path(path) => {
      return rust_type_to_js(&path.path.to_token_stream().to_string()).to_string();
    }
    Type::Array(array) => {
      return [&type_to_string(&array.elem), "[]"].join("");
    }
    Type::Slice(slice) => {
      return [&type_to_string(&slice.elem), "[]"].join("");
    }
    Type::Never(_) => "never".to_string(),
    _ => unimplemented!("Unsupported Type"),
  }
}

pub struct Ty {
  pub item: Item,
}

impl Parse for Ty {
  // Only support externaly tagged enums for now
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let item: Item = input.parse()?;
    // Enum
    // Struct
    // Type
    // Union ?

    let mut string = String::new();
    string.push_str("export type ");

    match &item {
      // We DON'T support Generics, for now, until maybe forever
      Item::Struct(s) => {
        string.push_str(&s.ident.to_string());
        string.push_str(" = ");

        match &s.fields {
          Fields::Named(fields) => {
            string.push_str("{\n");
            for field in &fields.named {
              string.push_str("  ");
              string.push_str(&field.ident.as_ref().unwrap().to_string());
              string.push_str(": ");
              string.push_str(&type_to_string(&field.ty));
              string.push_str(";\n");
            }
            string.push_str("}\n\n");
          }
          Fields::Unnamed(fields) => {
            string.push_str("{\n");
            for (i, field) in fields.unnamed.iter().enumerate() {
              string.push_str("  ");
              string.push_str(&format!("{}: ", i));
              string.push_str(&type_to_string(&field.ty));
              string.push_str(";\n");
            }
            string.push_str("}\n\n");
          }
          Fields::Unit => {
            string.push_str("never");
          }
        }
      }
      Item::Enum(e) => {
        string.push_str(&e.ident.to_string());
        string.push_str(" = ");

        let mut is_first = true;
        for variant in &e.variants {
          if !is_first {
            string.push_str(" | ");
          }
          is_first = false;
          string.push_str("{ ");
          string.push_str(&variant.ident.to_string());
          string.push_str(": ");

          match variant.fields {
            Fields::Unit => {
              string.push_str("never");
            }
            _ => unimplemented!("Unsupported Variant Type for Enums"),
          }

          string.push_str(" }");
        }

        string.push_str(";\n\n");
      }
      _ => unimplemented!("Unsupported Type"),
    }

    println!("{}", string);
    export_ts(&string)?;

    // match &item {
    //   Type::Path(path) => {
    //     path.
    //     // return rust_type_to_js(&path.path.to_token_stream().to_string()).to_string();
    //   }
    //   Type::Array(array) => {
    //     // return [&type_to_string(&array.elem), "[]"].join("");
    //   }
    //   Type::Slice(slice) => {
    //     // return [&type_to_string(&slice.elem), "[]"].join("");
    //   }
    //   // Type::Never(_) => "never".to_string(),
    //   _ => unimplemented!("Unsupported Type"),
    // }

    Ok(Ty { item })
  }
}
