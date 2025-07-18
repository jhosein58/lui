#[derive(Clone, Copy)]
pub enum PosKind {
    Relative(usize),
    Percentage(f32)
}

#[derive(Clone, Copy)]
pub enum Pos {
    Pos(PosKind, PosKind),
    Default
}

