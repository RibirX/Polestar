# Contribute to Polestar

Want to contribute to Polestar? There are a few things you need to know.

## Catalog Structure

* The core code of Polestar in `core` folder, what has all data structure and logic in it. It is a standalone AI chat tool that can be integrated into your product separately.
* `cli` folder is the Polestar CLI version. You can use it when you don't need GUI. The relevant introduction is available in the CLI folder: `cli/README.md`.
* `gui` folder is the Polestar GUI version. It build by Ribir what is Rust GUI framework.
* If you want to adjust the preset prompt of the Bot, you can configure it in `config/bots.json`.

## Development and Build

1. Git clone the repo
2. Start polestar-gui:

```sh
cargo run --bin polestar-gui 
```

3. Build polestar-gui:
  
```sh
cargo build --bin polestar-gui --release
```
