# Tutorials-TFHE-rs

## Setup & Installation

### Pre-requisites

Install Rust (in Mac OS):

```sh
brew install rustup
rustup-init
```

### Using TFHE-rs with nightly toolchain

Install the needed Rust toolchain:

```sh
rustup toolchain install nightly
```

Override the toolchain to use for the current project:

```sh
rustup override set nightly
```

To check the toolchain that Cargo will use by default, you can use the following command:

```sh
rustup show
```

### Building the project

Use in release mode (huge difference in performance) by running the following command in your project directory:

```sh
cargo build --release
```

### Running the project

Run the project by running the following command in your project directory:

```sh
cargo run --release
```

--------
