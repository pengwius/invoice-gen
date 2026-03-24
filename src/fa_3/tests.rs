#![cfg(test)]

use rust_decimal::Decimal;
use std::str::FromStr;

use crate::fa_3::builder::{
    AuthorizedSubjectBuilder, BuyerBuilder, LineBuilder, SellerBuilder, Subject3Builder,
};
use crate::fa_3::models::*;
use crate::shared::models::*;

use std::error::Error;
use xmltree::{Element, EmitterConfig};

fn pretty_print_xml(xml: &str) -> Result<String, Box<dyn Error>> {
    let body = if xml.starts_with("<?xml") {
        match xml.find("?>") {
            Some(p) => &xml[p + 2..],
            None => xml,
        }
    } else {
        xml
    };

    let root = Element::parse(body.as_bytes())?;
    let mut buffer = Vec::new();
    let config = EmitterConfig::new()
        .perform_indent(true)
        .write_document_declaration(false);
    root.write_with_config(&mut buffer, config)?;
    let pretty_body = String::from_utf8(buffer)?;

    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n{}",
        pretty_body
    ))
}

fn make_minimal_invoice() -> Invoice {
    let seller = SellerBuilder::new("5261234567", "Acme Widgets Sp. z o.o.")
        .set_address("PL", "ul. Przykładowa", "1", None, "Warszawa", "00-001")
        .build();

    let buyer = BuyerBuilder::new("9876543210", "Klient S.A.")
        .set_address("PL", "ul. Kupiecka", "5", None, "Warszawa", "00-100")
        .set_buyer_id("ID-0001")
        .build();

    let line = LineBuilder::new(
        "Sample product",
        Decimal::new(10, 0),
        Decimal::from_str("25.00").unwrap(),
        TaxRate::from_str("23").expect("valid tax rate"),
    )
    .set_uu_id("line-1-uuid")
    .set_delivery_date(&chrono::Local::now().format("%Y-%m-%d").to_string())
    .set_index("SKU-001")
    .set_gtin("00012345600012")
    .build();

    let mut invoice = Invoice::default();
    invoice.subject1 = seller;
    invoice.subject2 = buyer;
    invoice.invoice_body.invoice_number = "FV/2026/0001".to_string();
    invoice.invoice_body.issue_date = chrono::Local::now().format("%Y-%m-%d").to_string();
    invoice.invoice_body.currency_code = CurrencyCode::new("PLN");
    invoice.invoice_body.lines = vec![line];
    invoice
}

#[test]
fn test_generate_minimal_invoice_and_basic_xml_checks() {
    let inv = make_minimal_invoice();
    let xml = inv.to_xml().expect("serialize to xml");
    assert!(xml.len() > 0, "Generated XML should not be empty");

    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
    assert!(xml.contains("FV/2026/0001"));
    assert!(xml.contains("Acme Widgets Sp. z o.o."));
    assert!(xml.contains("Klient S.A."));

    assert!(xml.contains("kodSystemowy=\"FA (3)\""));

    assert!(
        xml.contains("<P_13_1>250.00</P_13_1>") || xml.contains("<P_13_1>250</P_13_1>"),
        "XML does not contain expected P_13_1 value"
    );

    assert!(
        xml.contains("<P_14_1>57.50</P_14_1>")
            || xml.contains("<P_14_1>57.5</P_14_1>")
            || xml.contains("<P_14_1>57.5</P_14_1>")
            || xml.contains("<P_14_1>57.5</P_14_1>"),
        "XML does not contain expected P_14_1 value"
    );

    assert!(
        xml.contains("<P_15>307.50</P_15>")
            || xml.contains("<P_15>307.5</P_15>")
            || xml.contains("<P_15>307.50</P_15>"),
        "XML does not contain expected P_15 total"
    );

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }
}

