

#[derive(Parser)] 
#[grammar = "dsl.pest"]
pub struct DslParser;
#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_dsl_parser() {
        
        let input = r#"field { "id" number }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "r_uuid" regexgen "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "r_rand1" number between 10000 and 90000 }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "r_str" regexgen "[A-Z][a-zA-Z]{4}[0-9]{4}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "payload_id" template "PERFT_${id}_${r_uuid}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "gender" gender }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "id" counter }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "id" count }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "id" iteration }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "id" rownumber }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "firstname" name givenname} "#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "surname_data" name surname }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "surname"  template  " ${surname_data}-${r_str}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "fullname" template "${firstname} ${surname}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "dob" regexgen "19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "email_address" template "TEST_${firstname}${surname}@noemail.test"  }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        
        let input = r#"field { "regex_nesty" regexgen "([A-K]{2}|ABC|BAC)[0-9]"  }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        
        let input = r#"field { "choice_with_multiplier" regexgen "([A-K]|LAB){2}"  }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "nino" regexgen "(A|B|C|E|G|H|J|K|L|M|N|O|P|R|S|T|W|X|Y|Z){2}[0-9]{6}A"  }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "street_number" number between 1 and 100 }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "street_name" template "RS Performance Street" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "town" address citytown }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "suburb" address suburb }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "address_line_1" address property }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "street" address property }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        
        let input = r#"field { "street" address statecounty }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "street" address postzipcode }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "street" address country }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "postcode" regexgen "[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"  field { "initial_investment" number between 10000.00 and 90000.00 }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        print!("{:?}", result);

        let input = r#"field { "regular_investment_amount" regexgen "(50|100|150|200|250|300|350|400|450|500|550|600|650|700|750|800|850|900|950)" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "account_number" number between 8800000 and 8899999 }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "sort_code" regexgen "(402205|110124|830608|880011|938424|938343|938130)" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "mobile_phone_number" regexgen "07777 [0-9]{3} [0-9]{3}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "retirement_age" number between 65 and 75 }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"field { "simple_gen_template" template "${gen:counter}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);


        let input = r#"field { "complex_template" template "Something : ${gen:counter} ${gen:address town} ${gen:number between 1 and 200}" }"#;
        let mut result = DslParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

    }
}