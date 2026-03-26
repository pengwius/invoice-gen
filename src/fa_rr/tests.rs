#![cfg(test)]

use super::builder::InvoiceRRBuilder;
use super::models::Header;
use crate::shared::models::CurrencyCode;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

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

#[test]
fn test_generate_invoice_rr_basic() {
    use super::builder::{Subject1Builder, Subject2Builder, AddressBuilder, FooterBuilder};
    use crate::shared::models::IdentificationData;

    let subject1 = Subject1Builder::new(IdentificationData {
            nip: "5261234567".to_string(),
            name: "Rolnik Jan Kowalski".to_string(),
        })
        .set_address(
            AddressBuilder::new()
                .set_country_code("PL")
                .set_address_line_1("Wiejska 1")
                .set_line2("Gospodarstwo 2")
                .build(),
        )
        .build();

    let subject2 = Subject2Builder::new(IdentificationData {
            nip: "9876543210".to_string(),
            name: "Firma Skupująca Sp. z o.o.".to_string(),
        })
        .set_address(
            AddressBuilder::new()
                .set_country_code("PL")
                .set_address_line_1("Przemysłowa 5")
                .build(),
        )
        .build();

    let footer = FooterBuilder::new()
        .set_footer_text("Dziękujemy za współpracę.")
        .build();

    let builder = InvoiceRRBuilder::new()
        .set_header(Header::default())
        .set_subject1(subject1)
        .set_subject2(subject2)
        .set_currency("PLN")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 10, 25).unwrap())
        .set_invoice_number("RR/2023/10/001")
        .set_net_value(Decimal::from_str("1000.00").unwrap())
        .set_tax_value(Decimal::from_str("65.00").unwrap())
        .set_total_value(Decimal::from_str("1065.00").unwrap())
        .set_total_value_pln(Decimal::from_str("1065.00").unwrap())
        .set_total_value_words("jeden tysiąc sześćdziesiąt pięć złotych 00/100")
        .set_invoice_type(super::models::InvoiceType::VatRr)
        .set_footer(footer);

    let invoice = builder.build();

    assert_eq!(invoice.invoice_body.currency_code, CurrencyCode::new("PLN"));
    assert_eq!(invoice.invoice_body.issue_date, "2023-10-25");
    assert_eq!(invoice.invoice_body.invoice_number, "RR/2023/10/001");
    assert_eq!(invoice.invoice_body.net_value, Decimal::from_str("1000.00").unwrap());
    assert_eq!(invoice.invoice_body.tax_value, Decimal::from_str("65.00").unwrap());
    assert_eq!(invoice.invoice_body.total_value, Decimal::from_str("1065.00").unwrap());

    let xml_result = invoice.to_xml();
    assert!(xml_result.is_ok(), "XML generation failed");
    let xml = xml_result.unwrap();

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    } else {
        println!("{}", xml);
    }

    assert!(xml.contains("RR/2023/10/001"));
    assert!(xml.contains("Rolnik Jan Kowalski"));
    assert!(xml.contains("Firma Skupująca Sp. z o.o."));
    assert!(xml.contains("<P_11_1>1000.00</P_11_1>"));
    assert!(xml.contains("<P_11_2>65.00</P_11_2>"));
    assert!(xml.contains("<P_12_1>1065.00</P_12_1>"));
    assert!(xml.contains("kodSystemowy=\"FA_RR (1)\""));
    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));
}

#[test]
fn test_generate_invoice_rr_with_correspondence_address() {
    use super::builder::{Subject1Builder, Subject2Builder, AddressBuilder};
    use crate::shared::models::IdentificationData;

    let subject1 = Subject1Builder::new(IdentificationData {
            nip: "5261234567".to_string(),
            name: "Rolnik Jan Kowalski".to_string(),
        })
        .set_address(
            AddressBuilder::new()
                .set_country_code("PL")
                .set_address_line_1("Wiejska 1")
                .set_line2("Gospodarstwo 2")
                .build(),
        )
        .set_correspondence_address(
            AddressBuilder::new()
                .set_country_code("PL")
                .set_address_line_1("Pocztowa 3")
                .build(),
        )
        .build();

    let subject2 = Subject2Builder::new(IdentificationData {
            nip: "9876543210".to_string(),
            name: "Firma Skupująca Sp. z o.o.".to_string(),
        })
        .set_address(
            AddressBuilder::new()
                .set_country_code("PL")
                .set_address_line_1("Przemysłowa 5")
                .build(),
        )
        .build();

    let builder = InvoiceRRBuilder::new()
        .set_subject1(subject1)
        .set_subject2(subject2)
        .set_currency("EUR")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap())
        .set_invoice_number("RR/2023/11/002")
        .set_net_value(Decimal::from_str("2000.00").unwrap())
        .set_tax_value(Decimal::from_str("130.00").unwrap())
        .set_total_value(Decimal::from_str("2130.00").unwrap())
        .set_total_value_pln(Decimal::from_str("9000.00").unwrap())
        .set_total_value_words("dwa tysiące sto trzydzieści euro 00/100")
        .set_invoice_type(super::models::InvoiceType::VatRr);

    let invoice = builder.build();

    assert_eq!(invoice.invoice_body.currency_code, CurrencyCode::new("EUR"));
    assert_eq!(invoice.invoice_body.issue_date, "2023-11-01");
    assert_eq!(invoice.invoice_body.invoice_number, "RR/2023/11/002");
    assert_eq!(invoice.invoice_body.net_value, Decimal::from_str("2000.00").unwrap());
    assert_eq!(invoice.invoice_body.tax_value, Decimal::from_str("130.00").unwrap());
    assert_eq!(invoice.invoice_body.total_value, Decimal::from_str("2130.00").unwrap());

    let xml_result = invoice.to_xml();
    assert!(xml_result.is_ok(), "XML generation failed");
    let xml = xml_result.unwrap();

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    } else {
        println!("{}", xml);
    }

    assert!(xml.contains("RR/2023/11/002"));
    assert!(xml.contains("Rolnik Jan Kowalski"));
    assert!(xml.contains("Firma Skupująca Sp. z o.o."));
    assert!(xml.contains("<P_11_1>2000.00</P_11_1>"));
    assert!(xml.contains("<P_11_2>130.00</P_11_2>"));
    assert!(xml.contains("<P_12_1>2130.00</P_12_1>"));
    assert!(xml.contains("kodSystemowy=\"FA_RR (1)\""));
 }
