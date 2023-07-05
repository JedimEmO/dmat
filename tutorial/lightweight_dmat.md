# Some light DMAT introduction

## What is DMAT?

DMAT is a small component library providing common material-inspired components implemented using dominator.


## Let's make a simple counter

We'll go through implementing some of the [7 GUIs](https://eugenkiss.github.io/7guis/tasks) tasks using DMAT.
First off, let's make a simple counter.

The counter component will have one button to increment a number, and a read only input to display the current value. 

We can start by making a simple component that just has a button that increments the state.

```rust
pub fn counter() -> Dom {
    let counter_state = Mutable::new(0);

    html!("div", {
        .child(button!({
            .label("Increment")
            .style(ButtonStyle::Prominent)
            .click_handler(clone!(counter_state => move |_| {
                counter_state.set(counter_state.get() + 1);
            }))
            .apply(|dom_builder| dom_builder.attr("id", "increment-button"))
        }))
    })
}
```

You'll notice the use of the `button!` macro.
This is a dmat component. 
It is on the same form as the `html!` macro, but it is a bit more specialized.
You see that it has methods related to the component, i.e. `click_handler`, `style` and `label`.

All dmat component will have a similar macro, specialized for the components' domain.
Additionally, they all have the `apply()` method, which gives you access to the underlying `DomBuilder` for the component.
This is powerful but dangerous; you can end up breaking the component if you're not careful.

In this case, we use it to set the `id` attribute of the button, which we'll use later to test the component.
