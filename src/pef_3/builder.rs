use crate::pef_3::models::*;

macro_rules! builder_setter {
    ($name:ident, $field:ident, impl Into<String>) => {
        pub fn $name(mut self, value: impl Into<String>) -> Self {
            self.invoice.$field = Some(value.into());
            self
        }
    };
    ($name:ident, $field:ident, $type:ty) => {
        pub fn $name(mut self, value: $type) -> Self {
            self.invoice.$field = Some(value.into());
            self
        }
    };
}

macro_rules! builder_vec_setter {
    ($name:ident, $field:ident, impl Into<String>) => {
        pub fn $name(mut self, value: impl Into<String>) -> Self {
            if let Some(ref mut vec) = self.invoice.$field {
                vec.push(value.into());
            } else {
                self.invoice.$field = Some(vec![value.into()]);
            }
            self
        }
    };
    ($name:ident, $field:ident, $type:ty) => {
        pub fn $name(mut self, value: $type) -> Self {
            if let Some(ref mut vec) = self.invoice.$field {
                vec.push(value.into());
            } else {
                self.invoice.$field = Some(vec![value.into()]);
            }
            self
        }
    };
}

pub struct InvoiceBuilder {
    invoice: Invoice,
}

impl InvoiceBuilder {
    pub fn new(
        id: impl Into<String>,
        issue_date: impl Into<String>,
        accounting_supplier_party: PartyType,
        accounting_customer_party: PartyType,
        legal_monetary_total: MonetaryTotal,
        invoice_line: Vec<InvoiceLine>,
    ) -> Self {
        Self {
            invoice: Invoice {
                xmlns: Some("urn:oasis:names:specification:ubl:schema:xsd:Invoice-2".into()),
                xmlns_cac: Some(
                    "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2"
                        .into(),
                ),
                xmlns_cbc: Some(
                    "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2".into(),
                ),
                xmlns_ext: Some(
                    "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2"
                        .into(),
                ),
                ubl_extensions: None,
                ubl_version_id: Some("2.1".into()),
                customization_id: None,
                profile_id: None,
                profile_execution_id: None,
                id: Some(id.into().into()),
                copy_indicator: None,
                uuid: None,
                issue_date: Some(issue_date.into()),
                issue_time: None,
                due_date: None,
                invoice_type_code: None,
                note: None,
                tax_point_date: None,
                document_currency_code: None,
                tax_currency_code: None,
                pricing_currency_code: None,
                payment_currency_code: None,
                payment_alternative_currency_code: None,
                accounting_cost_code: None,
                accounting_cost: None,
                line_count_numeric: None,
                buyer_reference: None,
                invoice_period: None,
                order_reference: None,
                billing_reference: None,
                despatch_document_reference: None,
                receipt_document_reference: None,
                statement_document_reference: None,
                originator_document_reference: None,
                contract_document_reference: None,
                additional_document_reference: None,
                project_reference: None,
                signature: None,
                accounting_supplier_party: Some(PartyWrapper { party: accounting_supplier_party }),
            accounting_customer_party: Some(PartyWrapper { party: accounting_customer_party }),
                payee_party: None,
                buyer_customer_party: None,
                seller_supplier_party: None,
                tax_representative_party: None,
                delivery: None,
                delivery_terms: None,
                payment_means: None,
                payment_terms: None,
                prepaid_payment: None,
                allowance_charge: None,
                tax_exchange_rate: None,
                pricing_exchange_rate: None,
                payment_exchange_rate: None,
                payment_alternative_exchange_rate: None,
                tax_total: None,
                withholding_tax_total: None,
                legal_monetary_total: Some(legal_monetary_total),
                invoice_line: Some(invoice_line),
            },
        }
    }

