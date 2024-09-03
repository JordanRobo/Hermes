use crate::autocomplete::{fetch_client_names, ClientAutocompleter};
use crate::models::{Invoice, NewInvoice, NewInvoiceItem};
use crate::utils;
use chrono::Days;
use comfy_table::Table;
use diesel::prelude::*;
use inquire::{required, Confirm, CustomType, Text};

fn add_invoice_item(input_id: i32, conn: &mut SqliteConnection) -> anyhow::Result<()> {
    use crate::schema::invoice_items::dsl::*;

    let item_name = Text::new("Name of Item:")
        .with_validator(required!("Name is required"))
        .prompt()?;
    let item_description = Text::new("Item description:").prompt()?;
    let item_quantity = CustomType::<i32>::new("Item quantity:")
        .with_error_message("Please enter a valid number")
        .with_help_message("Enter the number of items as a whole number.")
        .prompt()?;
    let item_price = CustomType::<f64>::new("Price per item:")
        .with_formatter(&|i| format!("${:.2}", i))
        .with_error_message("Please enter a valid number")
        .with_help_message("Enter the price per item using a decimal point as a separator")
        .prompt()?;

    let item_gst = item_price / 11.00;
    let item_total = item_price * item_quantity as f64;

    let new_invoice_item = NewInvoiceItem {
        invoice_id: input_id,
        item: item_name,
        description: item_description,
        quantity: item_quantity,
        unit_price: item_price,
        gst: item_gst.round_ties_even(),
        total: item_total,
    };

    diesel::insert_into(invoice_items)
        .values(&new_invoice_item)
        .execute(conn)?;

    Ok(())
}

pub fn create_invoice(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    use crate::schema::invoices::dsl::*;

    let clients = fetch_client_names(conn);
    let autocompleter = ClientAutocompleter::new(clients);
    let new_id = utils::generate_invoice_id() as i32;

    let new_date = utils::get_today().to_string();
    let new_due_date = utils::get_today()
        .checked_add_days(Days::new(14))
        .unwrap()
        .to_string();
    let new_client = Text::new("Invoice for:")
        .with_validator(required!("Client is required"))
        .with_autocomplete(autocompleter)
        .with_help_message("Start typing to see suggestions")
        .with_page_size(5)
        .prompt()?;

    let mut new_invoice = NewInvoice {
        id: new_id,
        date: new_date,
        due_date: new_due_date,
        settings_id: 1,
        client_name: new_client.clone(),
        gst: 0.00,
        total: 0.00,
    };

    // Insert the initial invoice
    diesel::insert_into(invoices)
        .values(&new_invoice)
        .execute(conn)?;

    println!("Creating invoice for {}", new_client);

    // Loop for adding items
    loop {
        let add_item = Confirm::new("Add an item to invoice?")
            .with_default(true)
            .prompt()?;

        if add_item {
            add_invoice_item(new_id, conn)?;

            // Update invoice totals
            let updated_totals = utils::calculate_invoice_totals(new_id, conn)?;
            new_invoice.gst = updated_totals.0;
            new_invoice.total = updated_totals.1;

            // Update the invoice in the database
            diesel::update(invoices)
                .filter(id.eq(Some(new_id)))
                .set((gst.eq(new_invoice.gst), total.eq(new_invoice.total)))
                .execute(conn)?;
        } else {
            break;
        }
    }

    println!("Invoice created successfully with ID: {}", new_id);
    println!("Total GST: ${:.2}", new_invoice.gst);
    println!("Total Amount: ${:.2}", new_invoice.total);

    Ok(())
}

pub fn list_invoices(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    use crate::schema::invoices::dsl::*;

    let mut table = Table::new();
    table.set_header(vec!["ID", "Date", "Client", "Total"]);

    let invoices_list: Vec<Invoice> = invoices.load(conn)?;

    for invoice in invoices_list {
        table.add_row(vec![
            invoice.id.unwrap().to_string(),
            invoice.date,
            invoice.client_name,
            format!("${:.2}", invoice.total),
        ]);
    }

    println!("{table}");

    Ok(())
}

// pub fn edit_invoice(conn: &mut SqliteConnection, id: &str) -> anyhow::Result<()> {
//     println!("Editing invoice with ID: {}", id);

//     Ok(())
// }

// pub fn delete_invoice(conn: &mut SqliteConnection, id: &str) -> anyhow::Result<()> {
//     println!("Deleted invoice with ID: {}", id);

//     Ok(())
// }

// pub fn view_invoice(conn: &mut SqliteConnection, id: &str) -> anyhow::Result<()> {
//     println!("Viewing invoice with ID: {}", id);

//     Ok(())
// }
