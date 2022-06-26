pub trait Architecture {
    type Instruction;

    fn sum(lhs: usize, rhs: usize) -> Vec<Self::Instruction>;
    fn mul(lhs: usize, rhs: usize) -> Vec<Self::Instruction>;
    fn div(lhs: usize, rhs: usize) -> Vec<Self::Instruction>;
}