    builder_setter!(ubl_extensions, ubl_extensions, UblExtensions);
    builder_setter!(ubl_version_id, ubl_version_id, impl Into<String>);
    builder_setter!(customization_id, customization_id, impl Into<String>);
    builder_setter!(profile_id, profile_id, impl Into<String>);
    builder_setter!(
        profile_execution_id,
        profile_execution_id,
        impl Into<String>
    );
    builder_setter!(copy_indicator, copy_indicator, bool);
    builder_setter!(uuid, uuid, impl Into<String>);
    builder_setter!(issue_time, issue_time, impl Into<String>);
    builder_setter!(due_date, due_date, impl Into<String>);
    builder_setter!(invoice_type_code, invoice_type_code, impl Into<Code>);
    builder_setter!(tax_point_date, tax_point_date, impl Into<String>);
    builder_setter!(document_currency_code, document_currency_code, impl Into<Code>);
    builder_setter!(tax_currency_code, tax_currency_code, impl Into<Code>);
    builder_setter!(pricing_currency_code, pricing_currency_code, impl Into<Code>);
    builder_setter!(payment_currency_code, payment_currency_code, impl Into<Code>);
    builder_setter!(payment_alternative_currency_code, payment_alternative_currency_code, impl Into<Code>);
    builder_setter!(
        accounting_cost_code,
        accounting_cost_code,
        impl Into<String>
    );
    builder_setter!(accounting_cost, accounting_cost, impl Into<String>);
    builder_setter!(line_count_numeric, line_count_numeric, i32);
    builder_setter!(buyer_reference, buyer_reference, impl Into<String>);

    builder_vec_setter!(add_note, note, impl Into<TextType>);
    builder_vec_setter!(add_invoice_period, invoice_period, Period);

    builder_setter!(order_reference, order_reference, OrderReference);

    builder_vec_setter!(add_billing_reference, billing_reference, DocumentReference);
    builder_vec_setter!(
        add_despatch_document_reference,
        despatch_document_reference,
        DocumentReference
    );
    builder_vec_setter!(
        add_receipt_document_reference,
        receipt_document_reference,
        DocumentReference
    );
    builder_vec_setter!(
        add_statement_document_reference,
        statement_document_reference,
        DocumentReference
    );
    builder_vec_setter!(
        add_originator_document_reference,
        originator_document_reference,
        DocumentReference
    );
    builder_vec_setter!(
        add_contract_document_reference,
        contract_document_reference,
        DocumentReference
    );
    builder_vec_setter!(
        add_additional_document_reference,
        additional_document_reference,
        DocumentReference
    );
    builder_vec_setter!(add_project_reference, project_reference, ProjectReference);
    builder_vec_setter!(add_signature, signature, Signature);

    builder_setter!(payee_party, payee_party, PartyType);
    builder_setter!(buyer_customer_party, buyer_customer_party, PartyType);
    builder_setter!(seller_supplier_party, seller_supplier_party, PartyType);
    builder_setter!(
        tax_representative_party,
        tax_representative_party,
        PartyType
    );

    builder_vec_setter!(add_delivery, delivery, Delivery);
    builder_setter!(delivery_terms, delivery_terms, DeliveryTerms);

    builder_vec_setter!(add_payment_means, payment_means, PaymentMeans);
    builder_vec_setter!(add_payment_terms, payment_terms, PaymentTerms);
    builder_vec_setter!(add_prepaid_payment, prepaid_payment, Payment);
    builder_vec_setter!(add_allowance_charge, allowance_charge, AllowanceCharge);

    builder_setter!(tax_exchange_rate, tax_exchange_rate, ExchangeRate);
    builder_setter!(pricing_exchange_rate, pricing_exchange_rate, ExchangeRate);
    builder_setter!(payment_exchange_rate, payment_exchange_rate, ExchangeRate);
    builder_setter!(
        payment_alternative_exchange_rate,
        payment_alternative_exchange_rate,
        ExchangeRate
    );

    builder_vec_setter!(add_tax_total, tax_total, TaxTotal);
    builder_vec_setter!(add_withholding_tax_total, withholding_tax_total, TaxTotal);

