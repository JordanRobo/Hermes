pub mod args;
pub mod autocomplete;
pub mod handlers;
pub mod models;
pub mod schema;

use args::*;
use clap::Parser;
use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use handlers::*;
use std::fs;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
pub const SVG: &str = include_str!("../assets/svg/logo.svg");
pub const SVG_TEMPLATE: &str = include_str!("../assets/svg/template.svg");

fn establish_connection() -> SqliteConnection {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let db_dir = home_dir.join(".local").join("share").join("hermes");
    let database_path = db_dir.join("hermes.db");
    let database_url = database_path.to_str().unwrap();

    // Ensure the directory exists
    fs::create_dir_all(&db_dir).expect("Failed to create database directory");

    // Create the database file if it doesn't exist
    if !database_path.exists() {
        fs::File::create(&database_path).expect("Failed to create database file");
    }

    let mut conn = SqliteConnection::establish(database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

    // Run migrations
    conn.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");

    conn
}

fn main() -> anyhow::Result<()> {
    let args = HermesArgs::parse();
    let conn = &mut establish_connection();

    match args.entity {
        Entity::Invoice(cmd) => {
            if cmd.new {
                invoices::create_invoice(conn)?;
            } else if cmd.delete {
                invoices::delete_invoice(conn)?;
            } else if cmd.update {
                invoices::edit_invoice(conn)?;
                println!("Edit invoice");
            } else {
                match cmd.action {
                    Some(InvoiceAction::List) | None => invoices::list_invoices(conn)?,
                }
            }
        }
        Entity::Client(cmd) => {
            if cmd.new {
                clients::create_client(conn)?;
            } else if cmd.delete {
                // clients::delete_client(conn)?;
                println!("Deleting client");
            } else if cmd.update {
                // clients::edit_client(conn)?;
                println!("Edit client");
            } else {
                match cmd.action {
                    Some(ClientAction::List) | None => println!("Viewing client"), //creditors::list_creditors(conn)?,
                }
            }
        }
        Entity::Settings(cmd) => {
            if cmd.update {
                settings::update_settings(conn)?;
            } else {
                match cmd.action {
                    Some(SettingsAction::View) | None => settings::view_settings(conn)?,
                }
            }
        }
        Entity::Print => print::print_invoice(conn)?,
    }

    Ok(())
}
