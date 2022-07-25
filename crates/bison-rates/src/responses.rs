use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct BisonListResponse {
    pub name: String,
    pub herd: Option<Vec<BisonResponse>>,
}

#[derive(Serialize, Debug)]
pub struct BisonResponse {
    name: String,
    herd_rank: u32,
}
