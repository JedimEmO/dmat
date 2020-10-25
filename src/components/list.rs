use dominator::{Dom, html};
use futures_signals::signal_vec::MutableVec;
use futures_signals::signal_vec::SignalVecExt;
use wasm_bindgen::__rt::std::rc::Rc;
use wasm_bindgen::__rt::std::sync::RwLock;

pub trait ListEntry {
    fn dom(&self) -> Dom;
}

enum ListData {
    Static(RwLock<Option<Vec<Dom>>>),
    Dynamic(MutableVec<Rc<dyn ListEntry>>),
}

pub struct List {
    children: ListData
}

impl List {
    pub fn build() -> List {
        List {
            children: ListData::Static(RwLock::new(None))
        }
    }

    pub fn static_children(mut self, children: Vec<Dom>) -> Self {
        self.children = ListData::Static(RwLock::new(Some(children)));
        self
    }

    pub fn dynamic_children(mut self, children: MutableVec<Rc<dyn ListEntry>>) -> Self {
        self.children = ListData::Dynamic(children);
        self
    }

    pub fn dom(self) -> Dom {
        list(self)
    }
}

fn list(list: List) -> Dom {
    Dom::with_state(list, |list| {
        match &list.children {
            ListData::Static(s) => static_list(s),
            ListData::Dynamic(vec) => dynamic_list(vec)
        }
    })
}

fn dynamic_list(vec: &MutableVec<Rc<dyn ListEntry>>) -> Dom {
    html!("ul", {
        .class("dmat-list")
        .children_signal_vec(vec.signal_vec_cloned().map(|v| {
            html!("li", {
                .class("dmat-list-item")
                .child(v.dom())
            })
        }))
    })
}

fn static_list(data: &RwLock<Option<Vec<Dom>>>) -> Dom {
    let mut v = data.write().unwrap();

    let children = if v.is_some() {
        let list = v.take().unwrap();

        list.into_iter().map(|child| {
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
}
