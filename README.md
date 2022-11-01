# Inquire Derive

---

Derive support for [inquire](https://github.com/mikaelmello/inquire).
- See [inquire #65](https://github.com/mikaelmello/inquire/issues/65)
- Early stage of development

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
        prompt_message = "What's your path?",
        placeholder_value = "/my/placeholder/path",
        help_message = "my helper message for path",
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

## Tests/Example

* __Text__: `cargo run --example text`
* __Select__: `cargo run --example select`
* __Password__: `cargo run --example password`
* __Confirm__: `cargo run --example confirm`
* __DateSelect__: `cargo run --example date_select`
* __Form(Nested)__: `cargo run --example form`

