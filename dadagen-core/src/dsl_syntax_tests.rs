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
        let input = r#""age": number(min=18, max=99)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""score": number(min=0, max=100, distribution="normal")"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""count": number(min=1, max=1000)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""random_id": number(min=100000, max=999999, seed=42)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
    }

    #[test]
    fn test_float_with_decimal_places() {
        let input = r#""price": number(min=9.99, max=999.99, decimal_places=2)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""price": number(min=9, max=100, decimal_places=2)"#;
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

        let input = r#""gender": name firstname"#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert!(result.is_err(), "name firstname is not valid generator syntax; needs to be name.firstname");

        let input = r#""id" number "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert!(result.is_err(), "it's missing a colon");

        let input = r#""r_uuid" regexgen "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""r_rand1" number between 10000 and 90000 "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""r_str" regexgen "[A-Z][a-zA-Z]{4}[0-9]{4}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""payload_id" template "PERFT_{{id}}_{{r_uuid}}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""gender" gender "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert!(result.is_err(), "missing colon should cause parse error");

        let input = r#""id": counter "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""id": count "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""id": iteration "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""id" rownumber "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""id": sequence "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), false);

        let input = r#""firstname" name givenname}"#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""firstname" name.firstname}"#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""surname_data" name surname "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""surname"  template  " {{surname_data}}-{{r_str}}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""fullname" template "{{firstname}} {{surname}}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""dob" regexgen "19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""email_address" template "TEST_{{firstname}}{{surname}}@noemail.test"  "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);
        
        let input = r#""regex_nesty" regexgen "([A-K]{2}|ABC|BAC)[0-9]"  "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);
        
        let input = r#""choice_with_multiplier" regexgen "([A-K]|LAB){2}"  "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""nino" regexgen "(A|B|C|E|G|H|J|K|L|M|N|O|P|R|S|T|W|X|Y|Z){2}[0-9]{6}A"  "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""street_number" number between 1 and 100 "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""street_name" template "RS Performance Street" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""town" address citytown "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""suburb" address suburb "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""address_line_1" address property "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""street" address property "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);
        
        let input = r#""street" address statecounty "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""street" address postzipcode "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""street" address country "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""postcode" regexgen "[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#"  "initial_investment" number between 10000.00 and 90000.00 "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);
        print!("{:?}", result);

        let input = r#""regular_investment_amount" regexgen "(50|100|150|200|250|300|350|400|450|500|550|600|650|700|750|800|850|900|950)" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""account_number" number between 8800000 and 8899999 "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""sort_code" regexgen "(402205|110124|830608|880011|938424|938343|938130)" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""mobile_phone_number" regexgen "07777 [0-9]{3} [0-9]{3}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""retirement_age" number between 65 and 75 "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);

        let input = r#""simple_gen_template" template "{{gen:sequence}}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);


        let input = r#""complex_template" template "Something : {{gen:sequence}} {{gen:address town}} {{gen:number between 1 and 200}}" "#;
        let result = DslGrammarParser::parse(Rule::dsl, input);
        assert_eq!(result.is_err(), true);


    }
}
