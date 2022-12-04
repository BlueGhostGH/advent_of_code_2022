pub type Error = chumsky::error::Cheap<char>;
pub type BParser<T> = chumsky::BoxedParser<'static, char, T, Error>;
