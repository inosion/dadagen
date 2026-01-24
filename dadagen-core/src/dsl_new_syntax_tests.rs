//! Tests for the enhanced DSL syntax with new generator types

#[cfg(test)]
mod tests {
    use crate::dsl::{DslParser, Rule};
    use pest::Parser;

    #[test]
    fn test_string_generator_syntax() {
        // String generator with constraints
        let input = r#"field { "username" string(min_length=5, max_length=15) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "password" string(length=32, charset="hex") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "email_local" string(min_length=3, max_length=20, charset="alphanumeric", case="lower") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "code" string(charset="alpha", case="upper") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_boolean_generator_syntax() {
        let input = r#"field { "is_active" boolean }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "is_premium" bool(true_probability=0.25) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "enabled" bool(true_probability=0.95) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_number_new_syntax() {
        let input = r#"field { "age" integer(min=18, max=99) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "score" number(min=0, max=100, distribution="normal") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "count" int(min=1, max=1000) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "random_id" integer(min=100000, max=999999, seed=42) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_float_with_decimal_places() {
        let input = r#"field { "price" number(min=9.99, max=999.99, decimal_places=2) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "temperature" number(min=-10.5, max=45.5, decimal_places=1) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "rate" number(min=0.001, max=1.0, decimal_places=3) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_datetime_generators() {
        let input = r#"field { "created_at" datetime(start="2020-01-01T00:00:00Z", end="2025-12-31T23:59:59Z") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "birthdate" date(start="1950-01-01", end="2005-12-31") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "appointment" time(start="09:00:00", end="17:00:00") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "timestamp" datetime(start="2024-01-01T00:00:00Z", end="2024-12-31T23:59:59Z", format="%Y-%m-%d %H:%M:%S") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_choice_enum_generators() {
        let input = r#"field { "status" choice("pending", "active", "suspended", "closed") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "color" enum("red", "green", "blue", "yellow") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "priority" oneof("low", "medium", "high", "critical") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "size" choice("XS", "S", "M", "L", "XL", "XXL") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_list_generator() {
        let input = r#"field { "city" list(name="cities") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "suburb" list(name="suburbs", discriminator="city") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "skill" list(name="skills", weighted=true) }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "firstname" list(name="firstnames-4000") }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_backward_compatibility() {
        // Ensure old syntax still works
        let input = r#"field { "id" number }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "rand" number between 10 and 100 }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "counter" iteration }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "gender" gender }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }
}
