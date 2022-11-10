# Inquire Derive

---

Library to support `inquire_derive` via `proc-macro`.

## Early stage

- Derive support for [inquire](https://github.com/mikaelmello/inquire) see [#supported](#supported-widgets).
- Original feature request [inquire #65](https://github.com/mikaelmello/inquire/issues/65)

## Supported Widgets

* [x] __Text__
* [x] __Select__
* [x] __MultiSelect__
* [x] __CustomType__
* [x] __Editor__
* [x] __Password__
* [x] __Confirm__
* [x] __DateSelect__
* [x] __Nested__


## Inquire Derive in action

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]

# The inquire derive crate
inquire = { version = "0.5.2" }
inquire_derive = { git = "https://github.com/IniterWorker/inquire_derive", branch = "master" }
```

</details>
<p></p>

```rust
use inquire_derive::InquireForm;

#[derive(Debug, InquireForm)]
pub struct Demo {
    #[inquire(text(
        prompt_message = "\"What's your path?\"",
        initial_value = "\"/my/initial/path\"",
        placeholder_value = "\"/my/placeholder/path\"",
    ))]
    pub path: String,
}

impl Default for Demo {
    fn default() -> Self {
        Self {
            path: "/my/default/path".to_string(),
        }
    }
}

fn main() {
    let mut ex = Demo::default();
    println!("{:?}", ex.inquire_mut().unwrap());
}
```

## Examples

* __Text__: `cargo run --example text`
* __Select__: `cargo run --example select`
* __MultiSelect__: `cargo run --example multi_select`
* __CustomType__: `cargo run --example custom_type`
* __Editor__: `cargo run --example editor`
* __Password__: `cargo run --example password`
* __Confirm__: `cargo run --example confirm`
* __DateSelect__: `cargo run --example date_select`
* __Form(Nested)__: `cargo run --example form`

## License

See LICENSE for details.