mod command;
mod file;
mod invoke;
mod ty;

use command::Command;
use invoke::Invoke;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::parse_macro_input;
use ty::Ty;

#[proc_macro_attribute]
pub fn command(_args: TokenStream, s: TokenStream) -> TokenStream {
  let cmd = parse_macro_input!(s as Command);

  cmd.item.into_token_stream().into()
}

#[proc_macro_derive(TauriType, attributes(namespace))]
pub fn ty(s: TokenStream) -> TokenStream {
  parse_macro_input!(s as Ty);

  TokenStream::new()
}

#[proc_macro]
pub fn generate_invoke(s: TokenStream) -> TokenStream {
  let Invoke { item } = parse_macro_input!(s as Invoke);

  quote! {
    tauri::generate_handler![#item]
  }
  .into()
}
