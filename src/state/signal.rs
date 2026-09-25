use std::rc::Rc;

use crate::core::Unit;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Signal {
    // TODO: Add a signal ID for fast lookup.
    pub name: String,
    pub description: Option<String>,
    pub unit: Unit,
}

impl Signal {
    #[must_use]
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            description: None,
            unit: Unit::None,
        }
    }

    #[must_use]
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    #[must_use]
    pub fn without_description(mut self) -> Self {
        self.description = None;
        self
    }

    #[must_use]
    pub fn with_unit<S: Into<Unit>>(mut self, unit: S) -> Self {
        self.unit = unit.into();
        self
    }

    #[must_use]
    pub fn without_unit(mut self) -> Self {
        self.unit = Unit::None;
        self
    }

    #[must_use]
    pub fn reference_counted(self) -> Rc<Self> {
        Rc::new(self)
    }
}

pub type SignalId = usize;
