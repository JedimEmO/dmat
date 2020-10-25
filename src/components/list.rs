use dominator::{Dom, html};
use futures_signals::signal_vec::{MutableVec};
use futures_signals::signal_vec::MutableSignalVec;
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::__rt::std::sync::RwLock;

pub struct List {
    children: RwLock<Option<Vec<Dom>>>
}

impl List {
    pub fn build() -> List {
        List {
            children: Default::default()
        }
    }

    pub fn static_children(mut self, children: Vec<Dom>) -> Self {
        self.children = RwLock::new(Some(children));
        self
    }

    pub fn dom(self) -> Dom {
        list(self)
    }
}

fn list(mut list: List) -> Dom {
    Dom::with_state(list, |mut list| {
        let mut children_lock = list.children.write().unwrap();

        let children = if children_lock.is_some() {
            let children = children_lock.take().unwrap();

            children.into_iter().map(|child| {
                html!("li", {
                    .class("dmat-list-item")
                    .child(child)
                })
            }).collect()
        } else {
            vec![]
        };

        html!("ul", {
            .class("dmat-list")
            .children(children)
        })
    })
}
