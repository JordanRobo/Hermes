// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id, name) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
    }
}

diesel::table! {
    invoice_items (id) {
        id -> Nullable<Integer>,
        invoice_id -> Integer,
        item -> Text,
        description -> Text,
        quantity -> Integer,
        unit_price -> Double,
        gst -> Double,
        total -> Double,
    }
}

diesel::table! {
    invoices (id) {
        id -> Nullable<Integer>,
        date -> Text,
        due_date -> Text,
        settings_id -> Integer,
        client_name -> Text,
        gst -> Double,
        total -> Double,
    }
}

diesel::table! {
    settings (id) {
        id -> Nullable<Integer>,
        name -> Text,
        email -> Text,
        phone -> Text,
        abn -> Text,
        bank_bsb -> Text,
        bank_account_number -> Text,
    }
}

diesel::joinable!(invoice_items -> invoices (invoice_id));
diesel::joinable!(invoices -> settings (settings_id));

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    invoice_items,
    invoices,
    settings,
);
