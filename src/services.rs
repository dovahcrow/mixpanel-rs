use BASE_URL;

#[allow(dead_code)]
pub enum Service {
    AnnotationsList,
    AnnotationsCreate,
    AnnotationsUpdate,
    AnnotationsDelete,
    Events,
    EventsTopToday,
    EventsHotLast31Days,
    Export
}

impl Service {
    pub fn construct_url(self) -> String {
        match self {
            Service::Export => format!("{}/{}", BASE_URL, "export"),
            _ => unimplemented!()
        }
    }
}

