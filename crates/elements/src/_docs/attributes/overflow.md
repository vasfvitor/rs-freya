### overflow

Specify how overflow should be handled.

Accepted values: `clip` or `none`.

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            overflow: "clip"
            width: "100",
            height: "100%",
            rect {
                width: "500",
                height: "100%",
                background: "red",
            }
        }
    )
}
```