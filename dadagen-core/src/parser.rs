//! Parser module for converting Pest parse tree to AST
//!
//! This module contains the logic to transform the Pest parse tree
//! into a strongly-typed AST structure with proper error handling.

use crate::ast::*;
use crate::dsl::{DslGrammarParser, Rule};
use pest::iterators::Pair;

/// Parse a DSL string into an AST document
pub fn parse_dsl(input: &str) -> AstResult<DslDocument> {
    let mut pairs =
        DslGrammarParser::parse(Rule::dsl, input).map_err(|e| AstError::InvalidValue {
            message: format!("Parse error: {}", e),
            span: None,
        })?;

    let mut fields = Vec::new();

    // The dsl rule contains either schema_block or field_list
    if let Some(dsl_pair) = pairs.next() {
        for pair in dsl_pair.into_inner() {
            match pair.as_rule() {
                Rule::schema_block => {
                    // Parse schema block: skip schema_name, get field_expr+
                    for inner_pair in pair.into_inner() {
                        match inner_pair.as_rule() {
                            Rule::field_expr => {
                                fields.push(parse_field(inner_pair)?);
                            }
                            Rule::schema_name => {
                                // Store schema name if needed in future
                            }
                            _ => {}
                        }
                    }
                }
                Rule::field_list => {
                    // Parse field list: get field_expr+
                    for inner_pair in pair.into_inner() {
                        if inner_pair.as_rule() == Rule::field_expr {
                            fields.push(parse_field(inner_pair)?);
                        }
                    }
                }
                Rule::field_expr => {
                    // Direct field (shouldn't happen with current grammar but handle anyway)
                    fields.push(parse_field(pair)?);
                }
                Rule::END => break,
                _ => {}
            }
        }
    }

    let doc = DslDocument { fields, span: None };

    // Validate the document
    doc.validate()?;

    Ok(doc)
}

fn parse_field(pair: Pair<Rule>) -> AstResult<FieldDefinition> {
    let span = create_span(&pair);
    let mut name = String::new();
    let mut generator = None;

    // `pair` may contain different sub-rules depending on grammar variations:
    // - named_field (colon form)
    // - hidden_field
    // - field_block (legacy `field { "name" generator }`)
    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::named_field => {
                // named_field contains a quoted field_name and a generator inside
                for p in inner_pair.into_inner() {
                    match p.as_rule() {
                        Rule::field_name => {
                            name = p.as_str().to_string();
                        }
                        _ => {
                            generator = Some(parse_generator(p)?);
                        }
                    }
                }
            }
            Rule::hidden_field => {
                // Hidden seed: produce a synthetic name for now and parse generator
                // Name will be empty and treated as hidden by later AST updates.
                let mut idx = 0usize;
                for _ in 0..1 {
                    idx += 1;
                }
                name = format!("__hidden_{}", idx);
                for p in inner_pair.into_inner() {
                    generator = Some(parse_generator(p)?);
                }
            }
            Rule::field_name => {
                name = inner_pair.as_str().to_string();
            }
            _ => {
                // Fallback: attempt to parse generator if present
                generator = Some(parse_generator(inner_pair)?);
            }
        }
    }

    Ok(FieldDefinition {
        name,
        generator: generator.ok_or_else(|| AstError::InvalidValue {
            message: "Field must have a generator".to_string(),
            span: Some(span.clone()),
        })?,
        span: Some(span),
    })
}

