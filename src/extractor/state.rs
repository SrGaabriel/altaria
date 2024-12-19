use std::any::{Any, TypeId};
use std::collections::HashMap;
use async_trait::async_trait;
use crate::extractor::{ExtractorError, FromRequest};
use crate::request::HttpRequest;

pub struct Resource<T>(pub T);

impl<T> Resource<T> {
    pub fn new(value: T) -> Self {
        Resource(value)
    }
}

#[async_trait]
impl<T> FromRequest for Resource<T> where T : Clone + Send + Sync + 'static {
    async fn from_request(_index: usize, request: &mut HttpRequest) -> Result<Self, ExtractorError>
    where
        Self: Sized
    {
        let resources = request.flow.as_ref().unwrap().get_resource::<T>()
            .ok_or(ExtractorError::UnregisteredExtension)?;
        Ok(Self(resources.clone()))
    }
}

pub type ResourceMap = HashMap<TypeId, Box<dyn ResourceObligations + Send + Sync>>;

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