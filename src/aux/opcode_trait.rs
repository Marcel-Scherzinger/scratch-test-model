pub trait AsOpcodeName {
    fn opcode_name(&self) -> &'static str;
}
pub trait AsOpcodeUnit {
    type OpcodeUnit: Copy;
    #[allow(unused)]
    fn opcode(&self) -> Self::OpcodeUnit;
}

impl<T: AsOpcodeName> AsOpcodeName for &T {
    fn opcode_name(&self) -> &'static str {
        (*self).opcode_name()
    }
}
impl<T: AsOpcodeUnit> AsOpcodeUnit for &T {
    type OpcodeUnit = T::OpcodeUnit;
    fn opcode(&self) -> Self::OpcodeUnit {
        (*self).opcode()
    }
}
