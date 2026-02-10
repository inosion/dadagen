//! Dependency Resolution Engine
//!
//! This module provides dependency graph construction, circular dependency detection,
//! and topological sorting for field generation order.

use crate::ast::{DslDocument, Generator};
use std::collections::{HashMap, HashSet, VecDeque};
use thiserror::Error;

/// Dependency resolution errors
#[derive(Debug, Error, Clone, PartialEq)]
pub enum DependencyError {
    #[error("Circular dependency detected: {cycle}")]
    CircularDependency { cycle: String },

    #[error("Undefined field reference: '{field}' referenced by '{referrer}'")]
    UndefinedReference { field: String, referrer: String },

    #[error("Self-reference detected in field '{field}'")]
    SelfReference { field: String },

    #[error("Empty document cannot have dependencies")]
    EmptyDocument,
}

pub type DependencyResult<T> = Result<T, DependencyError>;

/// Dependency graph for field generation order
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// Map from field name to its dependencies (fields it depends on)
    dependencies: HashMap<String, HashSet<String>>,
    /// Map from field name to fields that depend on it (reverse dependencies)
    dependents: HashMap<String, HashSet<String>>,
    /// All field names in the document
    fields: HashSet<String>,
}

impl DependencyGraph {
    /// Build a dependency graph from a DSL document
    pub fn from_document(doc: &DslDocument) -> DependencyResult<Self> {
        if doc.fields.is_empty() {
            return Err(DependencyError::EmptyDocument);
        }

        let mut dependencies = HashMap::new();
        let mut dependents = HashMap::new();
        let mut fields = HashSet::new();

        // First pass: collect all field names
        for field in &doc.fields {
            fields.insert(field.name.clone());
            dependencies.insert(field.name.clone(), HashSet::new());
            dependents.insert(field.name.clone(), HashSet::new());
        }

        // Second pass: extract dependencies
        for field in &doc.fields {
            let deps = Self::extract_dependencies(&field.generator);

            // Validate dependencies
            for dep in &deps {
                // Check for self-reference
                if dep == &field.name {
                    return Err(DependencyError::SelfReference {
                        field: field.name.clone(),
                    });
                }

                // Check for undefined reference
                if !fields.contains(dep) {
                    return Err(DependencyError::UndefinedReference {
                        field: dep.clone(),
                        referrer: field.name.clone(),
                    });
                }
            }

            // Store dependencies
            dependencies.insert(field.name.clone(), deps.clone());

            // Update reverse dependencies
            for dep in deps {
                dependents
                    .entry(dep.clone())
                    .or_insert_with(HashSet::new)
                    .insert(field.name.clone());
            }
        }

        let graph = Self {
            dependencies,
            dependents,
            fields,
        };

        // Check for circular dependencies
        graph.detect_cycles()?;

        Ok(graph)
    }

    /// Extract field dependencies from a generator
    fn extract_dependencies(generator: &Generator) -> HashSet<String> {
        let mut deps = HashSet::new();

        match generator {
            Generator::Template(template_gen) => {
                // Template generators can reference other fields
                for var in &template_gen.variables {
                    // Variable names in templates reference other fields
                    deps.insert(var.name.clone());

                    // If the variable has a nested generator, extract its deps
                    if let Some(nested_gen) = &var.generator {
                        deps.extend(Self::extract_dependencies(nested_gen));
                    }
                }
            }
            Generator::List(list_gen) => {
                // List generators with discriminators depend on the discriminator field
                if let Some(discriminator) = &list_gen.discriminator {
                    deps.insert(discriminator.clone());
                }
            }
            // Other generator types don't have dependencies on other fields
            _ => {}
        }

        deps
    }

