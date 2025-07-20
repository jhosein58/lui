

pub enum PosVal {
    Absolute(usize),           
    Expr(Box<dyn Fn(usize, usize, usize, usize) -> usize>),
}

pub enum Position {
    Absolute(PosVal, PosVal), 
    Default,   
}              