The `pointerenter` event will fire when the user starts hovering/touching an element.

Event Data: [PointerData][crate::events::PointerData]

### Example:

```rust, no_run
fn app(cx: Scope) -> Element {
    render!(
        rect {
            width: "100",
            height: "100",
            background: "red",
            onpointerenter: |_| println!("Started hovering or touching!")
        }
    )
}