use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct HermesArgs {
    #[clap(subcommand)]
    pub entity: Entity,
}

#[derive(Debug, Subcommand)]
pub enum Entity {
    Invoice(InvoiceCommand),
    Client(ClientCommand),
    Settings(SettingsCommand),
    Print,
}

#[derive(Debug, Args)]
pub struct InvoiceCommand {
    #[clap(subcommand)]
    pub action: Option<InvoiceAction>,

    /// Create a new invoice
    #[clap(short = 'N', long = "new")]
    pub new: bool,

    /// Delete the invoice
    #[clap(short = 'D', long = "delete")]
    pub delete: bool,

    /// Update the invoice
    #[clap(short = 'U', long = "update")]
    pub update: bool,
}

#[derive(Debug, Subcommand)]
pub enum InvoiceAction {
    /// List all invoices (default if no subcommand is provided)
    List,
}

#[derive(Debug, Args)]
pub struct ClientCommand {
    #[clap(subcommand)]
    pub action: Option<ClientAction>,

    /// Create a new client
    #[clap(short = 'N', long = "new")]
    pub new: bool,

    /// Delete the client
    #[clap(short = 'D', long = "delete")]
    pub delete: bool,

    /// Update the client
    #[clap(short = 'U', long = "update")]
    pub update: bool,
}

#[derive(Debug, Subcommand)]
pub enum ClientAction {
    /// List all clients (default if no subcommand is provided)
    List,
}

#[derive(Debug, Args)]
pub struct SettingsCommand {
    #[clap(subcommand)]
    pub action: Option<SettingsAction>,

    /// Update settings
    #[clap(short = 'U', long = "update")]
    pub update: bool,
}

#[derive(Debug, Subcommand)]
pub enum SettingsAction {
    /// View settings (default if no subcommand is provided)
    View,
}
