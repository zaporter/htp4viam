#[derive(Debug, Clone)]
pub enum Resource {
    Apparatus(String),
    DeviceType(String),
}

pub struct ResourceList(Vec<Resource>);

impl ResourceList {
    pub fn new(resources : Vec<Resource>) -> Self {
        ResourceList(resources)
    }
    
}
