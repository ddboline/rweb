pub use indexmap::{indexmap, IndexMap};
pub use serde_json;
use std::convert::Infallible;
pub use std::{borrow::Cow, clone::Clone, default::Default};
pub use tokio;
pub use warp::http::StatusCode;
use warp::{any, Filter};

pub fn provider<T: Clone + Send + Sync>(
    data: T,
) -> impl Filter<Extract = (T,), Error = Infallible> + Clone {
    any().map(move || data.clone())
}
