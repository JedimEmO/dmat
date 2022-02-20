use dominator::Dom;

#[inline]
pub fn build_children(children: &mut [Option<Dom>]) -> Vec<Dom> {
    children.into_iter().filter_map(|v| v.take()).collect()
}
