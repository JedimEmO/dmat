# Some light DMAT introduction

## What is DMAT?

DMAT is a small component library providing common material-inspired components implemented using dominator.
If you ever need a reference for the components and what props they support, you can check out the [DMAT docs](https://jedimemo.github.io/dmat/doc/dmat_components/index.html).

## Let's make a simple counter

We'll go through implementing some of the [7 GUIs](https://eugenkiss.github.io/7guis/tasks) tasks using DMAT.
First off, let's make a simple counter.

The counter component will have one button to increment a number, and a read only input to display the current value. 

We can start by making a simple component that just has a button that increments the state.

```rust
pub fn counter() -> Dom {
    let counter_state = Mutable::new(0);

    list!({
        .rows([
            button!({
                .label("Increment")
                .style(ButtonStyle::Prominent)
                .click_handler(clone!(counter_state => move |_| {
                    counter_state.set(counter_state.get() + 1);
                }))
                .apply(|dom_builder| dom_builder.attr("id", "increment-button"))
            }),
            text_field!({
                .label(Some(html!("span", { .text("Counter Value")})))
                .value(MutableTValueAdapter::new_simple(&counter_state))
                .disabled(true)
            }).0
        ])
    })
}
```

You'll notice the use of the `button!` and `list!` and `text_field!` macros.
These are DMAT components!

Tey are on the same form as the `html!` macro, from dominator, but they are a bit more specialized.
You see that it has methods related to the specific component, i.e. `click_handler`, `style` and `label`.

All DMAT components will have a similar macro, specialized for the components' domain.
Additionally, they all have the `apply()` method, which gives you access to the underlying `DomBuilder` for the component the same way that the `html!` macro exposes this.
This is powerful but dangerous; you can end up breaking the component if you're not careful.

In this case, we use it to set the `id` attribute of the button, which we'll use later to test the component.

### List

Let's start at the top.
The `list!` component is a fairly simple one; it lists a vector of components vertically, as you would expect.

In the counter example, the list has a static collection of rows, but know that it has a `rows_signal` method that takes a signal of vectors of components, and will update the list as the signal changes.

```rust
fn dynamic_list() {
    let data = MutableVec::new_with_values(vec![
        "Hello".to_string(),
        "World".to_string(),
    ]);
    
    let rows = data.signal_cloned().map(|v| html!("div", { .text(v) }));
    
    list!({
        .rows_signal(rows)
    })
}
```

This allows us to make data driven lists.
It's also important to understand that since we use signal vec, *only* rows changed in the vector will be updated in the list.
This is generally true for all collection components in DMAT and dominator, and is the basis for the incredible rendering performance you can achieve!

### Button

The `button!` component allows us to have some interaction.
We can give it a `click_handler` that will be called when the button is clicked.
It can be given a style and a type, and of course dynamic or static content to display in the button text.

It's crucial to understand the need for cloning the mutable value in the click handler.
We have to do this because the click handler closure can be executed at any time (since it is triggered by a user action), and it has to know that the mutable value object has not been dropped.
As such, making sure that it owns a copy of the mutable value is necessary.


### Text field

Dealing with input is a relatively complex topic, and we will not cover a lot of it right now.

For our purpose here, the `text_field!` component is a simple input element with a label and a readonly number value.

The most important thing to be aware of here is the `MutableTValueAdapter`.
This allows us to bind the value of the input to a mutable value, and have it update as the value changes.
As long as the ty of the value implements `ToString` and `FromStr`, you can use this adapter as a shorthand for dealing with various types of data!

In the counter example, the input is disabled so that the user can not enter a value into the input.
But be aware that if you enable the input, the value adapter *will* mutate your mutable value upon user input!
In the input tutorial we will cover how to deal with validation and sanitization of user input.

That's it for the simple counter component! 
If you run the example, you'll see that it works as expected.

As a challenge, try to make a decrement button, and a reset button, and make sure that the reset button is disabled when the counter is 0!
Don't be afraid to look at the [DMAT docs](https://jedimemo.github.io/dmat/doc/dmat_components/index.html) for reference, and remember that most props have a `_signal` counterpart...

---
----
Previous: [Hello World!](./hello_world.md) Next: [Farmers Market](./farmers_market.md) 