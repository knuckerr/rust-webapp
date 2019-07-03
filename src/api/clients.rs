use crate::model::customer::{Costumer,Emails,rows_to_struct};
use crate::common::connection::PgPool;
use failure::Error;


#[juniper::object]
impl Costumer{
    fn id(&self) -> i32 {
        self.id
    }
    fn companyName(&self) -> &str {
        &self.company_name 
    }
    fn vatId(&self) -> &str {
        &self.vat_id
    }
    fn address(&self) -> &str {
        &self.address
    }
    fn area(&self) -> &str {
        &self.area
    }
    fn legalName(&self) -> &str {
        &self.legal_name
    }
    fn website(&self) -> &str {
        &self.website
    }
    fn postcode(&self) -> i32 {
        self.postcode
    }

}

#[juniper::object]
impl Emails{
    fn primaryEmail(&self) -> &str{
        &self.primary_email
    }
    fn salesEmail(&self) -> &str {
        &self.sales_email
    }
    fn customerSuport(&self) -> &str{
        &self.customer_suport
    }
    fn accountingEmail(&self) -> &str{
        &self.accounting_email
    }

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

