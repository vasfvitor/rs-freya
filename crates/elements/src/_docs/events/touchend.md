The `touchend` event will fire when the user stops touching an element.

Event Data: [TouchData][crate::events::TouchData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            ontouchend: |_| println!("Stopped touching!")
        }
    )
}