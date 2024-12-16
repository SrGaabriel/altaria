use std::any::{Any, TypeId};
use std::collections::HashMap;
use crate::extractor::{ExtractorError, FromRequest};
use crate::request::HttpRequest;

pub struct Resource<T>(pub T);

impl<T> Resource<T> {
    pub fn new(value: T) -> Self {
        Resource(value)
    }
}

impl<T> FromRequest for Resource<T> where T : Clone + Send + Sync + 'static {
    fn from_request(_index: usize, request: &HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized
    {
        let resources = request.flow.as_ref().unwrap().get_resource::<T>()
            .ok_or(ExtractorError::UnregisteredExtension)?;
        Ok(Self(resources.clone()))
    }
}

pub type ResourceMap = HashMap<TypeId, Box<dyn ResourceObligations + Send + Sync>>;

pub(crate) fn to_resource_map(vec: &Vec<Box<dyn ResourceObligations + Send + Sync>>) -> ResourceMap {
    vec.iter().map(|resource| {
        (resource.inner_type_id(), resource.clone_box())
    }
    ).collect()
}

pub trait ResourceObligations: Any + Sync + Send {
    fn inner_type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }
    fn clone_box(&self) -> Box<dyn ResourceObligations + Send + Sync>;
    fn as_any(&self) -> &dyn Any;
}

impl<T> Clone for Resource<T> where T : Clone {
    fn clone(&self) -> Self {
        Resource(self.0.clone())
    }
}

impl<T> ResourceObligations for Resource<T> where T : Clone + Send + Sync + 'static {
    fn clone_box(&self) -> Box<dyn ResourceObligations + Send + Sync> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}