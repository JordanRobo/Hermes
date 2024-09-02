use crate::models::{Invoice, InvoiceItem, PrintInvoice};
use crate::schema;
use diesel::prelude::*;
use inquire::Text;
use serde_json;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub fn print_invoice(conn: &mut SqliteConnection, id: &str) -> anyhow::Result<()> {
    let file_path = Text::new("Where would you like to save the file?")
        .with_default("~/Desktop")
        .prompt()?;

    // Parse the id as i32
    let invoice_id = id.parse::<i32>()?;

    // Fetch the invoice
    let invoice: Invoice = schema::invoices::table
        .filter(schema::invoices::id.eq(invoice_id))
        .first(conn)?;

    // Fetch the invoice items
    let items: Vec<InvoiceItem> = schema::invoice_items::table
        .filter(schema::invoice_items::invoice_id.eq(invoice_id))
        .load(conn)?;

    // Create the PrintInvoice struct
    let print_invoice = PrintInvoice { invoice, items };

    // Convert to JSON
    let json = serde_json::to_string_pretty(&print_invoice)?;

    // Create the file path
    let mut path = PathBuf::from(shellexpand::tilde(&file_path).into_owned());
    path.push(format!("invoice_{}.json", id));

    // Write to file
    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())?;

    println!("Invoice JSON saved to: {}", file_path);
    Ok(())
}
