#![cfg(test)]

use super::builder::{BuyerBuilder, InvoiceBuilder, LineBuilder, SellerBuilder};
use super::*;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use std::str::FromStr;

#[test]
fn test_generate_invoice_fa_2() {
    let seller = SellerBuilder::new("5261234567", "Firma Sprzedająca Sp. z o.o.")
        .set_address("PL", "Kwiatowa", "1", Some("2"), "Warszawa", "00-001")
        .build();

    let buyer = BuyerBuilder::new("9876543210", "Klient Kupujący Jan Kowalski")
        .set_address("PL", "Słoneczna", "5", None, "Kraków", "30-001")
        .build();

    let builder = InvoiceBuilder::new()
        .set_invoice_number("FV/2023/10/001")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 10, 25).unwrap())
        .set_sale_date(NaiveDate::from_ymd_opt(2023, 10, 20).unwrap())
        .set_currency("PLN")
        .set_seller(seller)
        .set_buyer(buyer)
        .add_lines(vec![
            LineBuilder::new(
                "Usługa programistyczna",
                "godz.",
                Decimal::from_str("10").unwrap(),
                Decimal::from_str("100").unwrap(),
                "23",
            )
            .build(),
            LineBuilder::new(
                "Konsultacje",
                "szt.",
                Decimal::from_str("1").unwrap(),
                Decimal::from_str("200").unwrap(),
                "23",
            )
            .build(),
        ]);

    let result = builder.build();

    assert!(result.is_ok(), "Builder failed: {:?}", result.err());
    let invoice = result.unwrap();

    assert_eq!(
        invoice.invoice_body.net_total_basic_rate,
        Decimal::from_str("1200.00").unwrap()
    );
    assert_eq!(
        invoice.invoice_body.tax_total_basic_rate,
        Decimal::from_str("276.00").unwrap()
    );
    assert_eq!(
        invoice.invoice_body.total_gross,
        Decimal::from_str("1476.00").unwrap()
    );

    let xml_result = invoice.to_xml();
    assert!(xml_result.is_ok(), "XML generation failed");
    let xml = xml_result.unwrap();

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }

    assert!(xml.contains("FV/2023/10/001"));
    assert!(xml.contains("Firma Sprzedająca Sp. z o.o."));
    assert!(
        xml.contains("<P_13_1>1200.00</P_13_1>") || xml.contains("<P_13_1>1200</P_13_1>"),
        "XML does not contain expected P_13_1 value"
    );
    assert!(xml.contains("<NoweSrodkiTransportu>"));
    assert!(xml.contains("<P_22N>1</P_22N>"));
    assert!(xml.contains("</NoweSrodkiTransportu>"));

    assert!(xml.contains("<PMarzy>"));
    assert!(xml.contains("<P_PMarzyN>1</P_PMarzyN>"));
    assert!(xml.contains("</PMarzy>"));

    assert!(xml.contains("kodSystemowy=\"FA (2)\""));

    assert!(xml.starts_with("<?xml version=\"1.0\" encoding=\"utf-8\"?>"));

    assert!(xml.contains(
        "<KodFormularza kodSystemowy=\"FA (2)\" wersjaSchemy=\"1-0E\">FA</KodFormularza>"
    ));

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }
}

