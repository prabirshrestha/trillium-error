# trillium-error

Error handler support for [trillium](https://trillium.rs) web framework.
Refer to (trillium discussions on error handling)[https://github.com/trillium-rs/trillium/discussions/31].


# Usage

```rust
#[macro_use]
extern crate trillium_error;

pub enum MyError {
    BarError,
    FooError,
}

#[async_trait]
impl Handler for MyError {
    async fn run(&self, conn: Conn) -> Conn {
        conn.with_status(500).with_body("Internal Server Error")
    }
}

#[handler]
async fn helloworld(conn: &mut Conn) -> Result<(), MyError> {
    conn.set_status(200);
    conn.set_body("hello world");
    // Ok(())
    Err(MyError::FooError)
}

fn main() {
    trillium_tokio::run(helloworld);
}
```

# LICENSE
License under Apache 2.0 or MIT
