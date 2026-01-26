//! Tests for the enhanced DSL syntax with new generator types

#[cfg(test)]
mod tests {
    use crate::dsl::{DslGrammarParser, Rule};
    use pest::Parser;

    #[test]
    fn test_string_generator_syntax() {
        // String generator with constraints
        let input = r#""username": string(min_length=5, max_length=15)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""password": string(length=32, charset="hex")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""email_local": string(min_length=3, max_length=20, charset="alphanumeric", case="lower")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""code": string(charset="alpha", case="upper")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_boolean_generator_syntax() {
        let input = r#""is_active": boolean"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""is_premium": bool(true_probability=0.25)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""enabled": bool(true_probability=0.95)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_number_new_syntax() {
        let input = r#""age": integer(min=18, max=99)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""score": number(min=0, max=100, distribution="normal")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""count": int(min=1, max=1000)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""random_id": integer(min=100000, max=999999, seed=42)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_float_with_decimal_places() {
        let input = r#""price": number(min=9.99, max=999.99, decimal_places=2)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""temperature": number(min=-10.5, max=45.5, decimal_places=1)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""rate": number(min=0.001, max=1.0, decimal_places=3)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_datetime_generators() {
        let input = r#""created_at": datetime(start="2020-01-01T00:00:00Z", end="2025-12-31T23:59:59Z")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""birthdate": date(start="1950-01-01", end="2005-12-31")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""appointment": time(start="09:00:00", end="17:00:00")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""timestamp": datetime(start="2024-01-01T00:00:00Z", end="2024-12-31T23:59:59Z", format="%Y-%m-%d %H:%M:%S")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_choice_enum_generators() {
        let input = r#""status": choice("pending", "active", "suspended", "closed")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""color": enum("red", "green", "blue", "yellow")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""priority": enum("low", "medium", "high", "critical")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""size": choice("XS", "S", "M", "L", "XL", "XXL")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_list_generator() {
        let input = r#""city": list(name="cities")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""suburb": list(name="suburbs", discriminator="city")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""skill": list(name="skills", weighted=true)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list(name="firstnames-4000")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""city": list("cities")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""suburb": list("suburbs", discriminator="city")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""suburb": list("suburbs", discriminator="city", mode=sequential)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""suburb": list("suburbs", discriminator="city", mode=random)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""skill": list("skills", weighted=true)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list("firstnames-4000")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list(name="./filename/firstnames-4000.txt")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list(name="/Some/path/filename/firstnames-4000 with spaces.txt")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list(name="C:\\the\\other\\operating-system\\firstnames-4000 with spaces.txt")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list(name="C:\\the\\other\\operating-system\\firstnames-4000 with spaces.txt", mode=random)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list(name="C:\\the\\other\\operating-system\\firstnames-4000 with spaces.txt", mode=sequential)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list("./filename/firstnames-4000.txt")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list("/Some/path/filename/firstnames-4000 with spaces.txt")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": list("C:\\the\\other\\operating-system\\firstnames-4000 with spaces.txt")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        // this should fail at semantic validation as mode=sequential and weighted=true are incompatible
        let input = r#""skill": list(name="skills", weighted=true, mode=sequential)"#;
        let result = crate::dsl::DslSemanticParser::parse(input);
        assert!(result.is_err(), "Expected semantic validation to fail for incompatible weighted+sequential mode");
    }

    #[test]
    fn test_that_should_fail() {
        // New syntax (legacy syntax removed)
        let input = r#""id": number.something"#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        // confirm that parsing fails
        assert!(result.is_err(), "Unexpectedly parsed invalid generator syntax");
        
        let input = r#""rand": number between 10 and 100"#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert!(result.is_err(), "Old syntax is not supported anymore");

        let input = r#""counter": itration"#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert!(result.is_err(), "Typo in generator name should cause parse error");

        let input = r#""gender": nothing.gender"#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert!(result.is_err(), "Wrong word before generator should cause parse error");
    }
}
