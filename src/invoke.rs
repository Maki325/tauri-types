use syn::{parse::Parse, punctuated::Punctuated, Path, Token};

use crate::file::export_ts;

pub struct Invoke {
  pub item: Punctuated<Path, Token![,]>,
}

fn create_function_data(string: &mut String, item: &Punctuated<Path, Token![,]>, ext: &str) {
  for ident in item {
    let str = &ident.segments.last().unwrap().ident.to_string();
    string.push_str("K extends '");
    string.push_str(&str);
    string.push_str("\'\n  ? ");
    string.push_str(&str);
    string.push_str(&ext);
    string.push_str("\n  : ");
  }
  string.push_str("never;\n");
}

impl Parse for Invoke {
  fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
    let item: Punctuated<Path, Token![,]> = Punctuated::parse_terminated(input)?;

    if item.len() == 0 {
      return Ok(Invoke { item });
    }

    let mut string = String::new();

    string.push_str("type Keys = ");

    let mut is_first = true;
    for ident in &item {
      let ident = &ident.segments.last().unwrap().ident;
      if !is_first {
        string.push_str(" | ");
      }
      is_first = false;
      string.push('\'');
      string.push_str(&ident.to_string());
      string.push('\'');
    }
    string.push_str(";\n");

    string.push_str("type FunctionArgs<K extends Keys> = ");
    create_function_data(&mut string, &item, "_args");

    string.push_str("type FunctionRet<K extends Keys> = ");
    create_function_data(&mut string, &item, "_return");
    string.push('\n');

    string.push_str(
      "import { invoke as tauriInvoke } from '@tauri-apps/api/core';

type Prettify<T> = {
  [K in keyof T]: T[K];
} & {};

export async function invoke<K extends Keys>(
  cmd: K,
  args: Prettify<FunctionArgs<K>>
): Promise<Prettify<FunctionRet<K>>> {
  return tauriInvoke(cmd, args);
}",
    );

    export_ts(&string)?;

    Ok(Invoke { item })
  }
}