fn parse_generator(pair: Pair<Rule>) -> AstResult<Generator> {
    let span = create_span(&pair);

    match pair.as_rule() {
        Rule::concat_expr => {
            // Build a TemplateGenerator by concatenating term parts. Terms may be
            // implicit templates (quoted strings with placeholders) or generator terms.
            let span = create_span(&pair);
            let mut template = String::new();
            let mut variables: Vec<TemplateVariable> = Vec::new();
            let mut var_idx = 0usize;
            let mut terms: Vec<Pair<Rule>> = pair.into_inner().collect();
            // If only a single term, delegate to parsing that term directly
            if terms.len() == 1 {
                return parse_generator(terms.remove(0));
            }

            for term_pair in terms.into_iter() {
                match term_pair.as_rule() {
                    _ => {
                        // Other generator term: treat as embedded variable with generated sub-generator
                        var_idx += 1;
                        let var_name = format!("__g{}", var_idx);
                        template.push_str(&format!("{{{{{}}}}}", var_name));
                        let generator = parse_generator(term_pair)?;
                        variables.push(TemplateVariable {
                            name: var_name.clone(),
                            generator: Some(Box::new(generator)),
                        });
                    }
                }
            }

            let tg = TemplateGenerator {
                template,
                variables,
                span: Some(span),
            };
            return Ok(Generator::Template(tg));
        }
        Rule::string_generator => Ok(Generator::String(parse_string_generator(pair)?)),
        Rule::boolean_generator => Ok(Generator::Boolean(parse_boolean_generator(pair)?)),
        Rule::number_generator => Ok(Generator::Number(parse_number_generator(pair)?)),
        Rule::double_number_generator => {
            Ok(Generator::Number(parse_double_number_generator(pair)?))
        }
        Rule::datetime_generator => Ok(Generator::DateTime(parse_datetime_generator(pair)?)),
        Rule::date_generator => Ok(Generator::Date(parse_date_generator(pair)?)),
        Rule::time_generator => Ok(Generator::Time(parse_time_generator(pair)?)),
        Rule::choice_generator => Ok(Generator::Choice(parse_choice_generator(pair)?)),
        Rule::list_generator => Ok(Generator::List(parse_list_generator(pair)?)),
        Rule::template_generator => Ok(Generator::Template(parse_template_generator(pair)?)),
        Rule::regexgen_generator => Ok(Generator::Regex(parse_regex_generator(pair)?)),
        Rule::sequence_generator => Ok(Generator::Counter(parse_counter_generator(pair)?)),
        Rule::gender_generator => Ok(Generator::Gender(parse_gender_generator(pair)?)),
        Rule::name_generator => Ok(Generator::Name(parse_name_generator(pair)?)),
        Rule::address_generator => Ok(Generator::Address(parse_address_generator(pair)?)),
        Rule::base_generator => {
            let mut inner = pair.into_inner();
            if let Some(inner_pair) = inner.next() {
                parse_generator(inner_pair)
            } else {
                Err(AstError::InvalidValue {
                    message: "Empty base_generator".to_string(),
                    span: Some(span),
                })
            }
        }
        _ => Err(AstError::InvalidValue {
            message: format!("Unknown generator type: {:?}", pair.as_rule()),
            span: Some(span),
        }),
    }
}

fn parse_string_generator(pair: Pair<Rule>) -> AstResult<StringGenerator> {
    let span = create_span(&pair);
    let mut generator = StringGenerator {
        span: Some(span),
        ..Default::default()
    };

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::string_constraints => {
                parse_string_constraints(&mut generator, inner_pair)?;
            }
            _ => {}
        }
    }

    generator.validate()?;
    Ok(generator)
}

