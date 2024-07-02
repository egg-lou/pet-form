use serde::Deserialize;

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub search: Option<String>,
}

// #[derive(Deserialize, Debug)]
// pub struct ParamOptions {
//     pub id: String
// }
