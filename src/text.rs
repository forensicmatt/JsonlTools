use jmespath::Expression;
use serde_json::Value;
use crate::errors::CustomError;
use crate::JMES_RUNTIME;

pub struct TextHandler<'a> {
    expr: Expression<'a>,
    delimiter: String
}

impl <'a>TextHandler<'a> {
    pub fn new(delimiter: String, pattern: String) -> Result<Self, CustomError> {
        let expr = JMES_RUNTIME.compile(
            &pattern
        )?;
        
        Ok( Self {
            expr: expr,
            delimiter: delimiter
        })
    }

    pub fn format_value(&self, value: &Value) -> Result<String, CustomError> {
        let array = self.expr.search(value).expect("Error using Expression")
                         .as_array().expect("Retrun value not array").to_owned();

        let formatted_str = array.iter().map(
            |x| match x.as_string() {
                Some(s) => s.to_owned(),
                None => x.to_string()
            }
        ).collect::<Vec<String>>().join(&self.delimiter);

        Ok(formatted_str)
    }
}