fn parse_string_constraints(generator: &mut StringGenerator, pair: Pair<Rule>) -> AstResult<()> {
    for constraint_pair in pair.into_inner() {
        match constraint_pair.as_rule() {
            Rule::string_length_constraint => {
                let value = extract_number_value(constraint_pair)?;
                generator.length = Some(value as usize);
            }
            Rule::string_min_length_constraint => {
                let value = extract_number_value(constraint_pair)?;
                generator.min_length = Some(value as usize);
            }
            Rule::string_max_length_constraint => {
                let value = extract_number_value(constraint_pair)?;
                generator.max_length = Some(value as usize);
            }
            Rule::string_charset_constraint => {
                generator.charset = extract_charset_value(constraint_pair)?;
            }
            Rule::string_case_constraint => {
                generator.case = extract_case_value(constraint_pair)?;
            }
            Rule::string_pattern_constraint => {
                generator.pattern = Some(extract_string_value(constraint_pair)?);
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_boolean_generator(pair: Pair<Rule>) -> AstResult<BooleanGenerator> {
    let span = create_span(&pair);
    let mut generator = BooleanGenerator {
        span: Some(span),
        ..Default::default()
    };

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::boolean_constraints {
            for constraint_pair in inner_pair.into_inner() {
                if constraint_pair.as_rule() == Rule::boolean_true_probability {
                    generator.true_probability = extract_float_value(constraint_pair)?;
                }
            }
        }
    }

    generator.validate()?;
    Ok(generator)
}

fn parse_number_generator(pair: Pair<Rule>) -> AstResult<NumberGenerator> {
    let span = create_span(&pair);
    let mut generator = NumberGenerator {
        span: Some(span),
        ..Default::default()
    };

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::number_constraints {
            parse_number_constraints(&mut generator, inner_pair)?;
        }
    }

    generator.validate()?;
    Ok(generator)
}

fn parse_double_number_generator(pair: Pair<Rule>) -> AstResult<NumberGenerator> {
    let span = create_span(&pair);
    let mut generator = NumberGenerator {
        span: Some(span),
        ..Default::default()
    };

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::double_constraints {
            parse_double_constraints(&mut generator, inner_pair)?;
        }
    }

    generator.validate()?;
    Ok(generator)
}

fn parse_number_constraints(generator: &mut NumberGenerator, pair: Pair<Rule>) -> AstResult<()> {
    for constraint_pair in pair.into_inner() {
        match constraint_pair.as_rule() {
            Rule::number_min_constraint => {
                generator.min = Some(extract_float_value(constraint_pair)?);
            }
            Rule::number_max_constraint => {
                generator.max = Some(extract_float_value(constraint_pair)?);
            }
            Rule::number_decimal_places_constraint => {
                let value = extract_number_value(constraint_pair)?;
                generator.decimal_places = Some(value as usize);
            }
            Rule::number_seed_constraint => {
                let value = extract_number_value(constraint_pair)?;
                generator.seed = Some(value as u64);
            }
            Rule::number_distribution_constraint => {
                generator.distribution = extract_distribution_value(constraint_pair)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_double_constraints(generator: &mut NumberGenerator, pair: Pair<Rule>) -> AstResult<()> {
    for constraint_pair in pair.into_inner() {
        match constraint_pair.as_rule() {
            Rule::double_min_constraint => {
                generator.min = Some(extract_float_value(constraint_pair)?);
            }
            Rule::double_max_constraint => {
                generator.max = Some(extract_float_value(constraint_pair)?);
            }
            Rule::double_decimal_places_constraint => {
                let value = extract_number_value(constraint_pair)?;
                generator.decimal_places = Some(value as usize);
            }
            Rule::number_seed_constraint => {
                let value = extract_number_value(constraint_pair)?;
                generator.seed = Some(value as u64);
            }
            Rule::number_distribution_constraint => {
                generator.distribution = extract_distribution_value(constraint_pair)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn parse_datetime_generator(pair: Pair<Rule>) -> AstResult<DateTimeGenerator> {
    let span = create_span(&pair);
    let mut generator = DateTimeGenerator {
        span: Some(span),
        ..Default::default()
    };

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::datetime_constraints {
            for constraint_pair in inner_pair.into_inner() {
                match constraint_pair.as_rule() {
                    Rule::datetime_start_constraint => {
                        generator.start = Some(extract_string_value(constraint_pair)?);
                    }
                    Rule::datetime_end_constraint => {
                        generator.end = Some(extract_string_value(constraint_pair)?);
                    }
                    Rule::datetime_format_constraint => {
                        generator.format = Some(extract_string_value(constraint_pair)?);
                    }
                    _ => {}
                }
            }
        }
    }

    generator.validate()?;
    Ok(generator)
}

fn parse_date_generator(pair: Pair<Rule>) -> AstResult<DateGenerator> {
    let span = create_span(&pair);
    let mut generator = DateGenerator {
        span: Some(span),
        ..Default::default()
    };

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::datetime_constraints {
            for constraint_pair in inner_pair.into_inner() {
                match constraint_pair.as_rule() {
                    Rule::datetime_start_constraint => {
                        generator.start = Some(extract_string_value(constraint_pair)?);
                    }
                    Rule::datetime_end_constraint => {
                        generator.end = Some(extract_string_value(constraint_pair)?);
                    }
                    Rule::datetime_format_constraint => {
                        generator.format = Some(extract_string_value(constraint_pair)?);
                    }
                    _ => {}
                }
            }
        }
    }

    generator.validate()?;
    Ok(generator)
}

fn parse_time_generator(pair: Pair<Rule>) -> AstResult<TimeGenerator> {
    let span = create_span(&pair);
    let mut generator = TimeGenerator {
        span: Some(span),
        ..Default::default()
    };

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::datetime_constraints {
            for constraint_pair in inner_pair.into_inner() {
                match constraint_pair.as_rule() {
                    Rule::datetime_start_constraint => {
                        generator.start = Some(extract_string_value(constraint_pair)?);
                    }
                    Rule::datetime_end_constraint => {
                        generator.end = Some(extract_string_value(constraint_pair)?);
                    }
                    Rule::datetime_format_constraint => {
                        generator.format = Some(extract_string_value(constraint_pair)?);
                    }
                    _ => {}
                }
            }
        }
    }

    generator.validate()?;
    Ok(generator)
}

fn parse_choice_generator(pair: Pair<Rule>) -> AstResult<ChoiceGenerator> {
    let span = create_span(&pair);
    let mut options = Vec::new();

    for inner_pair in pair.into_inner() {
        if inner_pair.as_rule() == Rule::choice_options {
            for option_pair in inner_pair.into_inner() {
                if option_pair.as_rule() == Rule::choice_option {
                    options.push(extract_choice_value(option_pair)?);
                }
            }
        }
    }

    let generator = ChoiceGenerator {
        options,
        span: Some(span),
    };

    generator.validate()?;
    Ok(generator)
}

fn parse_list_generator(pair: Pair<Rule>) -> AstResult<ListGenerator> {
    let span = create_span(&pair);
    let mut name = String::new();
    let mut discriminator = None;
    let mut weighted = false;
    let mut mode: crate::ast::ListMode = crate::ast::ListMode::Random;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::list_constraints => {
                for constraint_pair in inner_pair.into_inner() {
                    match constraint_pair.as_rule() {
                        Rule::list_name_constraint => {
                            name = extract_list_name_value(constraint_pair)?;
                        }
                        Rule::list_discriminator_constraint => {
                            discriminator = Some(extract_string_value(constraint_pair)?);
                        }
                        Rule::list_weighted_constraint => {
                            weighted = extract_bool_value(constraint_pair)?;
                        }
                        Rule::list_mode_constraint => {
                            // Parse mode token from the constraint text
                            let s = constraint_pair.as_str();
                            if s.contains("random") {
                                mode = crate::ast::ListMode::Random;
                            } else if s.contains("sequential") {
                                mode = crate::ast::ListMode::Sequential;
                            }
                        }
                        _ => {}
                    }
                }
            }
            Rule::list_name_value => {
                // positional quoted form: list("name") => list_name_value appears as child
                name = inner_pair.as_str().to_string();
            }
            _ => {}
        }
    }

    // Semantic validation: weighted lists cannot be sequential
    if weighted && mode == crate::ast::ListMode::Sequential {
        return Err(AstError::InvalidValue {
            message: "Invalid list configuration: weighted lists cannot use mode=sequential"
                .to_string(),
            span: Some(span.clone()),
        });
    }

    let generator = ListGenerator {
        name,
        discriminator,
        weighted,
        mode,
        span: Some(span),
    };

    generator.validate()?;
    Ok(generator)
}

fn parse_template_generator(pair: Pair<Rule>) -> AstResult<TemplateGenerator> {
    let span = create_span(&pair);
    let mut template = String::new();
    let mut variables = Vec::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::template_variable => {
                template.push_str(inner_pair.as_str());
                // Extract the variable name from {{variable_name}}
                let var_content = inner_pair.as_str();
                if let Some(var_name) = var_content
                    .strip_prefix("{{")
                    .and_then(|s| s.strip_suffix("}}"))
                {
                    variables.push(TemplateVariable {
                        name: var_name.to_string(),
                        generator: None,
                    });
                }
            }
            Rule::template_non_var_characters => {
                template.push_str(inner_pair.as_str());
            }
            _ => {}
        }
    }

    let generator = TemplateGenerator {
        template,
        variables,
        span: Some(span),
    };

    generator.validate()?;
    Ok(generator)
}

fn parse_regex_generator(pair: Pair<Rule>) -> AstResult<RegexGenerator> {
    let span = create_span(&pair);
    let mut pattern = String::new();

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::regexgen_range_expr
            | Rule::regexgen_multiselect_expr
            | Rule::regexgen_non_expr_characters => {
                pattern.push_str(inner_pair.as_str());
            }
            _ => {}
        }
    }

    let generator = RegexGenerator {
        pattern,
        span: Some(span),
    };

    generator.validate()?;
    Ok(generator)
}

