use crate::pef_3::builder::{
    InvoiceBuilder, InvoiceLineBuilder, MonetaryTotalBuilder, PartyBuilder, PostalAddressBuilder,
    TaxTotalBuilder,
};

fn create_common_invoice_builder() -> InvoiceBuilder {
    let address = PostalAddressBuilder::new()
        .city_name("Warszawa")
        .country("PL")
        .build();

    let supplier = PartyBuilder::new()
        .party_name("Seller Name Ltd.")
        .postal_address(address.clone())
        .party_tax_scheme("PL5261234567", "VAT")
        .build();

    let customer = PartyBuilder::new()
        .party_name("Buyer Name S.A.")
        .postal_address(address)
        .build();

    let total = MonetaryTotalBuilder::new()
        .line_extension_amount("PLN", "100.00")
        .tax_exclusive_amount("PLN", "100.00")
        .tax_inclusive_amount("PLN", "123.00")
        .payable_amount("PLN", "123.00")
        .build();

    let line = InvoiceLineBuilder::new("1")
        .invoiced_quantity("EA", "1.0")
        .line_extension_amount("PLN", "100.00")
        .item("Item Name")
        .price_amount("PLN", "100.00")
        .tax_category("S", "23", "VAT")
        .build();

    InvoiceBuilder::new(
        "INV-TEST-001",
        "2024-01-01",
        supplier,
        customer,
        total,
        vec![line],
    )
    .due_date("2024-01-31")
}

#[test]
fn test_br_06_seller_name_exists() {
    let builder = create_common_invoice_builder();
    let invoice = builder.build();

    let supplier_party = invoice.accounting_supplier_party.as_ref().unwrap();
    let party_names = supplier_party.party.party_name.as_ref();

    assert!(
        party_names.is_some(),
        "BR-06: AccountingSupplierParty/Party/PartyName must be present"
    );
    let names = party_names.unwrap();
    assert!(
        !names.is_empty(),
        "BR-06: At least one PartyName must be present"
    );
    assert!(
        !names[0].name.is_empty(),
        "BR-06: PartyName/Name must not be empty"
    );
    assert_eq!(names[0].name, "Seller Name Ltd.");

    let xml = invoice.to_xml().unwrap();
    assert!(
        xml.contains("<cbc:Name>Seller Name Ltd.</cbc:Name>"),
        "BR-06: XML must contain the seller name"
    );
}

#[test]
fn test_br_07_buyer_name_exists() {
    let builder = create_common_invoice_builder();
    let invoice = builder.build();

    let customer_party = invoice.accounting_customer_party.as_ref().unwrap();
    let party_names = customer_party.party.party_name.as_ref();

    assert!(
        party_names.is_some(),
        "BR-07: AccountingCustomerParty/Party/PartyName must be present"
    );
    let names = party_names.unwrap();
    assert!(
        !names.is_empty(),
        "BR-07: At least one PartyName must be present"
    );
    assert!(
        !names[0].name.is_empty(),
        "BR-07: PartyName/Name must not be empty"
    );
    assert_eq!(names[0].name, "Buyer Name S.A.");

    let xml = invoice.to_xml().unwrap();
    assert!(
        xml.contains("<cbc:Name>Buyer Name S.A.</cbc:Name>"),
        "BR-07: XML must contain the buyer name"
    );
}

