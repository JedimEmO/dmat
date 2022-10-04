use dominator::Dom;

#[inline]
pub fn build_children(children: &mut [Option<Dom>]) -> Vec<Dom> {
    children.iter_mut().filter_map(|v| v.take()).collect()
}
