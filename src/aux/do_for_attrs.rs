pub trait DoForAttrsStrategy<'a> {
    type Inputs;
    type Outputs;
    type Error;
}

/// This trait is implemented for all blocks.
///
/// For blocks a generic implementation will call [`DoForAttrs::do_for_attrs`]
/// for all attributes of the block variant at hand in the order they
/// stand in the source file.
/// Every attribute may modify the value of `outputs`.
///
/// For every kind of traversal the trait needs to be implemented for all attribute types.
/// That are many but it is easier than implementing
/// it for all occurences of an attribute in block variants separately.
pub trait DoForAttrs<'a, S: DoForAttrsStrategy<'a>> {
    fn do_for_attrs(&'a self, inputs: &S::Inputs, outputs: &mut S::Outputs)
    -> Result<(), S::Error>;
}
