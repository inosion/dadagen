use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "dsl.pest"]
pub struct DslGrammarParser;

impl DslGrammarParser {
    pub fn parse(
        rule: Rule,
        input: &str,
    ) -> Result<pest::iterators::Pairs<Rule>, pest::error::Error<Rule>> {
        <Self as pest::Parser<Rule>>::parse(rule, input)
    }
}

pub use pest::Parser;
// No alias: prefer explicit names. Use `DslGrammarParser` for grammar-level parsing
// and `DslSemanticParser` for semantic/AST parsing.

/// Thin semantic parser wrapper that produces an AST and runs validation.
pub struct DslSemanticParser;

impl DslSemanticParser {
    pub fn parse(input: &str) -> crate::ast::AstResult<crate::ast::DslDocument> {
        crate::parser::parse_dsl(input)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use pest::Parser;

    #[test]
    fn test_dsl_parser() {
        
        let input = r#""id": number"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""r_uuid": regexgen "[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""r_rand1": number(min=10000, max=90000)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""r_str": regexgen "[A-Z][a-zA-Z]{4}[0-9]{4}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""payload_id": template "PERFT_{{id}}_{{r_uuid}}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""gender": gender"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""id": sequence"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        // Only `sequence` is supported now (deprecated aliases removed)
        let input = r#""id": sequence"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""firstname": name givenname"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""surname_data": name surname"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""surname": template  " {{surname_data}}-{{r_str}}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""fullname": template "{{firstname}} {{surname}}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""dob": regexgen "19[3-9][0-9]-(1[012]|0[1-9])-(0[0-9]|1[0-9]|2[0-9])""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""email_address": template "TEST_{{firstname}}{{surname}}@noemail.test""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        
        let input = r#""regex_nesty": regexgen "([A-K]{2}|ABC|BAC)[0-9]""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        
        let input = r#""choice_with_multiplier": regexgen "([A-K]|LAB){2}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""nino": regexgen "(A|B|C|E|G|H|J|K|L|M|N|O|P|R|S|T|W|X|Y|Z){2}[0-9]{6}A""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""street_number": number(min=1, max=100)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""street_name": template "RS Performance Street""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""town": address citytown"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""suburb": address suburb"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""address_line_1": address property"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""street": address property"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        
        let input = r#""street": address statecounty"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""street": address postzipcode"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""street": address country"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""postcode": regexgen "[A-Z][A-Z][0-9] [0-9][A-Z][A-Z]""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#"  "initial_investment": number(min=10000.00, max=90000.00)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);
        print!("{:?}", result);

        let input = r#""regular_investment_amount": regexgen "(50|100|150|200|250|300|350|400|450|500|550|600|650|700|750|800|850|900|950)""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""account_number": number(min=8800000, max=8899999)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""sort_code": regexgen "(402205|110124|830608|880011|938424|938343|938130)""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""mobile_phone_number": regexgen "07777 [0-9]{3} [0-9]{3}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""retirement_age": number(min=65, max=75)"#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

        let input = r#""simple_gen_template": template "{{gen:sequence}}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);


        let input = r#""complex_template": template "Something : {{gen:sequence}} {{gen:address town}} {{gen:number(min=1, max=200)}}""#;
        let mut result = DslGrammarParser::parse(Rule::dsl, input).unwrap();
        assert_eq!(result.next().unwrap().as_span().as_str(), input);

    }
}