#[test]
fn test_br_06_seller_legal_name_exists() {
    let address = PostalAddressBuilder::new()
        .city_name("Warszawa")
        .country("PL")
        .build();

    let supplier = PartyBuilder::new()
        .party_legal_entity("Legal Seller Name Sp. z o.o.", None)
        .postal_address(address.clone())
        .party_tax_scheme("PL5261234567", "VAT")
        .build();

    let customer = PartyBuilder::new()
        .party_name("Buyer Name S.A.")
        .postal_address(address)
        .build();

    let total = MonetaryTotalBuilder::new()
        .line_extension_amount("PLN", "100.00")
        .tax_exclusive_amount("PLN", "100.00")
        .tax_inclusive_amount("PLN", "123.00")
        .payable_amount("PLN", "123.00")
        .build();

    let line = InvoiceLineBuilder::new("1")
        .invoiced_quantity("EA", "1.0")
        .line_extension_amount("PLN", "100.00")
        .item("Item Name")
        .price_amount("PLN", "100.00")
        .tax_category("S", "23", "VAT")
        .build();

    let invoice = InvoiceBuilder::new(
        "INV-TEST-LEGAL-001",
        "2024-01-01",
        supplier,
        customer,
        total,
        vec![line],
    )
    .due_date("2024-01-31")
    .build();

    let supplier_party = invoice.accounting_supplier_party.as_ref().unwrap();
    let legal_entity = supplier_party.party.party_legal_entity.as_ref();

    assert!(
        legal_entity.is_some(),
        "BR-06: AccountingSupplierParty/Party/PartyLegalEntity should be present"
    );
    let registration_name = legal_entity.unwrap().registration_name.as_ref();
    assert!(
        registration_name.is_some(),
        "BR-06: RegistrationName must be present"
    );
    assert_eq!(registration_name.unwrap(), "Legal Seller Name Sp. z o.o.");

    let xml = invoice.to_xml().unwrap();
    assert!(
        xml.contains("<cbc:RegistrationName>Legal Seller Name Sp. z o.o.</cbc:RegistrationName>"),
        "BR-06: XML must contain the seller legal name"
    );
}

#[test]
fn test_br_07_buyer_legal_name_exists() {
    let address = PostalAddressBuilder::new()
        .city_name("Warszawa")
        .country("PL")
        .build();

    let supplier = PartyBuilder::new()
        .party_name("Seller Name Ltd.")
        .postal_address(address.clone())
        .party_tax_scheme("PL5261234567", "VAT")
        .build();

    let customer = PartyBuilder::new()
        .party_legal_entity("Legal Buyer Name S.A.", None)
        .postal_address(address)
        .build();

    let total = MonetaryTotalBuilder::new()
        .line_extension_amount("PLN", "100.00")
        .tax_exclusive_amount("PLN", "100.00")
        .tax_inclusive_amount("PLN", "123.00")
        .payable_amount("PLN", "123.00")
        .build();

    let line = InvoiceLineBuilder::new("1")
        .invoiced_quantity("EA", "1.0")
        .line_extension_amount("PLN", "100.00")
        .item("Item Name")
        .price_amount("PLN", "100.00")
        .tax_category("S", "23", "VAT")
        .build();

    let invoice = InvoiceBuilder::new(
        "INV-TEST-LEGAL-002",
        "2024-01-01",
        supplier,
        customer,
        total,
        vec![line],
    )
    .due_date("2024-01-31")
    .build();

    let customer_party = invoice.accounting_customer_party.as_ref().unwrap();
    let legal_entity = customer_party.party.party_legal_entity.as_ref();

    assert!(
        legal_entity.is_some(),
        "BR-07: AccountingCustomerParty/Party/PartyLegalEntity should be present"
    );
    let registration_name = legal_entity.unwrap().registration_name.as_ref();
    assert!(
        registration_name.is_some(),
        "BR-07: RegistrationName must be present"
    );
    assert_eq!(registration_name.unwrap(), "Legal Buyer Name S.A.");

    let xml = invoice.to_xml().unwrap();
    assert!(
        xml.contains("<cbc:RegistrationName>Legal Buyer Name S.A.</cbc:RegistrationName>"),
        "BR-07: XML must contain the buyer legal name"
    );
}

#[test]
fn test_br_53_vat_accounting_currency_total() {
    let builder = create_common_invoice_builder();

    let tax_total_pln = TaxTotalBuilder::new()
        .tax_amount("PLN", "23.00")
        .build();

    let tax_total_eur = TaxTotalBuilder::new()
        .tax_amount("EUR", "5.35")
        .build();

    let invoice = builder
        .document_currency_code("PLN")
        .tax_currency_code("EUR") // BT-6
        .add_tax_total(tax_total_pln)
        .add_tax_total(tax_total_eur) // BT-111
        .build();

    let xml = invoice.to_xml().unwrap();

    assert!(
        xml.contains("<cbc:TaxCurrencyCode>EUR</cbc:TaxCurrencyCode>"),
        "BR-53: TaxCurrencyCode (BT-6) should be present"
    );

    assert!(
        xml.contains(r#"<cbc:TaxAmount currencyID="EUR">5.35</cbc:TaxAmount>"#),
        "BR-53: TaxAmount in TaxCurrencyCode must be provided (BT-111)"
    );
}
