CREATE TABLE settings (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL DEFAULT '',
    email TEXT NOT NULL DEFAULT '',
    phone TEXT NOT NULL DEFAULT '',
    abn TEXT NOT NULL DEFAULT '',
    bank_bsb TEXT NOT NULL DEFAULT '',
    bank_account_number TEXT NOT NULL DEFAULT ''
);

CREATE TABLE clients (
    id INTEGER,
    name TEXT NOT NULL,
    email TEXT NOT NULL,
    PRIMARY KEY (id, name)
);

CREATE TABLE invoices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    settings_id INTEGER NOT NULL DEFAULT 1,
    client_name TEXT NOT NULL,
    gst DECIMAL(10, 2) NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (settings_id) REFERENCES settings (id),
    FOREIGN KEY (client_name) REFERENCES clients (name)
);

CREATE TABLE invoice_items (
    id INTEGER PRIMARY KEY,
    invoice_id INTEGER NOT NULL,
    item TEXT NOT NULL,
    description TEXT NOT NULL,
    quantity INTEGER NOT NULL,
    unit_price DECIMAL(10, 2) NOT NULL,
    gst DECIMAL(10, 2) NOT NULL,
    total DECIMAL(10, 2) NOT NULL,
    FOREIGN KEY (invoice_id) REFERENCES invoices (id)
);

INSERT INTO settings (id, name, email, phone, abn, bank_bsb, bank_account_number) VALUES
(1, "", "", "", "", "", "");