fn parse_counter_generator(pair: Pair<Rule>) -> AstResult<CounterGenerator> {
    let span = create_span(&pair);
    let generator = CounterGenerator {
        span: Some(span),
        ..Default::default()
    };

    generator.validate()?;
    Ok(generator)
}

fn parse_gender_generator(pair: Pair<Rule>) -> AstResult<GenderGenerator> {
    let span = create_span(&pair);
    let generator = GenderGenerator { span: Some(span) };

    generator.validate()?;
    Ok(generator)
}

fn parse_name_generator(pair: Pair<Rule>) -> AstResult<NameGenerator> {
    let span = create_span(&pair);
    let mut name_type = NameType::Full;

    for inner_pair in pair.into_inner() {
        match inner_pair.as_rule() {
            Rule::given_name_label => name_type = NameType::GivenName,
            Rule::surname_label => name_type = NameType::Surname,
            _ => {}
        }
    }

    let generator = NameGenerator {
        name_type,
        span: Some(span),
    };

    generator.validate()?;
    Ok(generator)
}

fn parse_address_generator(pair: Pair<Rule>) -> AstResult<AddressGenerator> {
    let span = create_span(&pair);
    let mut component = AddressComponent::CityTown;

    for inner_pair in pair.into_inner() {
        component = match inner_pair.as_rule() {
            Rule::citytown_label => AddressComponent::CityTown,
            Rule::suburb_label => AddressComponent::Suburb,
            Rule::street_label => AddressComponent::Street,
            Rule::property_label => AddressComponent::Property,
            Rule::postzipcode_label => AddressComponent::PostZipCode,
            Rule::statecounty_label => AddressComponent::StateCounty,
            Rule::country_label => AddressComponent::Country,
            _ => continue,
        };
    }

    let generator = AddressGenerator {
        component,
        span: Some(span),
    };

    generator.validate()?;
    Ok(generator)
}

