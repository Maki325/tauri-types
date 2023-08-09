#[allow(unused)]
use tauri_types::{command, generate_invoke, TauriType};

#[derive(TauriType)]
struct Test {
  a: u32,
  b: u32,
}

// struct Test2(u32, u32);

#[command]
fn test(a: u32) -> u32 {
  return a;
}

#[command]
fn test2(a: Test) -> Test {
  return a;
}

// fn test3(a: &u32) -> u32 {
//   return *a;
// }

// fn test2([first, ..]: &[u32; 3]) -> u32 {
//   return *first;
// }

mod tauri {
  macro_rules! generate_handler {
    ($($a:tt),*) => {};
  }

  pub(crate) use generate_handler;
}

fn main() {
  // println!("Hello, world! A: {}", test2(&[1, 2, 3]));
  generate_invoke!(test, test2);
  println!("Hello, world! A: {}", test(2));
}
