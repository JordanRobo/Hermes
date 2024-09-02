use crate::models::InvoiceItem;
use chrono::prelude::*;
use chrono::NaiveDate;
use diesel::prelude::*;

pub fn generate_invoice_id() -> u32 {
    let now = chrono::Local::now();
    let year = now.year() as u32 % 100;
    let month = now.month() as u32;
    let day = now.day() as u32;
    let hour = now.hour() as u32;
    let minute = now.minute() as u32;
    let second = now.second() as u32;

    let id = year * 1_000_000 + month * 10_000 + day * 100 + hour;

    (id + minute * 60 + second) % 100_000_000
}

pub fn get_today() -> NaiveDate {
    chrono::Local::now().date_naive()
}

pub fn calculate_invoice_totals(
    input_id: i32,
    conn: &mut SqliteConnection,
) -> anyhow::Result<(f64, f64)> {
    use crate::schema::invoice_items::dsl::*;

    let items = invoice_items
        .filter(invoice_id.eq(input_id))
        .load::<InvoiceItem>(conn)?;

    let total_gst: f64 = items.iter().map(|invoice_item| invoice_item.gst).sum();
    let total_amount: f64 = items.iter().map(|invoice_item| invoice_item.total).sum();

    Ok((total_gst, total_amount))
}
