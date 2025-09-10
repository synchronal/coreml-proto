# CoreML Proto

`coreml_proto` uses the `.proto` files from [coremltools](https://github.com/apple/coremltools)
to generate Rust data structures and decoders using [prost](https://crates.io/crates/prost).

## Usage

``` Rust
use coreml_proto::proto::{Model, ModelDescription};
use prost::Message;

fn model_description(path: &Path) -> Result<Option<ModelDescription>> {
    let content = fs::read(path).await?;
    let model = Model::decode(&content[..])?;

    Ok(model.description)
}
```

## Development

``` shell
brew bundle
medic doc
medic update
medic audit
medic test
medic shipit
```

…or more manually:

``` shell
git submodule update --init
cargo build
cargo test
cargo check
cargo clippy
cargo fmt
cargo audit
```

## Licensing

This project uses protobuf definitions provided under
[this license](https://github.com/apple/coremltools/blob/main/LICENSE.txt). Take a look!
