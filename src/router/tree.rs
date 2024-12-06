use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use crate::router::handler::RouteHandler;

pub struct RouteNode {
    handler: Option<Box<dyn RouteHandler + Send + Sync>>,
    dynamic_child: Option<Box<RouteNode>>,
    dynamic_id: Option<String>,
    children: HashMap<String, RouteNode>,
}

impl RouteNode {
    pub fn new() -> Self {
        Self {
            handler: None,
            dynamic_child: None,
            dynamic_id: None,
            children: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: &str, handler: Box<dyn RouteHandler + Send + Sync>) {
        let path_segments: Vec<&str> = separate_path_segments(path);
        let mut current = self;

        for segment in path_segments {
            if segment.starts_with('{') && segment.ends_with('}') {
                if current.dynamic_child.is_none() {
                    current.dynamic_child = Some(Box::new(RouteNode::new()))
                }
                let child = current.dynamic_child.as_mut().unwrap();
                child.dynamic_id = Some(segment[1..segment.len()-1].to_string());
                current = child
            } else {
                current = current.children.entry(segment.to_string()).or_insert_with(RouteNode::new)
            }
        }
        current.handler = Some(handler);
    }

    pub fn find(&self, path: &str) -> Option<RouteHandlerPath> {
        let segments: Vec<&str> = separate_path_segments(path);
        let mut current = self;
        let mut values: HashMap<String, String> = HashMap::new();

        for segment in segments {
            if let Some(child) = current.children.get(segment) {
                current = child;
            } else if let Some(dynamic) = &current.dynamic_child {
                values.insert(
                    dynamic.dynamic_id.clone().expect("Dynamic child doesn't have a dynamic id"),
                    segment.to_string()
                );
                current = dynamic
            } else {
                return None
            }
        }
        current.handler.as_ref().map(move |handler| RouteHandlerPath {
            handler,
            values
        })
    }
}

fn separate_path_segments(path: &str) -> Vec<&str> {
    path.split('/').filter(|s| !s.is_empty()).collect()
}

pub struct RouteHandlerPath<'a> {
    pub handler: &'a Box<dyn RouteHandler + Send + Sync>,
    pub values: HashMap<String, String>
}

impl Debug for RouteNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RouteNode")
            .field("dynamic_child", &self.dynamic_child)
            .field("dynamic_id", &self.dynamic_id)
            .field("children", &self.children)
            .finish()
    }
}