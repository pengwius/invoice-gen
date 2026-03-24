#![cfg(test)]

use crate::pef_3::builder::{
    InvoiceBuilder, InvoiceLineBuilder, MonetaryTotalBuilder, PartyBuilder, PostalAddressBuilder,
    TaxTotalBuilder,
};
use crate::pef_3::models::*;

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
    let element = Element::parse(body.as_bytes())?;
    let mut cfg = EmitterConfig::new();
    cfg.perform_indent = true;

    let mut output = Vec::new();
    element.write_with_config(&mut output, cfg)?;
    Ok(String::from_utf8(output)?)
}

#[test]
fn test_party_parsing() {
    let xml = r#"<cac:AccountingSupplierParty>
<cac:Party>
<cbc:EndpointID schemeID="GLN" schemeAgencyID="9">1234567890123</cbc:EndpointID>
<cac:PartyIdentification>
<cbc:ID schemeID="ZZZ">Supp123</cbc:ID>
</cac:PartyIdentification>
<cac:PartyName>
<cbc:Name>Salescompany ltd.</cbc:Name>
</cac:PartyName>
</cac:Party>
</cac:AccountingSupplierParty>"#;

    #[derive(Debug, serde::Deserialize)]
    struct Wrapper {
        #[serde(rename = "cac:Party", alias = "Party")]
        party: PartyType,
    }

    let wrapper: Result<Wrapper, _> = quick_xml::de::from_str(xml);
    match wrapper {
        Ok(_p) => (),
        Err(e) => panic!("Failed to parse party: {:?}, source: {:?}", e, std::error::Error::source(&e)),
    }
}

#[test]
fn test_generate_minimal_invoice() {
    let address = PostalAddressBuilder::new()
        .city_name("Warszawa")
        .country("PL")
        .build();

    let supplier = PartyBuilder::new()
        .party_legal_entity("Minimal Supplier Sp. z o.o.", None)
        .postal_address(address.clone())
        .party_tax_scheme("PL5261234567", "VAT")
        .build();
    let customer = PartyBuilder::new()
        .party_legal_entity("Minimal Customer S.A.", None)
        .postal_address(address)
        .build();
    let total = MonetaryTotalBuilder::new()
        .line_extension_amount("PLN", "100.00")
        .tax_exclusive_amount("PLN", "100.00")
        .tax_inclusive_amount("PLN", "123.00")
        .payable_amount("PLN", "123.00")
        .build();

    let tax_total = TaxTotalBuilder::new()
        .tax_amount("PLN", "23.00")
        .add_subtotal("100.00", "23.00", "PLN", "S", "23", "VAT")
        .build();

    let line = InvoiceLineBuilder::new("1")
        .invoiced_quantity("EA", "1.0")
        .line_extension_amount("PLN", "100.00")
        .item("Minimal Item")
        .price_amount("PLN", "100.00")
        .tax_category("S", "23", "VAT")
        .build();

    let invoice = InvoiceBuilder::new(
        "MINIMAL-001",
        "2024-01-01",
        supplier,
        customer,
        total,
        vec![line],
    )
    .customization_id("urn:cen.eu:en16931:2017#compliant#urn:fdc:pef.mz.gov.pl:pola:krajowe:2018")
    .profile_id("urn:fdc:pef.mz.gov.pl:biznes:faktura:2018")
    .invoice_type_code(Code {
        value: "380".to_string(),
        ..Default::default()
    })
    .document_currency_code(Code {
        value: "PLN".to_string(),
        ..Default::default()
    })
    .add_tax_total(tax_total)
    .due_date("2024-01-15")
    .build();

    let xml = invoice
        .to_xml()
        .expect("Failed to serialize minimal invoice to XML");

    let pretty_xml = pretty_print_xml(&xml).unwrap_or(xml.clone());
    println!(
        "=== Minimal Invoice ===\n{}\n=======================",
        pretty_xml
    );

    assert!(xml.contains("<cbc:ID>MINIMAL-001</cbc:ID>"));
    assert!(xml.contains("<cbc:IssueDate>2024-01-01</cbc:IssueDate>"));
    assert!(xml.contains("urn:oasis:names:specification:ubl:schema:xsd:Invoice-2"));
}

