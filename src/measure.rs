use elementtree::Element;

use super::{Annotations, Annotation};

/// Measure
#[derive(Debug, Clone, PartialEq)]
pub struct Measure {
    name: String,
    column: String,
    aggregator: Aggregator,
    visible: bool,
    annotations: Annotations,
}

impl Measure {
    pub fn new(
        name: String,
        column: String,
        aggregator: Aggregator,
        visible: bool,
        ) -> Self
    {
        Measure {
            name: name,
            column: column,
            aggregator: aggregator,
            visible: visible,
            annotations: Annotations::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    pub(crate) fn build_xml(&self, parent: &mut Element) {
        use self::Aggregator::*;

        let agg = match self.aggregator {
            Sum => "sum",
            Count => "count",
            Min => "min",
            Max => "max",
            Avg => "",
            DistinctCount => "distinct-count",
            Median{..} => "None",
            Custom{..} => "None",
        };

        let mut measure = parent.append_new_child("Measure")
            .set_attr("name", self.name.clone())
            .set_attr("column", self.column.clone())
            .set_attr("visible", self.visible.to_string())
            .set_attr("aggregator", agg.to_owned());

        if let Median{ref database, ref formula} = self.aggregator {
            use self::Database::*;
            let db = match *database {
                Postgres => "postgres",
                MonetDb => "monetdb",
            };
            let expression = measure.append_new_child("MeasureExpression");
            expression.append_new_child("SQL")
                .set_attr("dialect", db)
                .set_text(&formula[..]);
        }

        if let Custom{ref formula} = self.aggregator {
            let expression = measure.append_new_child("MeasureExpression");
            expression.append_new_child("SQL")
                .set_text(&formula[..]);
        }

        self.annotations.build_xml(&mut measure);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Aggregator {
    Sum,
    Count,
    Min,
    Max,
    Avg,
    DistinctCount,
    Median {
        database: Database,
        formula: String,
    },
    Custom {
        formula: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Database {
    Postgres,
    MonetDb,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalculatedMember {
    name: String,
    dimension: String,
    formula: String,
    visible: bool,
    annotations: Annotations,
}

impl CalculatedMember {
    pub fn new(
        name: String,
        dimension: String,
        formula: String,
        visible: bool,
        ) -> Self
    {
        CalculatedMember {
            name: name,
            dimension: dimension,
            formula: formula,
            visible: visible,
            annotations: Annotations::new(),
        }
    }

    pub fn add_annotation(&mut self, annotation: Annotation) {
        self.annotations.add_annotation(annotation);
    }

    pub(crate) fn build_xml(&self, parent: &mut Element) {
        let mut calculated_member = parent.append_new_child("CalculatedMember")
            .set_attr("name", self.name.clone())
            .set_attr("dimension", self.dimension.clone())
            .set_attr("visible", self.visible.to_string());

        calculated_member.append_new_child("Formula")
            .set_text(&self.formula[..]);

        self.annotations.build_xml(&mut calculated_member);
    }
}


