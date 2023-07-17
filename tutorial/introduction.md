# Why WASM and Rust for frontend development?

Performance and soundness.
Your application code will run faster than the corresponding javascript code will do in V8.
This means that the richer your application is, the more you will benefit from this tech stack.

If your only goal is to show a few buttons and a text field, then this is probably not the right choice for you.
However, if you are building a complex web application with a complex state and business logic, you will quickly reap the benefits of using Rust over any other popular web language. 

## Introduction to dominator

[Dominator](https://crates.io/crates/dominator) is a library for building reactive frontend applications in Rust.
It is based on functional reactive principles, and relies on the [futures_signals](https://crates.io/crates/futures-signals) crate.

The tutorial is accompanied by a few examples, which can be found in the `tutorial/examples/` directory.
You can serve a locally hosted version of the examples by running:

```
cd tutorial/examples/<example>
npm install
npm start
```

This should open a browser window, with each example in a separate tab.
If you modify the code, the browser should automatically reload the page and reflect your changes.

## Requirements and setup

To build the examples this tutorial, you will need to have  with the `wasm32-unknown-unknown` target installed.

After following the instructions from [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install), you can install the target with:

```
rustup target add wasm32-unknown-unknown
```

Now install node and npm using nvm, use the latest LTS version.
Instructions on how to do this can be found [in the nvm repository](https://github.com/nvm-sh/nvm).

---

Next: [Hello, World!](hello_world.md)
