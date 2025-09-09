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