#[test]
fn test_generate_invoice_with_subject3_and_authorized_and_roles() {
    let mut inv = make_minimal_invoice();

    let subject3 = Subject3Builder::new("9876543210", "Podmiot3-Example")
        .set_role(TRolaPodmiotu3::WystawcaFaktury)
        .build();
    inv.subject3 = Some(subject3);

    let aut = AuthorizedSubjectBuilder::new(
        "9876543210",
        "Upowazniony-Example",
        TRolaPodmiotuUpowaznionego::KomornikSadowy,
    )
    .set_address("PL", "Upoważniony ul", "1", None, "City", "00-000")
    .build();
    inv.authorized_subject = Some(aut);

    inv.subject1.taxpayer_status = Some(TStatusInfoPodatnika::Restrukturyzacja);

    inv.invoice_body.lines = vec![
        LineBuilder::new(
            "Produkt z GTU i procedurą",
            Decimal::new(2, 0),
            Decimal::from_str("15.00").unwrap(),
            TaxRate::from_str("23").unwrap(),
        )
        .set_index("IDX-GTU-1")
        .set_uu_id("uuid-subj3-1")
        .set_gtu(GTU::Gtu01)
        .set_procedure(Procedure::BSpv)
        .build(),
    ];

    inv.invoice_body.invoice_number = "SUBJ3/2026/0001".to_string();
    inv.invoice_body.currency_code = CurrencyCode::new("EUR");

    let xml = inv.to_xml().expect("serialize to xml");

    assert!(xml.contains("Podmiot3-Example"));
    assert!(xml.contains("Upowazniony-Example"));
    assert!(xml.contains("WystawcaFaktury") || xml.contains("Rola"));
    assert!(xml.contains("BSpv") || xml.contains("Procedura") || xml.contains("procedure"));

    assert!(xml.contains("<P_15>") && xml.contains("</P_15>"));

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!(
            "\n--- test_generate_invoice_with_subject3_and_authorized ---\n{}",
            pretty
        );
    }
}

#[test]
fn test_generate_invoice_with_multiple_lines_and_currencies_checks_totals() {
    let mut inv = make_minimal_invoice();

    inv.invoice_body.currency_code = CurrencyCode::new("USD");
    inv.invoice_body.invoice_number = "MULTI/2026/0002".to_string();

    let line1 = LineBuilder::new(
        "Produkt A",
        Decimal::new(5, 0),
        Decimal::from_str("20.00").unwrap(),
        TaxRate::from_str("23").unwrap(),
    )
    .set_uu_id("uuid-multi-1")
    .set_delivery_date(&chrono::Local::now().format("%Y-%m-%d").to_string())
    .set_index("A-1")
    .build();

    let line2 = LineBuilder::new(
        "Produkt B - eksport",
        Decimal::new(1, 0),
        Decimal::from_str("500.00").unwrap(),
        TaxRate::from_str("0 EX").unwrap(),
    )
    .set_uu_id("uuid-multi-2")
    .set_delivery_date(&chrono::Local::now().format("%Y-%m-%d").to_string())
    .set_index("B-1")
    .build();

    inv.invoice_body.lines = vec![line1, line2];

    let xml = inv.to_xml().expect("serialize to xml");
    assert!(xml.contains("<KodWaluty>USD</KodWaluty>") || xml.contains("USD"));

    assert!(
        xml.contains("<P_15>623.00</P_15>")
            || xml.contains("<P_15>623</P_15>")
            || xml.contains("<P_15>623.0</P_15>"),
        "XML does not contain expected P_15 total for multi-line invoice"
    );

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!(
            "\n--- test_generate_invoice_with_multiple_lines_and_currencies ---\n{}",
            pretty
        );
    }
}

#[test]
fn test_generate_minimal_invoice() {
    let inv = make_minimal_invoice();
    let xml = inv.to_xml().expect("serialize to xml");
    let pretty = pretty_print_xml(&xml).unwrap_or(xml.clone());
    println!(
        "\n--- test_generate_minimal_invoice_print_xml ---\n{}",
        pretty
    );
    assert!(xml.len() > 0);
}
