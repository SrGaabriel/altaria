use std::marker::PhantomData;
use crate::request::HttpRequest;

pub trait Extractor<T> {
    fn extract(&self, request: HttpRequest) -> T;
}

pub struct Path<T> {
    key: String,
    __phantom: PhantomData<T>
}

impl<T> Extractor<T> for Path<T>
    where T : From<String>
{
    fn extract(&self, request: HttpRequest) -> T {
        let path_values = request.path_values.unwrap();
        let path_value = path_values.get(&self.key);

        match path_value {
            Some(value) => T::from(value.clone()),
            None => panic!("No value passed for {}", self.key)
        }
    }
}