    /// Detect circular dependencies using DFS with color marking
    fn detect_cycles(&self) -> DependencyResult<()> {
        #[derive(PartialEq, Eq)]
        enum Color {
            White, // Not visited
            Gray,  // Being processed (in current DFS path)
            Black, // Fully processed
        }

        let mut colors: HashMap<String, Color> = self
            .fields
            .iter()
            .map(|f| (f.clone(), Color::White))
            .collect();
        let mut path = Vec::new();

        fn visit(
            node: &str,
            colors: &mut HashMap<String, Color>,
            dependencies: &HashMap<String, HashSet<String>>,
            path: &mut Vec<String>,
        ) -> DependencyResult<()> {
            colors.insert(node.to_string(), Color::Gray);
            path.push(node.to_string());

            if let Some(deps) = dependencies.get(node) {
                for dep in deps {
                    match colors.get(dep).unwrap() {
                        Color::White => {
                            visit(dep, colors, dependencies, path)?;
                        }
                        Color::Gray => {
                            // Found a cycle - construct the cycle path
                            let cycle_start = path.iter().position(|f| f == dep).unwrap();
                            let cycle_path: Vec<String> = path[cycle_start..]
                                .iter()
                                .chain(std::iter::once(dep))
                                .cloned()
                                .collect();
                            return Err(DependencyError::CircularDependency {
                                cycle: cycle_path.join(" -> "),
                            });
                        }
                        Color::Black => {
                            // Already processed, skip
                        }
                    }
                }
            }

            colors.insert(node.to_string(), Color::Black);
            path.pop();
            Ok(())
        }

        // Visit all nodes to catch disconnected components
        for field in &self.fields {
            if colors.get(field).unwrap() == &Color::White {
                visit(field, &mut colors, &self.dependencies, &mut path)?;
            }
        }

        Ok(())
    }

