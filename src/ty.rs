use syn::{
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  spanned::Spanned,
  Attribute, Error, Expr, Fields, GenericArgument, Item, Lit, Meta, PathArguments, PathSegment,
  Token, Type,
};

use crate::file::export_ts;

pub fn rust_type_to_js(ty: &str) -> String {
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
      return rust_type_to_js(&segments_to_string(&path.path.segments)).to_string();
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

const VEC_CHECK: [&'static str; 3] = ["std", "vec", "Vec"];
fn segments_to_string(segments: &Punctuated<PathSegment, Token![::]>) -> String {
  if segments.len() == 3 {
    let not_vec = segments.iter().enumerate().any(|(i, segment)| {
      return segment.ident.to_string().ne(VEC_CHECK.get(i).unwrap());
    });
    if !not_vec {
      let segment = segments.iter().last().unwrap();
      if let PathArguments::AngleBracketed(arg) = &segment.arguments {
        if let Some(arg) = arg.args.iter().next() {
          if let GenericArgument::Type(ty) = arg {
            return [type_to_string(ty), "[]".to_string()].join("");
          }
        }
      }
    }
  } else if segments.len() == 1 {
    let segment = segments.iter().next().unwrap();
    if segment.ident.to_string().eq("Vec") {
      if let PathArguments::AngleBracketed(arg) = &segment.arguments {
        if let Some(arg) = arg.args.iter().next() {
          if let GenericArgument::Type(ty) = arg {
            return [type_to_string(ty), "[]".to_string()].join("");
          }
        }
      }
    }
  }

  return segments
    .iter()
    .map(|s| s.ident.to_string())
    .collect::<Vec<String>>()
    .join(".");
}

pub fn get_string_attribute_value(
  attrs: &Vec<Attribute>,
  path: &'static str,
) -> syn::Result<Option<(String, usize)>> {
  for (i, attr) in attrs.iter().enumerate() {
    if segments_to_string(&attr.path().segments) != path {
      continue;
    }

    let meta = match &attr.meta {
      Meta::NameValue(meta) => meta,
      _ => {
        return Err(Error::new(
          attr.meta.path().span(),
          "Only support NameValue Attribute for 'namespace'.",
        ))
      }
    };

    let path = match &meta.value {
      Expr::Lit(lit) => lit,
      _ => {
        return Err(Error::new(
          meta.value.span(),
          "Only support string literal values for 'namespace' attribute.",
        ))
      }
    };

    let path = match &path.lit {
      Lit::Str(path) => path,
      _ => {
        return Err(Error::new(
          path.lit.span(),
          "Only support string literal values for 'namespace' attribute.",
        ))
      }
    };

    return Ok(Some((path.value(), i)));
  }

  return Ok(None);
}

pub fn get_namespace(attrs: &mut Vec<Attribute>) -> syn::Result<Option<String>> {
  let result = get_string_attribute_value(attrs, "namespace");

  return result.map(|value| value.map(|(namespace, _)| namespace));
}

impl Parse for Ty {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut item: Item = input.parse()?;

    let mut string = String::new();

    match &mut item {
      // We DON'T support Generics, for now, until maybe forever
      Item::Struct(s) => {
        let namespace = get_namespace(&mut s.attrs)?;

        if let Some(namespace) = &namespace {
          string.push_str("export namespace ");
          string.push_str(namespace);
          string.push_str(" {\nexport type ");
        } else {
          string.push_str("export type ");
        }

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
            string.push_str("}");
          }
          Fields::Unnamed(fields) => {
            if fields.unnamed.len() == 1 {
              let first = fields.unnamed.first().unwrap();
              string.push_str(&type_to_string(&first.ty));
            } else {
              string.push_str("[ ");

              let mut is_first = true;
              for field in &fields.unnamed {
                if !is_first {
                  string.push_str(", ");
                }
                is_first = false;
                string.push_str(&type_to_string(&field.ty));
              }
              string.push_str(" ]");
            }
          }
          Fields::Unit => {
            string.push_str("null");
          }
        }

        if let Some(_) = &namespace {
          string.push_str(";\n}\n\n");
        } else {
          string.push_str(";\n\n");
        }
      }

      // Only support externaly tagged enums for now
      Item::Enum(e) => {
        let namespace = get_namespace(&mut e.attrs)?;

        if let Some(namespace) = &namespace {
          string.push_str("export namespace ");
          string.push_str(namespace);
          string.push_str(" {\nexport type ");
        } else {
          string.push_str("export type ");
        }

        string.push_str(&e.ident.to_string());
        string.push_str(" = ");

        let all_variant_names = &e
          .variants
          .iter()
          .map(|v| v.ident.to_string())
          .collect::<Vec<_>>();

        let mut is_first = true;
        for variant in &e.variants {
          if !is_first {
            string.push_str(" | ");
          }
          is_first = false;

          match &variant.fields {
            Fields::Unit => {
              string.push('\'');
              string.push_str(&variant.ident.to_string());
              string.push('\'');
            }
            Fields::Named(fields) => {
              let name = &variant.ident.to_string();

              string.push_str("{ ");
              string.push_str(name);
              string.push_str(": { ");

              for field in &fields.named {
                string.push_str(&field.ident.as_ref().unwrap().to_string());
                string.push_str(": ");
                string.push_str(&type_to_string(&field.ty));
                string.push_str("; ");
              }

              string.push_str(" }; ");

              all_variant_names
                .iter()
                .filter(|variant_name| variant_name != &name)
                .for_each(|name| {
                  string.push_str(name);
                  string.push_str(": undefined; ");
                });

              string.push_str("}");
            }
            Fields::Unnamed(fields) => {
              let name = &variant.ident.to_string();

              string.push_str("{ ");
              string.push_str(name);
              string.push_str(": ");

              if fields.unnamed.len() == 1 {
                let first = fields.unnamed.first().unwrap();
                string.push_str(&type_to_string(&first.ty));
                string.push_str("; ");
              } else {
                string.push_str("[ ");

                let mut is_first = true;
                for field in &fields.unnamed {
                  if !is_first {
                    string.push_str(", ");
                  }
                  is_first = false;
                  string.push_str(&type_to_string(&field.ty));
                }
                string.push_str(" ]; ");
              }

              all_variant_names
                .iter()
                .filter(|variant_name| variant_name != &name)
                .for_each(|name| {
                  string.push_str(name);
                  string.push_str(": undefined; ");
                });

              string.push_str(" }");
            }
          }
        }

        if let Some(_) = &namespace {
          string.push_str(";\n}\n\n");
        } else {
          string.push_str(";\n\n");
        }
      }

      Item::Type(ty) => {
        let namespace = get_namespace(&mut ty.attrs)?;

        if let Some(namespace) = &namespace {
          string.push_str("export namespace ");
          string.push_str(namespace);
          string.push_str(" {\nexport type ");
        } else {
          string.push_str("export type ");
        }

        string.push_str(&ty.ident.to_string());
        string.push_str(" = ");

        string.push_str(&type_to_string(&ty.ty));

        if let Some(_) = &namespace {
          string.push_str(";\n}\n\n");
        } else {
          string.push_str(";\n\n");
        }
      }
      _ => unimplemented!("Unsupported Type"),
    }

    export_ts(&string)?;

    Ok(Ty { item })
  }
}
