use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Identifiable, Insertable, Serialize, Deserialize)]
#[diesel(primary_key(id, name))]
#[diesel(table_name = clients)]
pub struct Client {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
#[diesel(table_name = invoice_items)]
pub struct InvoiceItem {
    pub id: Option<i32>,
    pub invoice_id: i32,
    pub item: String,
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub gst: f64,
    pub total: f64,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = invoices)]
pub struct Invoice {
    pub id: Option<i32>,
    pub date: String,
    pub due_date: String,
    pub settings_id: i32,
    pub client_name: String,
    pub gst: f64,
    pub total: f64,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize, AsChangeset)]
#[diesel(table_name = settings)]
pub struct Setting {
    pub id: Option<i32>,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub abn: String,
    pub bank_bsb: String,
    pub bank_account_number: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = invoices)]
pub struct NewInvoice {
    pub id: i32,
    pub date: String,
    pub due_date: String,
    pub settings_id: i32,
    pub client_name: String,
    pub gst: f64,
    pub total: f64,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = invoice_items)]
pub struct NewInvoiceItem {
    pub invoice_id: i32,
    pub item: String,
    pub description: String,
    pub quantity: i32,
    pub unit_price: f64,
    pub gst: f64,
    pub total: f64,
}

#[derive(Queryable, Debug, Insertable, Serialize, Deserialize)]
#[diesel(table_name = clients)]
pub struct NewClient {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrintInvoice {
    pub invoice: Invoice,
    pub items: Vec<InvoiceItem>,
}