#[test]
fn test_generate_invoice_fa_2_with_annotations() {
    let seller = SellerBuilder::new("5261234567", "Seller Sp. z o.o.")
        .set_address("PL", "Kwiatowa", "1", None, "Warszawa", "00-001")
        .build();

    let buyer = BuyerBuilder::new("9876543210", "Buyer Jan Kowalski")
        .set_address("PL", "Słoneczna", "5", None, "Kraków", "30-001")
        .build();

    let builder = InvoiceBuilder::new()
        .set_invoice_number("FV/2023/10/002")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 10, 26).unwrap())
        .set_sale_date(NaiveDate::from_ymd_opt(2023, 10, 21).unwrap())
        .set_currency("PLN")
        .set_seller(seller)
        .set_buyer(buyer)
        .add_line(
            LineBuilder::new(
                "Towar",
                "szt.",
                Decimal::from_str("1").unwrap(),
                Decimal::from_str("100").unwrap(),
                "23",
            )
            .build(),
        )
        .set_exemption(1, None, Some("Par 1"), None, None)
        .set_new_transport(1, Some(1))
        .set_margin_procedures(Some(1), None, Some(1), None, None);

    let result = builder.build();
    assert!(result.is_ok());
    let invoice = result.unwrap();

    let xml_result = invoice.to_xml();
    assert!(xml_result.is_ok());
    let xml = xml_result.unwrap();

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }

    assert!(xml.contains("<Adnotacje>"));
    assert!(xml.contains("<Zwolnienie>"));
    assert!(xml.contains("<P_19>1</P_19>"));
    assert!(xml.contains("<P_19A>Par 1</P_19A>"));
    assert!(xml.contains("</Zwolnienie>"));

    assert!(xml.contains("<NoweSrodkiTransportu>"));
    assert!(xml.contains("<P_22>1</P_22>"));
    assert!(xml.contains("<P_42_5>1</P_42_5>"));
    assert!(xml.contains("<NowySrodekTransportu>"));
    assert!(xml.contains("<P_22A>2023-10-26</P_22A>"));
    assert!(xml.contains("<P_NrWierszaNST>1</P_NrWierszaNST>"));
    assert!(xml.contains("</NowySrodekTransportu>"));
    assert!(xml.contains("</NoweSrodkiTransportu>"));
    assert!(!xml.contains("<P_22N>"));

    assert!(xml.contains("<P_23>1</P_23>"));
    assert!(xml.contains("<PMarzy>"));
    assert!(xml.contains("<P_PMarzy>1</P_PMarzy>"));
    assert!(!xml.contains("<P_PMarzy_2>"));
    assert!(xml.contains("<P_PMarzy_3_1>1</P_PMarzy_3_1>"));
    assert!(!xml.contains("<P_PMarzy_3_2>"));
    assert!(!xml.contains("<P_PMarzy_3_3>"));
    assert!(xml.contains("</PMarzy>"));
}

#[test]
fn test_invoice_minimal_no_annotations() {
    let seller = SellerBuilder::new("5261234567", "Seller")
        .set_address("PL", "Ul", "1", None, "City", "00-000")
        .build();

    let buyer = BuyerBuilder::new("9876543210", "Buyer")
        .set_address("PL", "Ul", "2", None, "City", "00-000")
        .build();

    let builder = InvoiceBuilder::new()
        .set_invoice_number("MIN/001")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap())
        .set_currency("PLN")
        .set_seller(seller)
        .set_buyer(buyer)
        .add_line(
            LineBuilder::new(
                "Service",
                "hrs",
                Decimal::from_str("1").unwrap(),
                Decimal::from_str("100").unwrap(),
                "23",
            )
            .build(),
        );

    let invoice = builder.build().expect("Build failed");
    let xml = invoice.to_xml().expect("XML failed");

    assert!(xml.contains("<PMarzy>"));
    assert!(xml.contains("<P_PMarzyN>1</P_PMarzyN>"));
    assert!(xml.contains("</PMarzy>"));

    assert!(xml.contains("<NoweSrodkiTransportu>"));
    assert!(xml.contains("<P_22N>1</P_22N>"));
    assert!(xml.contains("</NoweSrodkiTransportu>"));

    assert!(xml.contains("<Zwolnienie>"));
    assert!(xml.contains("<P_19N>1</P_19N>"));
    assert!(xml.contains("</Zwolnienie>"));

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }
}

