use crate::response::HttpResponse;
use crate::response::into::IntoResponse;

pub struct Either<L, R> {
    left: Option<L>,
    right: Option<R>
}

#[allow(dead_code)]
impl<L,R> Either<L, R> {
    pub fn left(left: L) -> Self {
        Either {
            left: Some(left),
            right: None
        }
    }

    pub fn right(right: R) -> Self {
        Either {
            left: None,
            right: Some(right)
        }
    }

    pub fn is_left(&self) -> bool {
        self.left.is_some()
    }

    pub fn is_right(&self) -> bool {
        self.right.is_some()
    }

    pub fn get_left(&self) -> Option<&L> {
        self.left.as_ref()
    }

    pub fn get_right(&self) -> Option<&R> {
        self.right.as_ref()
    }

    pub fn left_mut(&mut self) -> Option<&mut L> {
        self.left.as_mut()
    }

    pub fn right_mut(&mut self) -> Option<&mut R> {
        self.right.as_mut()
    }

    pub fn map_left<U, F: FnOnce(L) -> U>(self, f: F) -> Either<U, R> {
        match self {
            Either { left: Some(left), right: None } => Either::left(f(left)),
            Either { left: None, right: Some(right) } => Either::right(right),
            _ => unreachable!()
        }
    }

    pub fn map_right<U, F: FnOnce(R) -> U>(self, f: F) -> Either<L, U> {
        match self {
            Either { left: Some(left), right: None } => Either::left(left),
            Either { left: None, right: Some(right) } => Either::right(f(right)),
            _ => unreachable!()
        }
    }

    pub fn flatmap_left<U, F: FnOnce(L) -> Either<U, R>>(self, f: F) -> Either<U, R> {
        match self {
            Either { left: Some(left), right: None } => f(left),
            Either { left: None, right: Some(right) } => Either::right(right),
            _ => unreachable!()
        }
    }

    pub fn flatmap_right<U, F: FnOnce(R) -> Either<L, U>>(self, f: F) -> Either<L, U> {
        match self {
            Either { left: Some(left), right: None } => Either::left(left),
            Either { left: None, right: Some(right) } => f(right),
            _ => unreachable!()
        }
    }

    pub fn fold<U, V, F: FnOnce(L) -> U, G: FnOnce(R) -> V>(self, f: F, g: G) -> Either<U, V> {
        match self {
            Either { left: Some(left), right: None } => Either::left(f(left)),
            Either { left: None, right: Some(right) } => Either::right(g(right)),
            _ => unreachable!()
        }
    }

    pub fn unify<T>(self, f: impl FnOnce(L) -> T, g: impl FnOnce(R) -> T) -> T {
        match self {
            Either { left: Some(left), right: None } => f(left),
            Either { left: None, right: Some(right) } => g(right),
            _ => unreachable!()
        }
    }

    pub fn into_result(self) -> Result<R, L> {
        match self {
            Either { left: Some(left), right: None } => Err(left),
            Either { left: None, right: Some(right) } => Ok(right),
            _ => unreachable!()
        }
    }
}

impl<L, R> IntoResponse for Either<L, R> where L : IntoResponse, R : IntoResponse {
    fn into_response(self) -> HttpResponse {
        self.unify(
            |left| left.into_response(),
            |right| right.into_response()
        )
    }
}