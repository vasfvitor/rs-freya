### background

Specify a color as the background of an element.

You can learn about the syntax of this attribute [here](#color-syntax).

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            background: "red"
        }
    )
}
