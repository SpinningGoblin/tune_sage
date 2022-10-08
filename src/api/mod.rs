pub mod artists;
mod config;
mod general_options;
mod query_operator;
mod remote;

pub use config::Config;
pub use general_options::{GeneralOptions, GeneralOptionsBuilder, GeneralOptionsBuilderError};
pub use query_operator::QueryOperator;
pub use remote::{HttpRemote, Remote};
