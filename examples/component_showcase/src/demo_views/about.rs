use dominator::{html, with_node, Dom};

pub fn about_view() -> Dom {
    container!(|d| {
        d.child(card!(
            static_list!([
                text!("Dominator Material"),
                text!("A performance first component library built on FRP principles"),
                html!("div", {
                .with_node!(e => {
                    .apply(|d| {
                        e.set_inner_html(r#"
<p>
    Dominator Material is a component library built on top of the <a href="https://github.com/Pauan/rust-dominator" target="_blank">dominator</a> framework.
    It is written in Rust, and compiles to webassembly for optimal runtime performance and binary size.
</p>

<p>
    This library focuses heavily on performance. 
    It leverages the benefits granted by pure(ish) functional reactive programming principles.
    <b>Dominator</b> is built on <a href="https://github.com/Pauan/rust-signals">futures-signals</a>, and as such, <b>dominator-material</b> also relies heavily on it.
</p>

<p>
    To get started with <b>futures-signals</b>, check out the <a href ="https://docs.rs/futures-signals/latest/futures_signals/tutorial/index.html" target="_blank">tutorial</a>
</p>
                            "#);
                        d
                    })
                })
            })
            ])
        ))
    })
}
