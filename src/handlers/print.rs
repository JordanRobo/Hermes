use crate::models::{Invoice, InvoiceItem, PrintInvoice, Setting};
use crate::{schema, SVG, SVG_TEMPLATE};
use diesel::prelude::*;
use inquire::{required, Text};
use printpdf::*;
use std::convert::From;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

pub fn print_invoice(conn: &mut SqliteConnection) -> anyhow::Result<()> {
    let id = Text::new("Which invoice would you like to print?")
        .with_help_message("Enter ID of Invoice to print.")
        .with_validator(required!("ID is required"))
        .prompt()?;
    let file_path = Text::new("Where would you like to save the file?")
        .with_default("~/Desktop")
        .prompt()?;

    // Parse the id as i32
    let invoice_id = id.parse::<i32>()?;

    // Fetch the invoice
    let invoice: Invoice = schema::invoices::table
        .filter(schema::invoices::id.eq(invoice_id))
        .first(conn)?;

    let setting: Setting = schema::settings::table
        .filter(schema::settings::id.eq(1))
        .first(conn)?;

    // Fetch the invoice items
    let items: Vec<InvoiceItem> = schema::invoice_items::table
        .filter(schema::invoice_items::invoice_id.eq(invoice_id))
        .load(conn)?;

    // Create the PrintInvoice struct
    let print_invoice = PrintInvoice {
        invoice,
        setting,
        items,
    };

    // Create PDF Document
    let (doc, page1, layer1) = PdfDocument::new("Invoice", Mm(210.0), Mm(297.0), "Layer 1");
    let template_layer = doc.get_page(page1).get_layer(layer1);

    let logo_svg = Svg::parse(SVG).unwrap();
    let logo = logo_svg.into_xobject(&template_layer);

    logo.clone().add_to_layer(
        &template_layer,
        SvgTransform {
            translate_x: Some(Pt(19.5)),
            translate_y: Some(Pt(765.0)),
            scale_x: Some(2.3),
            scale_y: Some(2.3),
            ..Default::default()
        },
    );

    let template_svg = Svg::parse(SVG_TEMPLATE).unwrap();
    let template = template_svg.into_xobject(&template_layer);

    template.clone().add_to_layer(
        &template_layer,
        SvgTransform {
            translate_x: Some(Pt(0.0)),
            translate_y: Some(Pt(0.0)),
            scale_x: Some(4.2),
            scale_y: Some(4.2),
            ..Default::default()
        },
    );

    let text_layer = doc.get_page(page1).add_layer("Layer 2");
    let mut body_font_reader =
        std::io::Cursor::new(include_bytes!("../../assets/fonts/Montserrat-Regular.ttf").as_ref());
    let font = doc.add_external_font(&mut body_font_reader).unwrap();

    let mut strong_font_reader =
        std::io::Cursor::new(include_bytes!("../../assets/fonts/Montserrat-SemiBold.ttf").as_ref());
    let strong = doc.add_external_font(&mut strong_font_reader).unwrap();

    // Helper function to convert Option<i32> to String
    fn option_i32_to_string(opt: Option<i32>) -> String {
        opt.map(|id| id.to_string())
            .unwrap_or_else(|| "N/A".to_string())
    }

    // Add invoice details
    text_layer.use_text(
        format!("# {}", option_i32_to_string(print_invoice.invoice.id)),
        12.0,
        Mm(175.0),
        Mm(270.5),
        &font,
    );
    text_layer.use_text(
        format!("{}", print_invoice.invoice.date),
        12.0,
        Mm(175.0),
        Mm(248.75),
        &font,
    );
    text_layer.use_text(
        format!("{}", print_invoice.invoice.due_date),
        12.0,
        Mm(175.0),
        Mm(238.86),
        &font,
    );
    text_layer.use_text(
        format!("${:.0}", print_invoice.invoice.total),
        14.0,
        Mm(175.0),
        Mm(228.81),
        &strong,
    );

    // Add client details
    text_layer.use_text(
        format!("{}", print_invoice.invoice.client_name),
        12.0,
        Mm(9.0),
        Mm(218.0),
        &strong,
    );

    // Add company details
    text_layer.use_text(
        format!("{}", print_invoice.setting.name),
        12.0,
        Mm(9.0),
        Mm(248.75),
        &strong,
    );
    text_layer.use_text(
        format!("{}", print_invoice.setting.abn),
        12.0,
        Mm(24.0),
        Mm(241.28),
        &font,
    );

    // Add items
    let mut y_position = 181.58;
    for item in &print_invoice.items {
        text_layer.use_text(
            format!("{}", item.item),
            10.0,
            Mm(11.0),
            Mm(y_position),
            &strong,
        );
        text_layer.use_text(
            format!("{}", item.description),
            10.0,
            Mm(11.0),
            Mm(y_position - 6.70),
            &font,
        );
        text_layer.use_text(
            format!("{}", item.quantity),
            10.0,
            Mm(109.0),
            Mm(y_position),
            &strong,
        );
        text_layer.use_text(
            format!("${:.2}", item.unit_price),
            10.0,
            Mm(132.0),
            Mm(y_position),
            &strong,
        );
        text_layer.use_text(
            format!("${:.2}", item.gst),
            10.0,
            Mm(168.0),
            Mm(y_position),
            &strong,
        );
        text_layer.use_text(
            format!("${:.0}", item.total),
            10.0,
            Mm(191.0),
            Mm(y_position),
            &strong,
        );
        y_position -= 15.0;
    }

    // Add totals
    text_layer.use_text(
        format!("${:.2}", print_invoice.invoice.gst),
        12.0,
        Mm(181.0),
        Mm(107.54),
        &font,
    );
    text_layer.use_text(
        format!("${:.0}", print_invoice.invoice.total),
        14.0,
        Mm(181.0),
        Mm(99.04),
        &strong,
    );

    // Add Banking Details
    text_layer.use_text(
        format!("{}", print_invoice.setting.bank_bsb),
        10.0,
        Mm(34.0),
        Mm(88.38),
        &font,
    );
    text_layer.use_text(
        format!("{}", print_invoice.setting.bank_account_number),
        10.0,
        Mm(34.0),
        Mm(81.26),
        &font,
    );

    // Create the file path
    let mut path = PathBuf::from(shellexpand::tilde(&file_path).into_owned());
    path.push(format!("invoice_{}.pdf", id));
    let file = File::create(path)?;

    doc.save(&mut BufWriter::new(file)).unwrap();

    println!("Invoice PDF saved to: {}", file_path);
    Ok(())
}
