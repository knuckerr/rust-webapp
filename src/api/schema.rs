use crate::common::connection::PgPool;
use crate::model::customer::{rows_to_struct, Costumer};
use juniper::{Context, FieldResult, RootNode};
use crate::api;

//for the graphql creates context
#[derive(Debug, Clone)]
pub struct GContext {
    pub pool: PgPool,
}
//impl the context here
impl Context for GContext {}

pub struct QueryRoot();

#[juniper::object(
    Context = GContext,
)]
impl QueryRoot {
    fn apiVersion(context: &GContext) -> &str {
        "1.0"
    }
    fn customers(context: &GContext) -> FieldResult<Vec<Costumer>> {
        let mut customers = Vec::new();
        let conn = context.pool.get()?;
        let rows = conn.query("SELECT * FROM customers", &[])?;
        for row in rows.into_iter() {
            let customer = rows_to_struct(row)?;
            customers.push(customer)
        }
        Ok(customers)
    }
    fn customer(context: &GContext, id: i32) -> FieldResult<Costumer> {
        let customer = api::clients::get_customer_by_id(&context.pool,id)?;
        Ok(customer)
    }
}

pub struct MutationRoot;

#[juniper::object(
    Context = GContext,
)]
impl MutationRoot {
    pub fn delCustomer(context: &GContext, id: i32) -> FieldResult<String> {
        let conn = context.pool.get()?;
        let customer = api::clients::get_customer_by_id(&context.pool,id)?;
        conn.execute("DELETE FROM customers WHERE id = $1", &[&id])?;
        let customer_name = format!("customer with id {}", id);
        let msg = format!("Customer with id = {} was deleted", id);
        Ok(msg)
    }
}
    /*
    fn createcustomer(context: &GContext,input:Newcostumer) -> FieldResult<Costumer> {
        let emails:Value = serde_json::from_str(&input.emails)?;
        let phones:Value = serde_json::from_str(&input.phones)?;
        let conn = context.pool.get()?;
        let id:i32 = conn.query("INSERT INTO customers (company_name, vat_id, address, area, legal_name, emails, phones, postcode, website) VALUES ($1, $2,$3,$4,$5,$6,$7,$8,$9) RETURNING id",
                     &[&input.company_name, &input.vat_id,&input.address,&input.area,&input.legal_name,&emails,&phones,&input.postcode,&input.website])?
            .iter().next().unwrap().get(0);
        let customer = api::clients::get_customer_by_id(&context.pool,id)?;
        Ok(customer)
    }
}
*/
/*
pub fn update_customer(conn: &PgPool, customer: &Costumer, id: i32) -> Result<Msg, Error> {
    let conn = conn.get()?;
    conn.execute("UPDATE customers SET company_name = $1, vat_id=$2, address=$3, area=$4, legal_name=$5, emails=$6, phones=$7, postcode=$8, website=$9 WHERE id = $10",
                 &[&customer.company_name, &customer.vat_id,&customer.address,&customer.area,&customer.legal_name,&customer.emails,&customer.phones,&customer.postcode,&customer.website,&id])?;
    Ok(Msg::new(&customer.company_name, "update success"))
}
}
*/

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, MutationRoot {})
}