    /// Perform topological sort using Kahn's algorithm
    /// Returns fields in generation order (dependencies first)
    pub fn topological_sort(&self) -> DependencyResult<Vec<String>> {
        let mut result = Vec::new();
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut queue = VecDeque::new();

        // Calculate in-degree for each node
        for field in &self.fields {
            let degree = self.dependencies.get(field).map(|d| d.len()).unwrap_or(0);
            in_degree.insert(field.clone(), degree);

            // Add nodes with no dependencies to queue
            if degree == 0 {
                queue.push_back(field.clone());
            }
        }

        // Process queue
        while let Some(field) = queue.pop_front() {
            result.push(field.clone());

            // Reduce in-degree of dependents
            if let Some(deps) = self.dependents.get(&field) {
                for dependent in deps {
                    let degree = in_degree.get_mut(dependent).unwrap();
                    *degree -= 1;

                    if *degree == 0 {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }

        // If we didn't process all nodes, there's a cycle (shouldn't happen after detect_cycles)
        if result.len() != self.fields.len() {
            return Err(DependencyError::CircularDependency {
                cycle: "Cycle detected during topological sort".to_string(),
            });
        }

        Ok(result)
    }

    /// Get direct dependencies of a field
    pub fn get_dependencies(&self, field: &str) -> Option<&HashSet<String>> {
        self.dependencies.get(field)
    }

    /// Get fields that depend on the given field
    pub fn get_dependents(&self, field: &str) -> Option<&HashSet<String>> {
        self.dependents.get(field)
    }

    /// Check if a field has any dependencies
    pub fn has_dependencies(&self, field: &str) -> bool {
        self.dependencies
            .get(field)
            .map(|deps| !deps.is_empty())
            .unwrap_or(false)
    }

    /// Get all fields in the graph
    pub fn fields(&self) -> &HashSet<String> {
        &self.fields
    }

    /// Get the number of fields in the graph
    pub fn field_count(&self) -> usize {
        self.fields.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;

    fn create_field(name: &str, generator: Generator) -> FieldDefinition {
        FieldDefinition {
            name: name.to_string(),
            generator,
            span: None,
        }
    }

    fn create_string_field(name: &str) -> FieldDefinition {
        create_field(name, Generator::String(StringGenerator::default()))
    }

    fn create_template_field(name: &str, variables: Vec<&str>) -> FieldDefinition {
        let template_vars = variables
            .into_iter()
            .map(|v| TemplateVariable {
                name: v.to_string(),
                generator: None,
            })
            .collect();

        create_field(
            name,
            Generator::Template(TemplateGenerator {
                template: "template".to_string(),
                variables: template_vars,
                span: None,
            }),
        )
    }

    fn create_list_field(name: &str, discriminator: Option<&str>) -> FieldDefinition {
        create_field(
            name,
            Generator::List(ListGenerator {
                name: "test_list".to_string(),
                discriminator: discriminator.map(|s| s.to_string()),
                weighted: false,
                mode: ListMode::Random,
                span: None,
            }),
        )
    }

    #[test]
    fn test_empty_document() {
        let doc = DslDocument {
            fields: vec![],
            span: None,
        };

        let result = DependencyGraph::from_document(&doc);
        assert!(matches!(result, Err(DependencyError::EmptyDocument)));
    }

    #[test]
    fn test_no_dependencies() {
        let doc = DslDocument {
            fields: vec![
                create_string_field("field1"),
                create_string_field("field2"),
                create_string_field("field3"),
            ],
            span: None,
        };

        let graph = DependencyGraph::from_document(&doc).unwrap();
        assert_eq!(graph.field_count(), 3);
        assert!(!graph.has_dependencies("field1"));
        assert!(!graph.has_dependencies("field2"));
        assert!(!graph.has_dependencies("field3"));

        let order = graph.topological_sort().unwrap();
        assert_eq!(order.len(), 3);
    }

    #[test]
    fn test_simple_chain() {
        // field3 depends on field2, field2 depends on field1
        let doc = DslDocument {
            fields: vec![
                create_string_field("field1"),
                create_template_field("field2", vec!["field1"]),
                create_template_field("field3", vec!["field2"]),
            ],
            span: None,
        };

        let graph = DependencyGraph::from_document(&doc).unwrap();
        assert_eq!(graph.field_count(), 3);

        // Check dependencies
        assert!(!graph.has_dependencies("field1"));
        assert!(graph.has_dependencies("field2"));
        assert!(graph.has_dependencies("field3"));

        let field2_deps = graph.get_dependencies("field2").unwrap();
        assert!(field2_deps.contains("field1"));

        let field3_deps = graph.get_dependencies("field3").unwrap();
        assert!(field3_deps.contains("field2"));

        // Check topological order
        let order = graph.topological_sort().unwrap();
        let field1_pos = order.iter().position(|f| f == "field1").unwrap();
        let field2_pos = order.iter().position(|f| f == "field2").unwrap();
        let field3_pos = order.iter().position(|f| f == "field3").unwrap();

        assert!(field1_pos < field2_pos);
        assert!(field2_pos < field3_pos);
    }

    #[test]
    fn test_diamond_dependencies() {
        // field4 depends on field2 and field3
        // field2 and field3 both depend on field1
        let doc = DslDocument {
            fields: vec![
                create_string_field("field1"),
                create_template_field("field2", vec!["field1"]),
                create_template_field("field3", vec!["field1"]),
                create_template_field("field4", vec!["field2", "field3"]),
            ],
            span: None,
        };

        let graph = DependencyGraph::from_document(&doc).unwrap();
        assert_eq!(graph.field_count(), 4);

        let order = graph.topological_sort().unwrap();
        let field1_pos = order.iter().position(|f| f == "field1").unwrap();
        let field2_pos = order.iter().position(|f| f == "field2").unwrap();
        let field3_pos = order.iter().position(|f| f == "field3").unwrap();
        let field4_pos = order.iter().position(|f| f == "field4").unwrap();

        // field1 must come first
        assert!(field1_pos < field2_pos);
        assert!(field1_pos < field3_pos);

        // field2 and field3 must come before field4
        assert!(field2_pos < field4_pos);
        assert!(field3_pos < field4_pos);
    }

    #[test]
    fn test_list_discriminator_dependency() {
        let doc = DslDocument {
            fields: vec![
                create_string_field("country"),
                create_list_field("city", Some("country")),
            ],
            span: None,
        };

        let graph = DependencyGraph::from_document(&doc).unwrap();
        assert_eq!(graph.field_count(), 2);

        let city_deps = graph.get_dependencies("city").unwrap();
        assert!(city_deps.contains("country"));

        let order = graph.topological_sort().unwrap();
        let country_pos = order.iter().position(|f| f == "country").unwrap();
        let city_pos = order.iter().position(|f| f == "city").unwrap();
        assert!(country_pos < city_pos);
    }

    #[test]
    fn test_circular_dependency_simple() {
        // field1 depends on field2, field2 depends on field1
        let doc = DslDocument {
            fields: vec![
                create_template_field("field1", vec!["field2"]),
                create_template_field("field2", vec!["field1"]),
            ],
            span: None,
        };

        let result = DependencyGraph::from_document(&doc);
        assert!(matches!(
            result,
            Err(DependencyError::CircularDependency { .. })
        ));

        if let Err(DependencyError::CircularDependency { cycle }) = result {
            assert!(cycle.contains("field1"));
            assert!(cycle.contains("field2"));
        }
    }

    #[test]
    fn test_circular_dependency_complex() {
        // field1 -> field2 -> field3 -> field1 (cycle)
        let doc = DslDocument {
            fields: vec![
                create_template_field("field1", vec!["field2"]),
                create_template_field("field2", vec!["field3"]),
                create_template_field("field3", vec!["field1"]),
            ],
            span: None,
        };

        let result = DependencyGraph::from_document(&doc);
        assert!(matches!(
            result,
            Err(DependencyError::CircularDependency { .. })
        ));
    }

    #[test]
    fn test_self_reference() {
        let doc = DslDocument {
            fields: vec![create_template_field("field1", vec!["field1"])],
            span: None,
        };

        let result = DependencyGraph::from_document(&doc);
        assert!(matches!(result, Err(DependencyError::SelfReference { .. })));
    }

    #[test]
    fn test_undefined_reference() {
        let doc = DslDocument {
            fields: vec![
                create_string_field("field1"),
                create_template_field("field2", vec!["nonexistent"]),
            ],
            span: None,
        };

        let result = DependencyGraph::from_document(&doc);
        assert!(matches!(
            result,
            Err(DependencyError::UndefinedReference { .. })
        ));

        if let Err(DependencyError::UndefinedReference { field, referrer }) = result {
            assert_eq!(field, "nonexistent");
            assert_eq!(referrer, "field2");
        }
    }

    #[test]
    fn test_complex_scenario() {
        // Mix of independent and dependent fields
        let doc = DslDocument {
            fields: vec![
                create_string_field("id"),
                create_string_field("first_name"),
                create_string_field("last_name"),
                create_template_field("full_name", vec!["first_name", "last_name"]),
                create_template_field("email", vec!["first_name", "last_name"]),
                create_list_field("city", None),
                create_template_field("address", vec!["city"]),
            ],
            span: None,
        };

        let graph = DependencyGraph::from_document(&doc).unwrap();
        assert_eq!(graph.field_count(), 7);

        let order = graph.topological_sort().unwrap();
        assert_eq!(order.len(), 7);

        // Verify ordering constraints
        let get_pos = |name: &str| order.iter().position(|f| f == name).unwrap();

        assert!(get_pos("first_name") < get_pos("full_name"));
        assert!(get_pos("last_name") < get_pos("full_name"));
        assert!(get_pos("first_name") < get_pos("email"));
        assert!(get_pos("last_name") < get_pos("email"));
        assert!(get_pos("city") < get_pos("address"));
    }

    #[test]
    fn test_dependents_tracking() {
        let doc = DslDocument {
            fields: vec![
                create_string_field("base"),
                create_template_field("derived1", vec!["base"]),
                create_template_field("derived2", vec!["base"]),
            ],
            span: None,
        };

        let graph = DependencyGraph::from_document(&doc).unwrap();

        let base_dependents = graph.get_dependents("base").unwrap();
        assert_eq!(base_dependents.len(), 2);
        assert!(base_dependents.contains("derived1"));
        assert!(base_dependents.contains("derived2"));
    }

    #[test]
    fn test_multiple_template_variables() {
        let doc = DslDocument {
            fields: vec![
                create_string_field("var1"),
                create_string_field("var2"),
                create_string_field("var3"),
                create_template_field("combined", vec!["var1", "var2", "var3"]),
            ],
            span: None,
        };

        let graph = DependencyGraph::from_document(&doc).unwrap();

        let deps = graph.get_dependencies("combined").unwrap();
        assert_eq!(deps.len(), 3);
        assert!(deps.contains("var1"));
        assert!(deps.contains("var2"));
        assert!(deps.contains("var3"));
    }
}
