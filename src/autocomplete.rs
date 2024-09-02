use diesel::prelude::*;
use inquire::autocompletion::{Autocomplete, Replacement};
use inquire::CustomUserError;

#[derive(Clone)]
pub struct ClientAutocompleter {
    clients: Vec<String>,
}

impl ClientAutocompleter {
    pub fn new(clients: Vec<String>) -> Self {
        Self { clients }
    }
}

impl Autocomplete for ClientAutocompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let suggestions: Vec<String> = self
            .clients
            .iter()
            .filter(|clients| clients.to_lowercase().contains(&input.to_lowercase()))
            .take(5)
            .cloned()
            .collect();

        Ok(suggestions)
    }

    #[allow(unused_variables)]
    fn get_completion(
        &mut self,
        input: &str,
        highlighted_suggestion: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(highlighted_suggestion
            .map(Replacement::Some)
            .unwrap_or(Replacement::None))
    }
}

pub fn fetch_client_names(conn: &mut SqliteConnection) -> Vec<String> {
    use crate::schema::clients::dsl::*;

    clients
        .select(name)
        .load::<String>(conn)
        .unwrap_or_else(|_| Vec::new())
}
