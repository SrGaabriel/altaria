use std::any::{Any, TypeId};
use crate::extractor::state::{Resource, ResourceMap, ResourceObligations};

impl RequestFlow {
    pub fn new(resources: ResourceMap) -> Self {
        Self {
            resources,
            interrupted: false
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

    pub fn interrupt(&mut self) {
        self.interrupted = true;
    }
}

pub struct RequestFlow {
    pub resources: ResourceMap,
    pub(crate) interrupted: bool
}
