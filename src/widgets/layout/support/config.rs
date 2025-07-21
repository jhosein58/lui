
pub enum HorizontalAlign {
    Start,
    Center,
    End,
    Justify
}

pub enum CrossAlign {
    Top,
    Center,
    Bottom
}

pub enum Direction {
    LTR,
    RTL
}

pub enum WrapMode{
    Greedy,
    Balanced,
    FillLine
}

pub enum OrderStrategy {
    Natural,
    Index(Vec<usize>),
}

pub enum OrderMode {
    Forward,
    Reverse,
    Random
}