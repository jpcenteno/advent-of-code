#[derive(Clone, Debug)]
pub enum Instruction {
    Mul(u64, u64),
    Do,
    Dont,
    Ignored
}
