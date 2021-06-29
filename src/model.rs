use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Application {
    pub name: String,
    pub namespace: String,
    pub app_id: String,
    pub consumes: Option<String>,
    pub produces: Option<String>,
}

impl Application {
    pub fn new(namespace: String, name: String, app_id: String) -> Self {
        Application {
            namespace: namespace,
            name: name,
            app_id: app_id,
            consumes: None,
            produces: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Graph {
    pub applications: Vec<Application>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            applications: vec![],
        }
    }
}