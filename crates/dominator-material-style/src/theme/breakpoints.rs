use crate::theme::to_sass::ToSass;

pub struct BreakpointValue<T: ToSass> {
    pub small: T,
    pub medium: T,
    pub large: T,
}

impl<T: ToSass> ToSass for BreakpointValue<T> {
    fn to_sass(&self) -> String {
        format!(
            r"@include respond-to('small') {{ {} }}
@include respond-to('medium') {{ {} }}
@include respond-to('large') {{ {} }}",
            self.small.to_sass(),
            self.medium.to_sass(),
            self.large.to_sass()
        )
    }
}
