use jmespath::Expression;
use serde_json::Value;
use crate::errors::CustomError;
use crate::JMES_RUNTIME;

pub struct JsonlHandler<'a> {
    filter_list: Vec<Expression<'a>>
}

impl <'a>JsonlHandler<'a> {
    pub fn new(
        filter_queries: Vec<String>
    ) -> Result<Self, CustomError> {
        // Created an array for our compiled expressions
        let mut filter_list = Vec::new();

        // Iterate the string queries
        for filter_str in filter_queries {
            // Compile the string query
            let expr = JMES_RUNTIME.compile(
                &filter_str
            )?;
            filter_list.push(expr);
        }

        Ok(Self {
            filter_list: filter_list
        })
    }

    pub fn pass_json_str(&self, value: &str) -> Result<bool, CustomError> {
        let value = serde_json::from_str(value)?;
        Ok(self.pass(&value))
    }

    pub fn pass(&self, value: &Value) -> bool {
        if self.filter_list.len() == 0 {
            return true;
        }

        for expr in &self.filter_list {
            let result = expr.search(value).expect("Error using Expression")
                             .as_boolean().expect("Retrun value not bool");

            if result {
                return true;
            }
        }

        false
    }
}
