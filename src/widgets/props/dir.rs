pub enum Dir {
    Top,
    Bottom,
    Left,
    Right
}

#[derive(Clone, Copy)]
pub struct  DirVal<T> {
    pub top: T,
    pub bottom: T,
    pub left: T,
    pub right: T
}