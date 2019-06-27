use crate::common::connection::PgPool;
use failure::err_msg;
use failure::Error;
use postgres::rows::Row;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Serialize, Deserialize)]
pub struct Emails {
    pub primary_email: String,
    pub sales_email: String,
    pub cucstomer_suport: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub customer_name: String,
    pub msg: String,
}
impl Msg {
    pub fn new(customer: &str, msg: &str) -> Msg {
        Msg {
            customer_name: customer.to_string(),
            msg: msg.to_string(),
        }
    }
}

impl Costumer {
    pub fn new(
        company_name: &str,
        vat_id: &str,
        address: &str,
        area: &str,
        legal_name: &str,
        website: &str,
        postcode: i32,
        phones: &str,
        email: &str,
    ) -> Result<Costumer, Box<Error>> {
        let customer = Costumer {
            id: 0,
            company_name: company_name.to_string(),
            vat_id: vat_id.to_string(),
            address: address.to_string(),
            area: area.to_string(),
            legal_name: legal_name.to_string(),
            postcode: postcode,
            website: website.to_string(),
            phones: Some(json!(phones)),
            email: Some(json!(email)),
        };
        Ok(customer)
    }
}
fn rows_to_struct(row: Row) -> Result<Costumer, Error> {
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
pub fn get_customers(conn: &PgPool) -> Result<Vec<Costumer>, Error> {
    let mut customers = Vec::new();
    let conn = conn.get()?;
    let rows = conn.query("SELECT * FROM customers", &[])?;
    for row in rows.into_iter() {
        let customer = rows_to_struct(row)?;
        customers.push(customer)
    }
    Ok(customers)
}

pub fn get_customer_by_id(conn: &PgPool, id: i32) -> Result<Costumer, Error> {
    let mut customer: Costumer = Default::default();
    let conn = conn.get()?;
    let rows = conn.query("SELECT * FROM customers WHERE id = $1", &[&id])?;
    for row in rows.into_iter() {
        customer = rows_to_struct(row)?;
    }
    Ok(customer)
}

pub fn del_customer_by_id(conn: &PgPool, id: i32) -> Result<Msg, Error> {
    let conn = conn.get()?;
    conn.execute("DELETE FROM customers WHERE id = $1", &[&id])?;
    let customer_name = format!("customer with id {}", id);
    Ok(Msg::new(&customer_name, "deleted was sucess"))
}

pub fn new_customer(conn: &PgPool, customer: &Costumer) -> Result<Msg, Error> {
    let conn = conn.get()?;
    conn.execute("INSERT INTO customers (company_name, vat_id, address, area, legal_name, emails, phones, postcode, website) VALUES ($1, $2,$3,$4,$5,$6,$7,$8,$9)",
                 &[&customer.company_name, &customer.vat_id,&customer.address,&customer.area,&customer.legal_name,&customer.email,&customer.phones,&customer.postcode,&customer.website])?;
    Ok(Msg::new(&customer.company_name, "insert success"))
}

pub fn update_customer(conn: &PgPool, customer:&Costumer,id:i32) -> Result<Msg, Error> {
    let conn = conn.get()?;
    conn.execute("UPDATE customers SET company_name = $1, vat_id=$2, address=$3, area=$4, legal_name=$5, emails=$6, phones=$7, postcode=$8, website=$9 WHERE id = $10",
                 &[&customer.company_name, &customer.vat_id,&customer.address,&customer.area,&customer.legal_name,&customer.email,&customer.phones,&customer.postcode,&customer.website,&id])?;
    Ok(Msg::new(&customer.company_name, "update success"))
}
