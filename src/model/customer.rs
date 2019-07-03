use failure::err_msg;
use failure::Error;
use postgres::rows::Row;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Emails {
    pub primary_email: String,
    pub sales_email: String,
    pub customer_suport: String,
    pub accounting_email: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Costumer {
    #[serde(default)]
    pub id: i32,
    pub company_name: String,
    pub vat_id: String,
    pub address: String,
    pub area: String,
    pub legal_name: String,
    pub website: String,
    pub postcode: i32,
    pub phones: Option<Value>,
    pub email: Option<Value>,
}

pub fn rows_to_struct(row: Row) -> Result<Costumer, Error> {
    let company_name = row.get_opt("company_name");
    let address = row.get_opt("address");
    let area = row.get_opt("area");
    let legal_name = row.get_opt("legal_name");
    let postcode = row.get_opt("postcode");
    let phones = row.get_opt("phones");
    let email = row.get_opt("emails");
    let vat_id = row.get_opt("vat_id");
    let id = row.get_opt("id");
    let website = row.get_opt("website");
    let customer = Costumer {
        company_name: company_name
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(String::from("None")),
        vat_id: vat_id
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(String::from("None")),
        id: id.ok_or(err_msg("KEY_ERROR"))?.unwrap_or(0),
        address: address
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(String::from("None")),
        area: area
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(String::from("None")),
        legal_name: legal_name
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(String::from("None")),
        postcode: postcode.ok_or(err_msg("KEY_ERROR"))?.unwrap_or(0),
        phones: phones
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(Some(json!("{}"))),
        email: email
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(Some(json!("{}"))),
        website: website
            .ok_or(err_msg("KEY_ERROR"))?
            .unwrap_or(String::from("None")),
    };
    Ok(customer)
}