#[test]
fn test_invoice_margin_procedure() {
    let seller = SellerBuilder::new("5261234567", "Seller")
        .set_address("PL", "Ul", "1", None, "City", "00-000")
        .build();

    let buyer = BuyerBuilder::new("9876543210", "Buyer")
        .set_address("PL", "Ul", "2", None, "City", "00-000")
        .build();

    let builder = InvoiceBuilder::new()
        .set_invoice_number("MAR/001")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap())
        .set_currency("PLN")
        .set_seller(seller)
        .set_buyer(buyer)
        .add_line(
            LineBuilder::new(
                "Used Car",
                "pcs",
                Decimal::from_str("1").unwrap(),
                Decimal::from_str("20000").unwrap(),
                "23",
            )
            .build(),
        )
        .set_margin_procedures(
            Some(1), // P_23
            Some(1), // P_PMarzy_2
            None,
            None,
            None,
        );

    let invoice = builder.build().expect("Build failed");
    let xml = invoice.to_xml().expect("XML failed");

    assert!(xml.contains("<PMarzy>"));
    assert!(xml.contains("<P_PMarzy>1</P_PMarzy>"));
    assert!(xml.contains("<P_PMarzy_2>1</P_PMarzy_2>"));
    assert!(!xml.contains("<P_PMarzy_3_1>"));
    assert!(!xml.contains("<P_PMarzy_3_2>"));
    assert!(xml.contains("</PMarzy>"));

    assert!(!xml.contains("<P_PMarzyN>"));

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }
}

#[test]
fn test_invoice_exemption() {
    let seller = SellerBuilder::new("5261234567", "Seller")
        .set_address("PL", "Ul", "1", None, "City", "00-000")
        .build();

    let buyer = BuyerBuilder::new("9876543210", "Buyer")
        .set_address("PL", "Ul", "2", None, "City", "00-000")
        .build();

    let builder = InvoiceBuilder::new()
        .set_invoice_number("ZW/001")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap())
        .set_currency("PLN")
        .set_seller(seller)
        .set_buyer(buyer)
        .add_line(
            LineBuilder::new(
                "Medical Svc",
                "svc",
                Decimal::from_str("1").unwrap(),
                Decimal::from_str("100").unwrap(),
                "zw",
            )
            .build(),
        )
        .set_exemption(1, None, Some("Basis"), None, None);

    let invoice = builder.build().expect("Build failed");
    let xml = invoice.to_xml().expect("XML failed");

    assert!(xml.contains("<Zwolnienie>"));
    assert!(xml.contains("<P_19>1</P_19>"));
    assert!(xml.contains("<P_19A>Basis</P_19A>"));
    assert!(!xml.contains("<P_19N>"));
    assert!(xml.contains("</Zwolnienie>"));

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }
}

#[test]
fn test_invoice_new_transport() {
    let seller = SellerBuilder::new("5261234567", "Seller")
        .set_address("PL", "Ul", "1", None, "City", "00-000")
        .build();

    let buyer = BuyerBuilder::new("9876543210", "Buyer")
        .set_address("PL", "Ul", "2", None, "City", "00-000")
        .build();

    let builder = InvoiceBuilder::new()
        .set_invoice_number("NST/001")
        .set_issue_date(NaiveDate::from_ymd_opt(2023, 11, 1).unwrap())
        .set_currency("PLN")
        .set_seller(seller)
        .set_buyer(buyer)
        .add_line(
            LineBuilder::new(
                "Car",
                "pcs",
                Decimal::from_str("1").unwrap(),
                Decimal::from_str("50000").unwrap(),
                "23",
            )
            .build(),
        )
        .set_new_transport(1, Some(1));

    let invoice = builder.build().expect("Build failed");
    let xml = invoice.to_xml().expect("XML failed");

    assert!(xml.contains("<NoweSrodkiTransportu>"));
    assert!(xml.contains("<P_22>1</P_22>"));
    assert!(xml.contains("<NowySrodekTransportu>"));
    assert!(!xml.contains("<P_22N>"));
    assert!(xml.contains("</NoweSrodkiTransportu>"));

    if let Ok(pretty) = pretty_print_xml(&xml) {
        println!("{}", pretty);
    }
}
