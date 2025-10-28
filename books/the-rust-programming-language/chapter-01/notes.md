# Notes
 
## Installation

To install Rust, it's recommended to install `rustup`:

```bash
curl --proto '=https' --tlsv1.3 https://sh.rustup.rs -sSf | sh
```

On a macOS, it's also necessary to get a C compiler:

```bash
xcode-select --install
```

After that, to check the installation use the `rustc` command:

```bash
rustc --version
```

> [!NOTE]
> It's also possible to install Rust using `mise-en-place`:
> https://mise.jdx.dev/lang/rust.html.

## Hello, World!

> _Rust is an ahead-of-time compiled language, meaning you can compile a
> program and give the executable to someone else, and they can run it even
> without having Rust installed._

General ideas:

- Rust files have the `.rs` extension.
- Rust files follow the `snake_case` convention, but types use
  `UpperCamelCase`. The full naming guidelines can be found
  [here](https://rust-lang.github.io/api-guidelines/naming.html).

Language syntax:

- The keyword `fn` defines a function.
- A function body is wrapper in `{}`.
- Lines end with `;`.
- Macro names end with `!`.

Compilation and execution:

- To compile a program, run `rustc {file_name}.rs`. After the compilation, an
  executable, `{file_name}` is created.
- To run the executable, just type `./{file_name}`.

## Hello, Cargo!

> _Cargo is Rust's build system and package manager._

Cargo comes installed with Rust. To check the installation, run:

```bash
cargo --version
```

To create a new project with cargo:

```bash
cargo new {project_name}
```

Cargo commands:

- `cargo new`: Create new project.
- `cargo build`: Compile and build (create executable).
- `cargo check`: Check for compilation errors (without building).
- `cargo run`: Compile and run.

After running `cargo build`, the executable will be created. To run it:

```bash
./target/debug/{project_name}
```

It's also possible to use the `--release` flag for building. This enables
optimizations, which make the building slower, but the execution faster.
Instead of `target/debug`, the release version will be in `target/release`.