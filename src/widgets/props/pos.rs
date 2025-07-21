pub enum PosVal {
    Absolute(usize),           
    Expr(Box<dyn Fn(usize, usize, usize, usize) -> usize>),
}

pub enum Position {
    Absolute(PosVal, PosVal), 
    Default,   
}              


impl Position {
    
    pub fn center() -> Self{
        Position::Absolute(
            PosVal::Expr(Box::new(|sw,_,pw,_| (pw - sw) / 2 )), 
            PosVal::Expr(Box::new(|_: usize,sh,_,ph| (ph - sh) / 2))
        )
    }
}