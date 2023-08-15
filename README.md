# Tauri Types

A small library that translates the types from `Rust` to `TypeScript` for better integration for the `invoke` function from `Tauri`.

### Check it out on:
  - [Crates.io](https://crates.io/crates/tauri-types)
  - [GitHub](https://github.com/Maki325/tauri-types)
  - [Discord](discord.maki325.me?from=tauri-types-github)

### Usage

 1. Add `Tauri Types` to your project.

```sh
cargo add tauri-types
```

 2. Add the derive to a type you want to have access to in your JavaScript program. It can be either struct or enum.

```rust
use tauri_types::TauriType;

#[derive(TauriType)]
struct User {
  name: String,
  username: String,
  password: String,
  age: u32,
}
```

That's going to generate this in your `tauri-types.ts` file.

```ts
export type User = {
  name: string;
  username: string;
  password: string;
  age: number;
};
```

You can use the type by importing it from the `tauri-types.ts` file.

```ts
import { type User } from './tauri-types';
```

  3. Add the `tauri_types::command` macro above any function you want to export.

```rust
#[tauri_types::command]
fn get_user() -> User {
  return User {
    name: "Marko".to_string(),
    username: "Maki325".to_string(),
    password: "YoullNeverGuessIt".to_string(),
    age: 20,
  }
}
```

(You can export it with `use tauri_types::command;`, but it's better to use it this way, so it doesn't collide with the `command` macro from `tauri`.)

  4. Replace `tauri::generate_handler` macro with `tauri_types::generate_invoke`.

```rust
...
// .invoke_handler(tauri::generate_handler![
.invoke_handler(tauri_types::generate_invoke![
  get_user,
])
.run(tauri::generate_context!())
.expect("error while running tauri application");
...
```

  5. Import `invoke` from the `tauri-types.ts`, instead of directly from `tauri`.

```ts
// import { invoke } from '@tauri-apps/api/tauri';
import { invoke } from './tauri-types';
```

  6. Enjoy your type-safe `invoke`.

### Known dislike
**You will need to always use the second argument, even if it's `undefined`. I'm still trying to figure out a way to disable that, but for now just set it to `undefined`**.

Example:
```ts
import { invoke } from './tauri-types';

async function main() {
  // This **WILL** give a typescript error, for now
  // const user = await invoke('get_user');

  // This **WILL** work fine
  const user = await invoke('get_user', undefined);

  console.log('User:', user);
}

```


### Additional features

##### Export types with the same name under different `namespaces`:

```rust
#[derive(TauriType)]
#[namespace = "db"]
struct User {
  name: String,
  username: String,
  password: String,
  age: u32,
}
```

That's going to generate this in your `tauri-types.ts` file.

```ts
export namespace db {
  export type User = {
    name: string;
    username: string;
    password: string;
    age: number;
  };
}
```

##### Use custom type path for a function argument:

If the type you want is in another `namespace` or you want to explicitly type it to something else, you can use this feature.

```rust
#[tauri_types::command]
fn get_username(
  #[path = "db.User"]
  user: User
) -> String {
  return user.username;
}
```

##### Use custom type path for a function return type:

If the type you want is in another `namespace` or you want to explicitly type it to something else, you can use this feature.

```rust
#[tauri_types::command]
#[return_path = "db.User"]
fn get_user() -> User {
  return User {
    name: "Marko".to_string(),
    username: "Maki325".to_string(),
    password: "YoullNeverGuessIt".to_string(),
    age: 20,
  }
}
```


### Issues

If there are any issues, open one in the `Issues` tab on GitHub.
Just be sure that there isn't one like yours already opened!

This **is** a **very** side project for me, but I'll try to keep it working.