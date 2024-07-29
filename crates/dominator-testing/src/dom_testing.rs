use std::fmt::{Debug, Formatter};
use std::time::Duration;
use web_sys::NodeList;
use crate::{barrier, DominatorTestingError};

pub enum Condition {
    AtLeastCount(u32),
    AtMostCount(u32),
    Fn(Box<dyn Fn(NodeList) -> bool>)
}

impl Debug for Condition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Condition::AtLeastCount(count) => write!(f, "AtLeastCount({})", count),
            Condition::AtMostCount(count) => write!(f, "AtMostCount({})", count),
            Condition::Fn(_) => write!(f, "Fn")
        }
    }
}

/// Waits for a certain condition to be true for the given query, or times out.
///
/// # Example:
/// ```rust no_compile,ignore
/// async fn my_test() {
///     // my_button.click() should trigger an asynchronous action that eventually results
///     // in the expected dom structure
///     wait_for_query_selector_condition(".my-class", Condition::AtLeastCount(1), Duration::from_millis(200).await.unwrap();
///     wait_for_query_selector_condition(".my-class", Condition::Fn(Box::new(|node_list: NodeList| node_list.len() > 0)), Duration::from_millis(200).await.unwrap();
/// }
/// ```
pub async fn wait_for_query_selector_all_condition(query: &str, condition: Condition, timeout: Duration) -> Result<(), DominatorTestingError> {
    barrier(move || {
        let elements = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .query_selector_all(query)
            .unwrap();

        match &condition {
            Condition::AtLeastCount(count) => {
                elements.length() >= *count
            }
            Condition::AtMostCount(count) => {
                elements.length() <= *count
            }
            Condition::Fn(fn_) => {
                fn_(elements)
            }
        }
    }, timeout, format!("query: {query}")).await
}