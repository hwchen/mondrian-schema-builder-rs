// TODO
//
// - compare to python mondrian schema to make sure that everything is covered
// - one test case from python mondrian schema generator
// - fix main
// - implement fromStr! for things like Source ("schema.table" notataion, "alias:formula"
//   notation").
// - make it copy on write

extern crate elementtree;

pub mod dimension;
pub mod measure;

use elementtree::Element;

pub use self::dimension::{
    Dimension,
    Hierarchy,
    Level,
    Property,
    DimensionUsage,
    NamedSet
};
pub use self::measure::{
    Measure,
    Aggregator,
    Database,
    CalculatedMember
};

// Mondrian Schema elements
// All structs are listed up front,
// and then impl are done in second half of module.

#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    name: String,
    cubes: Vec<Cube>,
    dimensions: Vec<Dimension>,
    annotations: Annotations,
}

impl Schema {
    pub fn new(name: String) -> Self {
        Schema {
            name: name,
            dimensions: Vec::new(),
            cubes: Vec::new(),
            annotations: Annotations::new(),
        }
    }

    pub fn add_cube(&mut self, cube: Cube) {
        self.cubes.push(cube);
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    pub fn xml_render(&self) -> String {
        let mut schema = Element::new("Schema");
        schema .set_attr("name", self.name.clone());

        self.annotations.build_xml(&mut schema);

        for dimension in &self.dimensions {
            dimension.build_xml(&mut schema);
        }

        for cube in &self.cubes {
            cube.build_xml(&mut schema);
        }

        schema.to_string().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    name: String,
    source: Source,
    dimensions: Vec<Dimension>,
    dimension_usages: Vec<DimensionUsage>,
    measures: Vec<Measure>,
    calculated_members: Vec<CalculatedMember>,
    named_sets: Vec<NamedSet>,
    annotations: Annotations,
}

impl Cube {
    pub fn new(
        name: String,
        source: Source,
        ) -> Self
    {
        Cube {
            name: name,
            source: source,
            dimensions: Vec::new(),
            dimension_usages: Vec::new(),
            measures: Vec::new(),
            calculated_members: Vec::new(),
            named_sets: Vec::new(),
            annotations: Annotations::new(),
        }
    }

    pub fn add_dimension(&mut self, dimension: Dimension) {
        self.dimensions.push(dimension);
    }

    pub fn add_dimension_usage(&mut self, dimension_usage: DimensionUsage) {
        self.dimension_usages.push(dimension_usage);
    }

    pub fn add_measure(&mut self, measure: Measure) {
        self.measures.push(measure);
    }

    pub fn add_calculated_member(&mut self, calculated_member: CalculatedMember) {
        self.calculated_members.push(calculated_member);
    }

    pub fn add_named_set(&mut self, named_set: NamedSet) {
        self.named_sets.push(named_set);
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    fn build_xml(&self, parent: &mut Element) {
        let mut cube = parent.append_new_child("Cube")
            .set_attr("name", self.name.clone());

        self.annotations.build_xml(&mut cube);

        self.source.build_xml(&mut cube);

        for dimension in &self.dimensions {
            dimension.build_xml(&mut cube);
        }

        for dimension_usage in &self.dimension_usages {
            dimension_usage.build_xml(&mut cube);
        }

        for measure in &self.measures {
            measure.build_xml(&mut cube);
        }

        for calculated_member in &self.calculated_members {
            calculated_member.build_xml(&mut cube);
        }

        for named_set in &self.named_sets {
            named_set.build_xml(&mut cube);
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotations(Vec<Annotation>);

impl Annotations {
    pub fn new() -> Self {
        Annotations(Vec::new())
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.0.push(annotation);
    }

    fn build_xml(&self, parent: &mut Element) {
        if !self.0.is_empty() {
            let mut annotations = parent.append_new_child("Annotations");
            for annotation in &self.0 {
                annotation.build_xml(&mut annotations);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Annotation {
    name: String,
    value: String,
}

impl Annotation {
    pub fn new(name: String, value: String) -> Self {
        Annotation {
            name: name,
            value: value,
        }
    }

    fn build_xml(&self, parent: &mut Element) {
        parent.append_new_child("Annotation")
            .set_attr("name", self.name.clone())
            .set_text(&self.value[..]);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    Table {
        name: String,
        schema: Option<String>,
        annotations: Annotations,
    },
    View {
        alias: String,
        formula: String,
        annotations: Annotations,
    }
}

impl Source {
    pub fn new_table(name: String, schema: Option<String>) -> Self {
        Source::Table {
            name: name,
            schema: schema,
            annotations: Annotations::new(),
        }
    }

    pub fn new_view(alias: String, formula: String) -> Self {
        Source::View {
            alias: alias,
            formula: formula,
            annotations: Annotations::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        match *self {
            Source::Table {ref mut annotations, ..} => {
                annotations.add_annotation(annotation);
            },
            Source::View {ref mut annotations, ..} => {
                annotations.add_annotation(annotation);
            },
        }
    }

    fn build_xml(&self, parent: &mut Element) {
        match *self {
            Source::Table {ref name, ref schema, ref annotations} => {
                let mut table = parent.append_new_child("Table")
                    .set_attr("name", name.clone());
                if let Some(ref schema) = *schema {
                    table.set_attr("schema", schema.clone());
                }

                annotations.build_xml(&mut table);
            },
            Source::View {ref alias, ref formula, ref annotations} => {
                let mut view = parent.append_new_child("View")
                    .set_attr("alias", alias.clone())
                    .set_attr("formula", formula.clone());

                annotations.build_xml(&mut view);
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn it_works() {
        let mut schema = Schema::new("us_sales".to_owned());

        // Measure can be cloned for each clone
        let mut measure_dollars = Measure::new(
            "dollars".to_owned(),
            "mea_dollars".to_owned(),
            Aggregator::Sum,
            true,
        );
        measure_dollars.add_annotation(Annotation::new("shorthand".to_owned(), "in 100k".to_owned()));

        let mut dim_product = Dimension::new("Product".to_owned());
        let mut hierarchy_product = Hierarchy::new(
            None,
            Some(Source::new_view("source_alias".to_owned(), "sqlformula".to_owned())),
            true,
        );
        hierarchy_product.add_level(
            Level::new(
                "Product Group".to_owned(),
                "prod_code".to_owned(),
                Some("prod_desc".to_owned()),
                None,
                None,
                true,
            )
        );
        dim_product.add_hierarchy(hierarchy_product);

        // One Cube
        let mut source_eastern_sales = Source::new_table("eastern_sales".to_owned(), None);
        source_eastern_sales.add_annotation(Annotation::new("states".to_owned(), "MA,NY".to_owned()));
        let mut cube_eastern_sales = Cube::new("eastern_sales".to_owned(), source_eastern_sales);

        cube_eastern_sales.add_dimension(dim_product.clone());
        cube_eastern_sales.add_measure(measure_dollars.clone());
        schema.add_cube(cube_eastern_sales);

        // Second Cube
        let mut source_western_sales = Source::Table {
            name: "western_sales".to_owned(),
            schema: None,
            annotations: Annotations::new()
        };
        source_western_sales.add_annotation(Annotation::new("states".to_owned(), "CA,NV".to_owned()));
        let mut cube_western_sales = Cube::new("western_sales".to_owned(), source_western_sales);

        cube_western_sales.add_dimension(dim_product.clone());
        cube_western_sales.add_measure(measure_dollars.clone());
        schema.add_cube(cube_western_sales);

        println!("{}", schema.xml_render());
        panic!();
    }
}
