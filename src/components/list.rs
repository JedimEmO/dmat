use dominator::{Dom, html};
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

fn list(list: List) -> Dom {
    Dom::with_state(list, |list| {
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
