#[macro_use]
extern crate lazy_static;

pub mod errors;
pub mod jsonl;
pub mod input;
pub mod text;

use jmespath::Runtime;


lazy_static! {
    pub static ref JMES_RUNTIME: Runtime = {
        let mut runtime = Runtime::new();
        runtime.register_builtin_functions();
        runtime
    };
}