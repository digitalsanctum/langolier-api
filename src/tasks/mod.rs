use crate::models::Company;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct CompanyPayload {
    pub company: Company,
}

