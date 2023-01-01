# Rust Learning

Learning the rust programming language. Following [The Rust Programming Language](https://doc.rust-lang.org/book/index.html) instructions.

Also learning from [Learn Rust](https://tryhackme.com/room/rust) from tryhackme.

[Cargo](https://doc.rust-lang.org/cargo/)
Rust's version of NPM or PyPi. Download packages others have created.

[Clippy](https://github.com/rust-lang/rust-clippy)
Microsoft Clippy, but re-imagined for Rust to aid with development.

[RustFmt](https://github.com/rust-lang/rustfmt)
Automatically formats Rust code

[Cargo Test](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
A built in testing application created by the Rust developers.

[Cargo docs](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)
Automatically generate documentation for your code, using documentation comments (written in Markdown). This documentation is then sent to docs.rs upon publishing to Cargo. Not to mention that examples written in documentation are automatically tested for you. No more untested documentation examples!

[Rust-Analyzer](https://rust-analyzer.github.io/)
Think IDE but more intelligent. Rust Analyzer clearly labels what is wrong with your code, why it is wrong, the exact characters that conflict and cause the error, and 90% of the time it provides an "auto-fix" function that automatically fixes these errors for you.

[The Rust Book & Docs](https://doc.rust-lang.org/stable/book/)
Rust has a book, called The Book which details everything you could want to know about Rust. Neatly chaptered, easily searchable and at your disposal for free. If this isn't good enough, thanks to Rust's documentation comments almost every library you'll use will have extensive documentation online.

## Rust related commands

rustup

    It is the version manager.
    Perform "rustup --verison" to see the version
    Perform "rustup update" to update

rustc

    It is the compiler

cargo

    It is the package manager.

## Cargo related commands to build and run

Once the project is created using "cargo init" or "cargo new"

We can perform "cargo run" to compile and then run the code

    - The command will create a target folder (./target/)
    - a debug folder will be created under it (./target/debug/)
    - Under that, we will see the executeable (./target/debug/<executeable for the current project>)
    - And then the code will also be ran
    Note: This will be unoptimized

We can perform "cargo build" to build the program / compile the project

    - The command will create a target folder (./target/)
    - a debug folder will be created under it (./target/debug/)
    - Under that, we will see the executeable (./target/debug/<executeable for the current project>)
    - The code will not be run
    Note: This will be unoptimized

We can perform "cargo build --release" to compile it for production / it will create an optimized build

    - The command will create a target folder (./target/)
    - a debug folder will be created under it (./target/debug/)
    - Under that, we will see the executeable (./target/debug/<executeable for the current project>)
    - A release folder will be created under the target folder (./target/release/)
    - Here, there will be the optimized executeble (./target/debug/<optimized executeable for the current project>)
    Note: This will be optimized.
