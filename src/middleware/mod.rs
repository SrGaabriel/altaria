use std::any::{Any, TypeId};
use crate::extractor::state::{Resource, ResourceObligations, ResourceMap};
use crate::router::handler::RouteHandler;

pub struct MiddlewareRequestFlow<'a> {
    pub handler: &'a Box<dyn RouteHandler>,
    pub resources: ResourceMap
}

pub struct RequestFlow {
    pub resources: ResourceMap
}

impl RequestFlow {
    pub fn new(resources: ResourceMap) -> Self {
        Self {
            resources
        }
    }

    pub fn set_resources(&mut self, resources: ResourceMap) {
        self.resources = resources;
    }

    pub fn insert_resource(&mut self, resource: Box<dyn ResourceObligations + Send + Sync>) {
        self.resources.insert(resource.type_id(), resource);
    }

    pub fn add_resource<T>(&mut self, resource: Resource<T>) where T : Clone + Send + Sync + 'static {
        self.resources.insert(resource.type_id(), Box::new(resource));
    }

    pub fn get_resource<T : 'static>(&self) -> Option<&T> {
        self.resources.get(&TypeId::of::<Resource<T>>())
            .and_then(|resource| resource.as_any().downcast_ref::<Resource<T>>())
            .map(|resource| &resource.0)
    }
}

impl<'a> MiddlewareRequestFlow<'a> {
    pub fn new(handler: &'a Box<dyn RouteHandler>, resources: ResourceMap) -> Self {
        Self {
            handler,
            resources
        }
    }

    pub fn set_handler(&mut self, handler: &'a Box<dyn RouteHandler>) {
        self.handler = handler;
    }
}