    pub fn build(self) -> Invoice {
        self.invoice
    }
}

pub struct PartyBuilder {
    party: PartyType,
}

impl PartyBuilder {
    pub fn new() -> Self {
        Self {
            party: PartyType::default(),
        }
    }

    pub fn party_name(mut self, name: impl Into<String>) -> Self {
        let name_obj = PartyName { name: name.into() };
        if let Some(ref mut vec) = self.party.party_name {
            vec.push(name_obj);
        } else {
            self.party.party_name = Some(vec![name_obj]);
        }
        self
    }

    pub fn postal_address(mut self, address: PostalAddress) -> Self {
        self.party.postal_address = Some(address);
        self
    }

    pub fn party_tax_scheme(
        mut self,
        company_id: impl Into<String>,
        tax_scheme_id: impl Into<String>,
    ) -> Self {
        let scheme = PartyTaxScheme {
            company_id: Some(company_id.into().into()),
            tax_scheme: TaxScheme {
                id: Some(tax_scheme_id.into().into()),
            },
        };
        if let Some(ref mut vec) = self.party.party_tax_scheme {
            vec.push(scheme);
        } else {
            self.party.party_tax_scheme = Some(vec![scheme]);
        }
        self
    }

    pub fn party_legal_entity(
        mut self,
        registration_name: impl Into<String>,
        company_id: Option<Identifier>,
    ) -> Self {
        self.party.party_legal_entity = Some(PartyLegalEntity {
            registration_name: Some(registration_name.into()),
            company_id,
            company_legal_form: None,
        });
        self
    }

    pub fn contact(
        mut self,
        name: Option<String>,
        telephone: Option<String>,
        email: Option<String>,
    ) -> Self {
        self.party.contact = Some(Contact {
            name,
            telephone,
            electronic_mail: email,
        });
        self
    }

    pub fn build(self) -> PartyType {
        self.party
    }
}

pub struct MonetaryTotalBuilder {
    total: MonetaryTotal,
}

impl MonetaryTotalBuilder {
    pub fn new() -> Self {
        Self {
            total: MonetaryTotal::default(),
        }
    }

    pub fn line_extension_amount(
        mut self,
        currency_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.total.line_extension_amount = Some(Amount {
            currency_id: currency_id.into(),
            value: value.into(),
            currency_code_list_version_id: None,
        });
        self
    }

    pub fn tax_exclusive_amount(
        mut self,
        currency_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.total.tax_exclusive_amount = Some(Amount {
            currency_id: currency_id.into(),
            value: value.into(),
            currency_code_list_version_id: None,
        });
        self
    }

    pub fn tax_inclusive_amount(
        mut self,
        currency_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.total.tax_inclusive_amount = Some(Amount {
            currency_id: currency_id.into(),
            value: value.into(),
            currency_code_list_version_id: None,
        });
        self
    }

    pub fn payable_amount(
        mut self,
        currency_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.total.payable_amount = Some(Amount {
            currency_id: currency_id.into(),
            value: value.into(),
            currency_code_list_version_id: None,
        });
        self
    }

    pub fn build(self) -> MonetaryTotal {
        self.total
    }
}

pub struct InvoiceLineBuilder {
    line: InvoiceLine,
}

