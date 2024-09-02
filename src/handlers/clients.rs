use diesel::prelude::*;
use inquire::Text;

use crate::models::NewClient;

pub fn create_client(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    use crate::schema::clients;

    let name = Text::new("Clients name:").prompt()?;
    let email = Text::new("Email address:").prompt()?;

    let new_client = NewClient { name, email };

    diesel::insert_into(clients::table)
        .values(&new_client)
        .execute(conn)?;

    Ok(())
}

// pub fn view_creditor(conn: &mut SqliteConnection, id: &str) -> anyhow::Result<()> {
//     // Implementation for viewing a specific buyer
//     Ok(())
// }

// pub fn edit_creditor(conn: &mut SqliteConnection, id: &str) -> anyhow::Result<()> {
//     // Implementation for editing a buyer
//     Ok(())
// }

// pub fn delete_creditor(conn: &mut SqliteConnection, id: &str) -> anyhow::Result<()> {
//     // Implementation for deleting a buyer
//     Ok(())
// }

// pub fn list_creditors(conn: &mut SqliteConnection) -> anyhow::Result<()> {
//     // Implementation for listing all buyers
//     Ok(())
// }