#[test]
fn test_generate_complex_invoice() {
    let address = PostalAddressBuilder::new()
        .city_name("Warszawa")
        .country("PL")
        .build();

    let supplier = PartyBuilder::new()
        .party_legal_entity("Complex Supplier Sp. z o.o.", None)
        .party_tax_scheme("PL5261234567", "VAT")
        .postal_address(address.clone())
        .build();

    let customer = PartyBuilder::new()
        .party_legal_entity("Complex Customer S.A.", None)
        .party_tax_scheme("PL5261234567", "VAT")
        .postal_address(address)
        .build();

    let total = MonetaryTotalBuilder::new()
        .line_extension_amount("PLN", "1000.00")
        .tax_exclusive_amount("PLN", "1000.00")
        .tax_inclusive_amount("PLN", "1230.00")
        .payable_amount("PLN", "1230.00")
        .build();

    let line1 = InvoiceLineBuilder::new("1")
        .item("Laptop")
        .invoiced_quantity("EA", "1.0")
        .price_amount("PLN", "500.00")
        .line_extension_amount("PLN", "500.00")
        .tax_category("S", "23", "VAT")
        .build();

    let line2 = InvoiceLineBuilder::new("2")
        .item("Monitor")
        .invoiced_quantity("EA", "1.0")
        .price_amount("PLN", "500.00")
        .line_extension_amount("PLN", "500.00")
        .tax_category("S", "23", "VAT")
        .build();

    let tax_total = TaxTotalBuilder::new()
        .tax_amount("PLN", "230.00")
        .add_subtotal("1000.00", "230.00", "PLN", "S", "23", "VAT")
        .build();

    let tax_total_eur = TaxTotalBuilder::new()
        .tax_amount("EUR", "53.49")
        .build();

    let invoice = InvoiceBuilder::new(
        "COMPLEX-002",
        "2024-12-31",
        supplier,
        customer,
        total,
        vec![line1, line2],
    )
    .ubl_version_id("2.1")
    .customization_id("urn:cen.eu:en16931:2017#compliant#urn:fdc:pef.mz.gov.pl:pola:krajowe:2018")
    .profile_id("urn:fdc:pef.mz.gov.pl:biznes:faktura:2018")
    .invoice_type_code(Code {
        value: "380".to_string(),
        ..Default::default()
    })
    .document_currency_code("PLN")
    .tax_currency_code("EUR")
    .add_tax_total(tax_total)
    .add_tax_total(tax_total_eur)
    .add_note("First note regarding delivery")
    .add_note("Second note regarding payment")
    .due_date("2025-01-14")
    .build();

    let xml = invoice
        .to_xml()
        .expect("Failed to serialize complex invoice to XML");

    let pretty_xml = pretty_print_xml(&xml).unwrap_or(xml.clone());
    println!(
        "=== Complex Invoice ===\n{}\n=======================",
        pretty_xml
    );

    assert!(xml.contains("<cbc:ID>COMPLEX-002</cbc:ID>"));
    assert!(xml.contains("<cbc:Note>First note regarding delivery</cbc:Note>"));
    assert!(xml.contains("<cbc:Note>Second note regarding payment</cbc:Note>"));
    assert!(xml.contains("<cbc:TaxCurrencyCode>EUR</cbc:TaxCurrencyCode>"));
    assert!(xml.contains("<cbc:DueDate>2025-01-14</cbc:DueDate>"));
    assert!(xml.contains("<cbc:UBLVersionID>2.1</cbc:UBLVersionID>"));
}

#[test]
fn test_generate_invoice_with_references() {
    let address = PostalAddressBuilder::new()
        .city_name("Warszawa")
        .country("PL")
        .build();

    let supplier = PartyBuilder::new()
        .party_legal_entity("Ref Supplier", None)
        .postal_address(address.clone())
        .party_tax_scheme("PL5261234567", "VAT")
        .build();
    let customer = PartyBuilder::new()
        .party_legal_entity("Ref Customer", None)
        .postal_address(address)
        .build();
    let total = MonetaryTotalBuilder::new()
        .line_extension_amount("PLN", "50.00")
        .tax_exclusive_amount("PLN", "50.00")
        .tax_inclusive_amount("PLN", "61.50")
        .payable_amount("PLN", "61.50")
        .build();

    let tax_total = TaxTotalBuilder::new()
        .tax_amount("PLN", "11.50")
        .add_subtotal("50.00", "11.50", "PLN", "S", "23", "VAT")
        .build();

    let line = InvoiceLineBuilder::new("1")
        .invoiced_quantity("EA", "1.0")
        .line_extension_amount("PLN", "50.00")
        .item("Ref Item")
        .price_amount("PLN", "50.00")
        .tax_category("S", "23", "VAT")
        .build();

    let invoice = InvoiceBuilder::new(
        "REF-003",
        "2024-06-15",
        supplier,
        customer,
        total,
        vec![line],
    )
    .ubl_version_id("2.1")
    .customization_id("urn:cen.eu:en16931:2017#compliant#urn:fdc:pef.mz.gov.pl:pola:krajowe:2018")
    .profile_id("urn:fdc:pef.mz.gov.pl:biznes:faktura:2018")
    .invoice_type_code(Code {
        value: "380".to_string(),
        ..Default::default()
    })
    .document_currency_code(Code {
        value: "PLN".to_string(),
        ..Default::default()
    })
    .add_tax_total(tax_total)
    .order_reference(OrderReference {
        id: Some("ORD-456".into()),
    })
    .buyer_reference("BUYER-REF-999")
    .add_billing_reference(DocumentReference {
        id: Some("INV-001".into()),
    })
    .add_billing_reference(DocumentReference {
        id: Some("INV-002".into()),
    })
    .add_contract_document_reference(DocumentReference {
        id: Some("CONTRACT-ABC".into()),
    })
    .due_date("2024-07-15")
    .build();

    let xml = invoice
        .to_xml()
        .expect("Failed to serialize reference invoice to XML");

    let pretty_xml = pretty_print_xml(&xml).unwrap_or(xml.clone());
    println!(
        "=== Reference Invoice ===\n{}\n=========================",
        pretty_xml
    );

    assert!(xml.contains("<cbc:ID>REF-003</cbc:ID>"));
    assert!(xml.contains("<cbc:BuyerReference>BUYER-REF-999</cbc:BuyerReference>"));
    assert!(xml.contains("<cac:OrderReference"));
    assert!(xml.contains("<cac:BillingReference"));
    assert!(xml.contains("<cac:ContractDocumentReference"));
}
