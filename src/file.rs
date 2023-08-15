use std::{fs::File, io::Write};

static PATH: &'static str = "../src/tauri-types.ts";

static mut FILE: Option<File> = None;

pub fn export_ts(string: &String) -> syn::Result<()> {
  let mut file = match unsafe { &FILE } {
    Some(file) => file,
    None => {
      let file = match File::create(PATH) {
        Ok(file) => file,
        Err(why) => panic!("Couldn't create tauri-types.ts: {}", why.to_string()),
      };

      unsafe {
        FILE = Some(file);
      }

      match unsafe { &FILE } {
        Some(file) => file,
        _ => panic!("Couldn't get tauri-types.ts"),
      }
    }
  };
  match file.write_all(string.as_bytes()) {
    Err(why) => panic!("Couldn't write to tauri-types.ts: {}", why.to_string()),
    Ok(_) => (),
  };
  match file.flush() {
    Err(why) => panic!("Couldn't flush tauri-types.ts: {}", why.to_string()),
    Ok(_) => (),
  };

  return Ok(());
}
