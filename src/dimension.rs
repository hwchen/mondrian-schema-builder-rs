use elementtree::Element;

use super::{Annotations, Annotation, Source};

#[derive(Debug, Clone, PartialEq)]
pub struct Dimension {
    name: String,
    hierarchies: Vec<Hierarchy>,
    annotations: Annotations,
}

impl Dimension {
    pub fn new(name: String) -> Self {
        Dimension {
            name: name,
            hierarchies: Vec::new(),
            annotations: Annotations::new(),
        }
    }

    pub fn add_hierarchy(&mut self, hierarchy: Hierarchy) {
        // figure out a way to make this compile time check?
        // Or just make this permissive and allow anything
//        if !self.hierarchies.is_empty() {
//            if hierarchy.name.is_none() {
//                bail!("hierarchy {:?} requires a name because it is not default hierarchy", self);
//            }
//        }

        self.hierarchies.push(hierarchy);
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    pub(crate) fn build_xml(&self, parent: &mut Element) {
        let mut dimension = parent.append_new_child("Dimension")
            .set_attr("name", self.name.clone());

        self.annotations.build_xml(&mut dimension);

        for hierarchy in &self.hierarchies {
            hierarchy.build_xml(&mut dimension);
        }
    }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Hierarchy {
    name: Option<String>, // default hierarchy does not need a name, but all following do.
    source: Option<Source>,
    has_all: bool,
    levels: Vec<Level>,
    annotations: Annotations,
}

impl Hierarchy {
    pub fn new(name: Option<String>, source: Option<Source>, has_all: bool) -> Self {
        Hierarchy {
            name: name,
            source: source,
            has_all: has_all,
            levels: Vec::new(),
            annotations: Annotations::new(),
        }
    }

    pub fn add_level(&mut self, level: Level) {
        self.levels.push(level);
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    fn build_xml(&self, parent: &mut Element) {
        let mut hierarchy = parent.append_new_child("Hierarchy")
            .set_attr("hasAll", self.has_all.to_string());

        self.annotations.build_xml(&mut hierarchy);

        if let Some(ref name) = self.name {
            hierarchy.set_attr("name", name.clone());
        }

        if let Some(ref source) = self.source {
            source.build_xml(&mut hierarchy);
        }

        for level in &self.levels {
            level.build_xml(&mut hierarchy);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Level {
    name: String,
    column: String,
    name_column: Option<String>,
    level_type: Option<String>,
    _type: Option<String>,
    unique_members: bool,
    annotations: Annotations,  // these are rendered inside <Annotations />
    properties: Vec<Property>,  // these are rendered directly under Level
}

impl Level {
    pub fn new(
        name: String,
        column: String,
        name_column: Option<String>,
        level_type: Option<String>,
        _type: Option<String>,
        unique_members: bool,
        ) -> Self
    {
        Level {
            name: name,
            column: column,
            name_column: name_column,
            level_type: level_type,
            _type: _type,
            unique_members: unique_members,
            annotations: Annotations::new(),
            properties: Vec::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    pub fn add_property(&mut self, property: Property) {
       self.properties.push(property);
    }

    fn build_xml(&self, parent: &mut Element) {
        let mut level = parent.append_new_child("Level")
            .set_attr("name", self.name.clone())
            .set_attr("column", self.column.clone())
            .set_attr("uniqueMembers", self.unique_members.to_string());

        if let Some(ref level_type) = self.level_type {
            level.set_attr("levelType", level_type.clone());
        }
        if let Some(ref name_column) = self.name_column {
            level.set_attr("nameColumn", name_column.clone());
        }

        self.annotations.build_xml(&mut level);

        for property in &self.properties {
            property.build_xml(&mut level);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Property {
    name: String,
    column: String,
    annotations: Annotations,
}

impl Property {
    pub fn new(name: String, column: String) -> Self {
        Property {
            name: name,
            column: column,
            annotations: Annotations::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    fn build_xml(&self, parent: &mut Element) {
        let mut property = parent.append_new_child("Property")
            .set_attr("name", self.name.clone())
            .set_attr("column", self.column.clone());

        self.annotations.build_xml(&mut property);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DimensionUsage {
    name: String,
    source: String, // this source is a reference to Dimenions; not table/view
    foreign_key: String,
    annotations: Annotations,
}

impl DimensionUsage {
    pub fn new(name: String, source: String, foreign_key: String) -> Self {
        DimensionUsage {
            name: name,
            source: source,
            foreign_key: foreign_key,
            annotations: Annotations::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    pub(crate) fn build_xml(&self, parent: &mut Element) {
        let mut dimension_usage = parent.append_new_child("DimensionUsage")
            .set_attr("name", self.name.clone())
            .set_attr("source", self.source.clone())
            .set_attr("foreign_key", self.foreign_key.clone());

        self.annotations.build_xml(&mut dimension_usage);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NamedSet {
    name: String,
    formula: String,
    visible: bool,
    annotations: Annotations,
}

impl NamedSet {
    pub fn new(name: String, formula: String, visible: bool) -> Self {
        NamedSet {
            name: name,
            formula: formula,
            visible: visible,
            annotations: Annotations::new(),
        }
    }

    pub(crate) fn build_xml(&self, parent: &mut Element) {
        let mut named_set = parent.append_new_child("NamedSet")
            .set_attr("name", self.name.clone())
            .set_attr("formula", self.formula.clone())
            .set_attr("visible", self.visible.to_string());

        self.annotations.build_xml(&mut named_set);

    }
}