// Helper functions for extracting values from Pest pairs

fn create_span(pair: &Pair<Rule>) -> Span {
    let (line, col) = pair.line_col();
    Span::new(pair.as_span().start(), pair.as_span().end(), line, col)
}

fn extract_number_value(pair: Pair<Rule>) -> AstResult<i64> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::number {
            return inner.as_str().parse().map_err(|_| AstError::InvalidValue {
                message: format!("Invalid number: {}", inner.as_str()),
                span: Some(create_span(&inner)),
            });
        }
    }
    Err(AstError::InvalidValue {
        message: "No number value found".to_string(),
        span: None,
    })
}

fn extract_float_value(pair: Pair<Rule>) -> AstResult<f64> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::number | Rule::double_number => {
                return inner.as_str().parse().map_err(|_| AstError::InvalidValue {
                    message: format!("Invalid number: {}", inner.as_str()),
                    span: Some(create_span(&inner)),
                });
            }
            _ => {}
        }
    }
    Err(AstError::InvalidValue {
        message: "No number value found".to_string(),
        span: None,
    })
}

fn extract_string_value(pair: Pair<Rule>) -> AstResult<String> {
    for inner in pair.into_inner() {
        match inner.as_rule() {
            Rule::pattern_string
            | Rule::datetime_value
            | Rule::format_string
            | Rule::field_name => {
                return Ok(inner.as_str().to_string());
            }
            _ => {}
        }
    }
    Err(AstError::InvalidValue {
        message: "No string value found".to_string(),
        span: None,
    })
}

fn extract_charset_value(pair: Pair<Rule>) -> AstResult<CharacterSet> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::charset_type {
            return match inner.as_str() {
                "alpha" => Ok(CharacterSet::Alpha),
                "numeric" => Ok(CharacterSet::Numeric),
                "alphanumeric" => Ok(CharacterSet::AlphaNumeric),
                "ascii" => Ok(CharacterSet::Ascii),
                "hex" => Ok(CharacterSet::Hex),
                "base64" => Ok(CharacterSet::Base64),
                s => Err(AstError::InvalidValue {
                    message: format!("Unknown charset: {}", s),
                    span: Some(create_span(&inner)),
                }),
            };
        }
    }
    Err(AstError::InvalidValue {
        message: "No charset value found".to_string(),
        span: None,
    })
}

fn extract_case_value(pair: Pair<Rule>) -> AstResult<Case> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::case_type {
            return match inner.as_str() {
                "lower" => Ok(Case::Lower),
                "upper" => Ok(Case::Upper),
                "title" => Ok(Case::Title),
                "mixed" => Ok(Case::Mixed),
                s => Err(AstError::InvalidValue {
                    message: format!("Unknown case: {}", s),
                    span: Some(create_span(&inner)),
                }),
            };
        }
    }
    Err(AstError::InvalidValue {
        message: "No case value found".to_string(),
        span: None,
    })
}

