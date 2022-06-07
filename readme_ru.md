# Шаблон Rust для плат Iskra JS или Iskra JS Mini

## Требования

### Установите Rustup и Rust

Если у вас еще не установлен `rustup`, следуйте инструкциям на сайте [rustup.rs](https://rustup.rs).

### Установите плагины Rust
```sh
rustup default stable
rustup target add thumbv7em-none-eabihf
rustup component add llvm-tools-preview
```

### Установите VS Code и плагины

Если у вас еще не установлен VS Code, следуйте инструкциям на сайте [code.visualstudio.com](https://code.visualstudio.com).

Установите плагины:
```sh
code --install-extension matklad.rust-analyzer
code --install-extension bungcip.better-toml
code --install-extension marus25.cortex-debug
```

### Установите подкоманды и инструменты Cargo

```sh
cargo install cargo-generate cargo-binutils probe-run cargo-embed uf2conv
```

## Создайте проект

```sh
cargo generate gh:amperka/iskrajs-rust-template
```

## Скомпелируйте

Выполните в корне сгенерированного проекта.

```sh
cargo build --release
```

## Загрузите на плату

Выполните в корне сгенерированного проекта.

Для сборки и прошивки с использованием загрузчика uf2 используйте:
```sh
# для Windows или MacOS
cargo objcopy --release -- -O binary ./target/iskrajs.bin
uf2conv ./target/iskrajs.bin --base 0x08008000 --family 0x57755a57 --output iskrajs.uf2
# скопируйте файл iskrajs.uf2 на диск IskraJSBOOT или IskraJSMini

# для Linux
sh ./load.sh
```

Для сборки и прошивки с помощью отладчика используйте:
```sh
cargo embed --release
```
