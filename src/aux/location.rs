#[derive(derive_more::Debug, derive_more::Display, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum AttrLocation {
    #[debug("inputs")]
    #[display("inputs")]
    Inputs,
    #[debug("fields")]
    #[display("fields")]
    Fields,
    #[debug("mutation")]
    #[display("mutation")]
    Mutation,
}