impl InvoiceLineBuilder {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            line: InvoiceLine {
                id: Some(id.into().into()),
                ..Default::default()
            },
        }
    }

    pub fn invoiced_quantity(
        mut self,
        unit_code: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.line.invoiced_quantity = Some(Quantity {
            unit_code: Some(unit_code.into()),
            unit_code_list_id: None,
            unit_code_list_agency_id: None,
            unit_code_list_agency_name: None,
            value: value.into(),
        });
        self
    }

    pub fn line_extension_amount(
        mut self,
        currency_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.line.line_extension_amount = Some(Amount {
            currency_id: currency_id.into(),
            currency_code_list_version_id: None,
            value: value.into(),
        });
        self
    }

    pub fn item(mut self, name: impl Into<String>) -> Self {
        if let Some(ref mut item) = self.line.item {
            item.name = Some(name.into());
        } else {
            self.line.item = Some(Item {
                name: Some(name.into()),
                ..Default::default()
            });
        }
        self
    }

    pub fn tax_category(
        mut self,
        id: impl Into<String>,
        percent: impl Into<String>,
        tax_scheme_id: impl Into<String>,
    ) -> Self {
        let category = ClassifiedTaxCategory {
            id: Some(id.into().into()),
            percent: Some(percent.into()),
            tax_scheme: TaxScheme {
                id: Some(tax_scheme_id.into().into()),
            },
        };

        if let Some(ref mut item) = self.line.item {
            if let Some(ref mut vec) = item.classified_tax_category {
                vec.push(category);
            } else {
                item.classified_tax_category = Some(vec![category]);
            }
        } else {
            self.line.item = Some(Item {
                classified_tax_category: Some(vec![category]),
                ..Default::default()
            });
        }
        self
    }

    pub fn price_amount(
        mut self,
        currency_id: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        self.line.price = Some(Price {
            price_amount: Some(Amount {
                currency_id: currency_id.into(),
                currency_code_list_version_id: None,
                value: value.into(),
            }),
            ..Default::default()
        });
        self
    }

    pub fn build(self) -> InvoiceLine {
        self.line
    }
}

pub struct PostalAddressBuilder {
    address: PostalAddress,
}

impl PostalAddressBuilder {
    pub fn new() -> Self {
        Self {
            address: PostalAddress::default(),
        }
    }

    pub fn street_name(mut self, name: impl Into<String>) -> Self {
        self.address.street_name = Some(name.into());
        self
    }

    pub fn additional_street_name(mut self, name: impl Into<String>) -> Self {
        self.address.additional_street_name = Some(name.into());
        self
    }

    pub fn city_name(mut self, name: impl Into<String>) -> Self {
        self.address.city_name = Some(name.into());
        self
    }

    pub fn postal_zone(mut self, zone: impl Into<String>) -> Self {
        self.address.postal_zone = Some(zone.into());
        self
    }

    pub fn country(mut self, country_code: impl Into<String>) -> Self {
        self.address.country = Some(Country::new(country_code));
        self
    }

    pub fn build(self) -> PostalAddress {
        self.address
    }
}

pub struct TaxTotalBuilder {
    total: TaxTotal,
}

impl TaxTotalBuilder {
    pub fn new() -> Self {
        Self {
            total: TaxTotal::default(),
        }
    }

    pub fn tax_amount(mut self, currency: impl Into<String>, value: impl Into<String>) -> Self {
        self.total.tax_amount = Some(Amount::new(currency, value));
        self
    }

    pub fn add_subtotal(
        mut self,
        taxable_amount: impl Into<String>,
        tax_amount: impl Into<String>,
        currency: impl Into<String>,
        category_id: impl Into<String>,
        percent: impl Into<String>,
        tax_scheme_id: impl Into<String>,
    ) -> Self {
        let currency = currency.into();
        let subtotal = TaxSubtotal {
            taxable_amount: Some(Amount::new(currency.clone(), taxable_amount)),
            tax_amount: Some(Amount::new(currency, tax_amount)),
            tax_category: TaxCategory {
                id: Some(Identifier::new(category_id)),
                percent: Some(percent.into()),
                tax_scheme: TaxScheme {
                    id: Some(Identifier::new(tax_scheme_id)),
                },
                ..Default::default()
            },
            ..Default::default()
        };

        if let Some(ref mut vec) = self.total.tax_subtotal {
            vec.push(subtotal);
        } else {
            self.total.tax_subtotal = Some(vec![subtotal]);
        }
        self
    }

    pub fn build(self) -> TaxTotal {
        self.total
    }
}
