use dominator::Dom;

pub fn line_chart() -> Dom {
    svg!("rect", {
        .attr("width", "100")
        .attr("height", "100")
    })
}
