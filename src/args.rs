use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct HermesArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    Invoices(InvoicesCommand),
    Clients(ClientsCommand),
    Settings(SettingsCommand),
    Print,
}

#[derive(Debug, Args)]
pub struct InvoicesCommand {
    #[clap(subcommand)]
    pub action: Option<InvoicesAction>,

    /// Create a new invoice
    #[clap(short = 'N', long = "new")]
    pub new: bool,

    /// Invoice ID for update or delete operations
    pub id: Option<String>,

    /// Delete the invoice
    #[clap(short = 'D', long = "delete")]
    pub delete: bool,

    /// Update the invoice
    #[clap(short = 'U', long = "update")]
    pub update: bool,
}

#[derive(Debug, Subcommand)]
pub enum InvoicesAction {
    /// List all invoices (default if no subcommand is provided)
    List,
}

#[derive(Debug, Args)]
pub struct ClientsCommand {
    #[clap(subcommand)]
    pub action: Option<ClientsAction>,

    /// Create a new buyer
    #[clap(short = 'N', long = "new")]
    pub new: bool,

    /// Buyer ID for update or delete operations
    pub id: Option<String>,

    /// Delete the buyer
    #[clap(short = 'D', long = "delete")]
    pub delete: bool,

    /// Update the buyer
    #[clap(short = 'U', long = "update")]
    pub update: bool,
}

#[derive(Debug, Subcommand)]
pub enum ClientsAction {
    /// List all buyers (default if no subcommand is provided)
    List,
}

#[derive(Debug, Args)]
pub struct SettingsCommand {
    #[clap(subcommand)]
    pub action: SettingsAction,
}

#[derive(Debug, Subcommand)]
pub enum SettingsAction {
    /// View settings
    View,
    /// Update settings
    Update,
}
