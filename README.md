# What is this?

I wanted to play around with [stdweb](https://github.com/koute/stdweb) to create a proof of concept to allow Web Components easily. If you aren't familiar with them. They are blocks of code that allow us to create our own html tags. Here's an example of where i'm going:

```rust
impl WebComponent for HelloWorld {
    fn get_element_name() -> &'static str {"hello-world"}

    fn constructor(){
        js! {
            window.currentElement.innerHTML = "Hello World!";
        }
    }
}

...

define_web_component(HelloWorld);
```

```html
<hello-world></hello-world>
```

would output in the browser

```html
Hello World!
```

# How to run this

```bash
curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly
rustup update nightly
rustup target add wasm32-unknown-unknown --toolchain=nightly
cargo +nightly web start --target-webasm
```