fn extract_distribution_value(pair: Pair<Rule>) -> AstResult<Distribution> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::distribution_type {
            return match inner.as_str() {
                "uniform" => Ok(Distribution::Uniform),
                "normal" => Ok(Distribution::Normal {
                    mean: 0.0,
                    std_dev: 1.0,
                }),
                "exponential" => Ok(Distribution::Exponential { lambda: 1.0 }),
                s => Err(AstError::InvalidValue {
                    message: format!("Unknown distribution: {}", s),
                    span: Some(create_span(&inner)),
                }),
            };
        }
    }
    Err(AstError::InvalidValue {
        message: "No distribution value found".to_string(),
        span: None,
    })
}

fn extract_choice_value(pair: Pair<Rule>) -> AstResult<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::choice_value {
            return Ok(inner.as_str().to_string());
        }
    }
    Err(AstError::InvalidValue {
        message: "No choice value found".to_string(),
        span: None,
    })
}

fn extract_list_name_value(pair: Pair<Rule>) -> AstResult<String> {
    for inner in pair.into_inner() {
        if inner.as_rule() == Rule::list_name_value {
            return Ok(inner.as_str().to_string());
        }
    }
    Err(AstError::InvalidValue {
        message: "No list name value found".to_string(),
        span: None,
    })
}

fn extract_bool_value(pair: Pair<Rule>) -> AstResult<bool> {
    for inner in pair.into_inner() {
        let text = inner.as_str();
        if text == "true" || text == "false" {
            return Ok(text == "true");
        }
    }
    Err(AstError::InvalidValue {
        message: "No boolean value found".to_string(),
        span: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_field() {
        let input = r#""test": boolean"#;
        let doc = parse_dsl(input).unwrap();

        assert_eq!(doc.fields.len(), 1);
        assert_eq!(doc.fields[0].name, "test");
        assert!(matches!(doc.fields[0].generator, Generator::Boolean(_)));
    }

    #[test]
    fn test_parse_string_with_constraints() {
        let input = r#""username": string(min_length=5, max_length=15, charset="alphanumeric")"#;
        let doc = parse_dsl(input).unwrap();

        assert_eq!(doc.fields.len(), 1);
        if let Generator::String(generator) = &doc.fields[0].generator {
            assert_eq!(generator.min_length, Some(5));
            assert_eq!(generator.max_length, Some(15));
            assert_eq!(generator.charset, CharacterSet::AlphaNumeric);
        } else {
            panic!("Expected string generator");
        }
    }

    #[test]
    fn test_parse_number_with_constraints() {
        let input = r#""age": number(min=18, max=99)"#;
        let doc = parse_dsl(input).unwrap();

        assert_eq!(doc.fields.len(), 1);
        if let Generator::Number(generator) = &doc.fields[0].generator {
            assert_eq!(generator.min, Some(18.0));
            assert_eq!(generator.max, Some(99.0));
        } else {
            panic!("Expected number generator");
        }
    }

    #[test]
    fn test_parse_choice_generator() {
        let input = r#""status": choice("pending", "active", "closed")"#;
        let doc = parse_dsl(input).unwrap();

        assert_eq!(doc.fields.len(), 1);
        if let Generator::Choice(generator) = &doc.fields[0].generator {
            assert_eq!(generator.options.len(), 3);
            assert_eq!(generator.options[0], "pending");
            assert_eq!(generator.options[1], "active");
            assert_eq!(generator.options[2], "closed");
        } else {
            panic!("Expected choice generator");
        }
    }

    #[test]
    fn test_parse_list_generator() {
        let input = r#""city": list(name="cities", discriminator="country")"#;
        let doc = parse_dsl(input).unwrap();

        assert_eq!(doc.fields.len(), 1);
        if let Generator::List(generator) = &doc.fields[0].generator {
            assert_eq!(generator.name, "cities");
            assert_eq!(generator.discriminator, Some("country".to_string()));
        } else {
            panic!("Expected list generator");
        }
    }

    #[test]
    fn test_parse_multiple_fields() {
        let input = r#"
            "id": sequence,
            "name": string(min_length=3, max_length=20),
            "age": number(min=18, max=99)
        "#;
        let doc = parse_dsl(input).unwrap();

        assert_eq!(doc.fields.len(), 3);
        assert_eq!(doc.fields[0].name, "id");
        assert_eq!(doc.fields[1].name, "name");
        assert_eq!(doc.fields[2].name, "age");
    }

    #[test]
    fn test_validation_duplicate_fields() {
        let input = r#"
            "id": sequence
            "id": gender
        "#;
        let result = parse_dsl(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_invalid_constraints() {
        let input = r#""test": string(min_length=20, max_length=10)"#;
        let result = parse_dsl(input);
        assert!(result.is_err());
    }
}
