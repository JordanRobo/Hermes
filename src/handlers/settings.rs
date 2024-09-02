use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, Table};
use diesel::prelude::*;
use inquire::Text;

use crate::models::Setting;

pub fn view_settings(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    use crate::schema::settings::dsl::*;

    let result = settings
        .filter(id.eq(1))
        .first::<Setting>(conn)
        .optional()?;

    if let Some(setting) = result {
        let mut table = Table::new();

        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS);
        table.set_header(vec![
            Cell::new(""),
            Cell::new("Settings")
                .add_attribute(comfy_table::Attribute::Bold)
                .fg(comfy_table::Color::Cyan),
        ]);
        table.add_row(vec!["Name", &setting.name]);
        table.add_row(vec!["Email", &setting.email]);
        table.add_row(vec!["Phone", &setting.phone]);
        table.add_row(vec!["ABN", &setting.abn]);
        table.add_row(vec!["BSB", &setting.bank_bsb]);
        table.add_row(vec!["Account #", &setting.bank_account_number]);

        println!("{table}");
    } else {
        println!("No settings found.");
    }

    Ok(())
}

pub fn update_settings(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    use crate::schema::settings::dsl::*;

    let new_name = Text::new("Name:").prompt()?;
    let new_email = Text::new("Email:").prompt()?;
    let new_phone = Text::new("Phone Number:").prompt()?;
    let new_abn = Text::new("ABN:").prompt()?;
    let new_bank_bsb = Text::new("BSB:").prompt()?;
    let new_bank_account_number = Text::new("Account Number:").prompt()?;

    let new_settings = Setting {
        id: Some(1),
        name: new_name,
        email: new_email,
        phone: new_phone,
        abn: new_abn,
        bank_bsb: new_bank_bsb,
        bank_account_number: new_bank_account_number,
    };

    diesel::update(settings)
        .filter(id.eq(Some(1)))
        .set(&new_settings)
        .execute(conn)?;

    println!("Updated Settings.");
    Ok(())
}
