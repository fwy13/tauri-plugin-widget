use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Results<T> {
    pub(crate) results: Option<T>,
}