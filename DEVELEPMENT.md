# Architecture
* In addition to our GUI version, you can also use our CLI version. You can find the relevant introduction in the CLI folder: `cli/README.md`.
* If you want to adjust the GUI related code, you can find the relevant files in the `cli` folder.
* If you want to adjust the preset prompt of the Bot, you can configure it in `gui/bots.json`.

# Development and Build
1. Git clone the repo
2. Start polestar-gui:
```sh
cargo run --bin polestar-gui 
```
3. Build polestar-gui:
```sh
cargo build --bin polestar-gui --release
```