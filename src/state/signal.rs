use std::rc::Rc;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signal {
    pub name: String,
    pub description: Option<String>,
    pub unit: Option<String>,
}

impl Signal {
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            description: None,
            unit: None,
        }
    }

    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn without_description(mut self) -> Self {
        self.description = None;
        self
    }

    pub fn with_unit<S: Into<String>>(mut self, unit: S) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn without_unit(mut self) -> Self {
        self.unit = None;
        self
    }

    pub fn reference_counted(self) -> Rc<Self> {
        Rc::new(self)
    }
}
