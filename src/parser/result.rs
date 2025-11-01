

pub enum ParserResult<T, E> {
    Some(T),
    None,
    Err(E),   
}

impl<T, E> ParserResult<T, E> {
    pub fn or(self, other: impl FnOnce() -> Self) -> Self {
        match self {
            Self::Some(_) => self,
            Self::Err(_) => self,
            Self::None => other()
        }
    }
}