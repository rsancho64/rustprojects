## 0: synaptic 

## 1: ubuntu setup

[ocean](https://www.digitalocean.com/community/tutorials/install-rust-on-ubuntu-linux) con **`rustup`**

1. Elimino la instalación previa del sistema (rustup la ve)

`curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh`

Welcome to Rust!

This will download and install the 
- official compiler for the Rust programming language (**rustc**) and
- its package manager, **cargo**.

metadata n toolchains >> Rustup home: `/home/ray/.rustup`
This can be modified setting (`RUSTUP_HOME` void env var.)

Cargo home: `/home/ray/.cargo`
This can be modified setting (`CARGO_HOME` void env var.)

cargo, rustc, rustup and other commands will be added to Cargo's bin directory:
Cargo /bins: `/home/ray/.cargo/bin`

This path added to PATH env var.: updated:

**updated**: /home/ray/.profile:
ray@daw1 ~> cat /home/ray/.profile | grep -i cargo
**`. "$HOME/.cargo/env"`**

**updated**: /home/ray/.bashrc
ray@daw1 ~> cat /home/ray/.bashrc | grep -i cargo
**`. "$HOME/.cargo/env"`**

/home/ray/.config/fish/conf.d/rustup.fish
ray@daw1 ~> cat /home/ray/.config/fish/conf.d/rustup.fish
**`. "$HOME/.cargo/env.fish"`**

You can uninstall at any time with rustup self uninstall and these changes will be reverted.

Current installation options:


   default host triple: x86_64-unknown-linux-gnu
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

**1) Proceed with standard installation (default - just press enter)**
2) Customize installation -- no
3) Cancel installation    -- no 
>

...
downloading **6 components:
    `cargo` ; `clippy` ; `rust-docs` ; `rust-std` ; `rustc` ; `rustfmt`
Rust is installed now. Great!

**To get started: restart current shell...**
This would reload your PATH environment variable to include Cargo's /bin ($HOME/.cargo/bin).
To configure your current shell, you need to source the corresponding env file under $HOME/.cargo.
This is usually done by running one of the following (note the leading DOT):
- `. "$HOME/.cargo/env"`            # For sh/bash/zsh/ash/dash/pdksh
- ... or `source "$HOME/.cargo/env.fish"`  # For fish

or `source $HOME/.cargo/env`

`rustc --version` >> rustc 1.77.2 (25ef9e3d8 2024-04-09)

## go!

mkdir -p ~/rustprojects/testdir
cd ~/rustprojects/testdir

- [x] rustc
- [x] rustup update          // update rust
- [x] rustup self uninstall  // NOT NOW. 

## un primer contacto comentado con el lenguaje

[aqui](https://dev.to/maxwellnewage/un-pequeno-paseo-por-rust-4lko) 

