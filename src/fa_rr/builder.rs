use super::models::{InvoiceRR, Header, Subject1, Subject2, Footer};
use rust_decimal::Decimal;
use chrono::NaiveDate;
use crate::shared::models::{Address, ContactData, CurrencyCode, IdentificationData};

pub struct InvoiceRRBuilder {
    invoice: InvoiceRR,
}

impl InvoiceRRBuilder {
    pub fn new() -> Self {
        Self {
            invoice: InvoiceRR::default(),
        }
    }

    pub fn set_header(mut self, header: Header) -> Self {
        self.invoice.header = header;
        self
    }

    pub fn set_subject1(mut self, subject1: Subject1) -> Self {
        self.invoice.subject1 = subject1;
        self
    }

    pub fn set_subject2(mut self, subject2: Subject2) -> Self {
        self.invoice.subject2 = subject2;
        self
    }

    pub fn set_currency(mut self, currency: &str) -> Self {
        self.invoice.invoice_body.currency_code = CurrencyCode::new(currency);
        self
    }

    pub fn set_issue_date(mut self, date: NaiveDate) -> Self {
        self.invoice.invoice_body.issue_date = date.format("%Y-%m-%d").to_string();
        self
    }

    pub fn set_invoice_number(mut self, number: &str) -> Self {
        self.invoice.invoice_body.invoice_number = number.to_string();
        self
    }

    pub fn set_net_value(mut self, value: Decimal) -> Self {
        self.invoice.invoice_body.net_value = value;
        self
    }

    pub fn set_tax_value(mut self, value: Decimal) -> Self {
        self.invoice.invoice_body.tax_value = value;
        self
    }

    pub fn set_total_value(mut self, value: Decimal) -> Self {
        self.invoice.invoice_body.total_value = value;
        self
    }

    pub fn set_total_value_pln(mut self, value: Decimal) -> Self {
        self.invoice.invoice_body.total_value_pln = Some(value);
        self
    }

    pub fn set_total_value_words<S: Into<String>>(mut self, value: S) -> Self {
        self.invoice.invoice_body.total_value_words = value.into();
        self
    }

    pub fn set_footer(mut self, footer: Footer) -> Self {
        self.invoice.footer = Some(footer);
        self
    }

    pub fn set_invoice_type(mut self, invoice_type: super::models::InvoiceType) -> Self {
        self.invoice.invoice_body.invoice_type = invoice_type;
        self
    }

    pub fn build(mut self) -> InvoiceRR {
        if self.invoice.invoice_body.invoice_type != super::models::InvoiceType::VatRr
            && self.invoice.invoice_body.invoice_type != super::models::InvoiceType::KorVatRr
        {
            self.invoice.invoice_body.invoice_type = super::models::InvoiceType::VatRr;
        }
        self.invoice
    }
}

pub struct Subject1Builder {
    identification_data: Option<IdentificationData>,
    address: Option<Address>,
    correspondence_address: Option<Address>,
    contact_data: Vec<ContactData>,
}

impl Subject1Builder {
    pub fn new(identification_data: IdentificationData) -> Self {
        Self {
            identification_data: Some(identification_data),
            address: None,
            correspondence_address: None,
            contact_data: Vec::new(),
        }
    }

    pub fn set_address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn set_correspondence_address(mut self, address: Address) -> Self {
        self.correspondence_address = Some(address);
        self
    }

    pub fn add_contact(mut self, contact: ContactData) -> Self {
        self.contact_data.push(contact);
        self
    }

    pub fn build(self) -> Subject1 {
        Subject1 {
            identification_data: self.identification_data.expect("IdentificationData is required for Subject1"),
            address: self.address.expect("Address is required for Subject1"),
            correspondence_address: self.correspondence_address,
            contact_data: self.contact_data,
        }
    }
}

pub struct Subject2Builder {
    identification_data: Option<IdentificationData>,
    address: Option<Address>,
    correspondence_address: Option<Address>,
    contact_data: Vec<ContactData>,
}

impl Subject2Builder {
    pub fn new(identification_data: IdentificationData) -> Self {
        Self {
            identification_data: Some(identification_data),
            address: None,
            correspondence_address: None,
            contact_data: Vec::new(),
        }
    }

    pub fn set_address(mut self, address: Address) -> Self {
        self.address = Some(address);
        self
    }

    pub fn set_correspondence_address(mut self, address: Address) -> Self {
        self.correspondence_address = Some(address);
        self
    }

    pub fn add_contact(mut self, contact: ContactData) -> Self {
        self.contact_data.push(contact);
        self
    }

    pub fn build(self) -> Subject2 {
        Subject2 {
            identification_data: self.identification_data.expect("IdentificationData is required for Subject2"),
            address: self.address.expect("Address is required for Subject2"),
            correspondence_address: self.correspondence_address,
            contact_data: self.contact_data,
        }
    }
}

pub struct AddressBuilder {
    country_code: Option<String>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    gln: Option<String>,
}

impl AddressBuilder {
    pub fn new() -> Self {
        Self {
            country_code: None,
            address_line_1: None,
            address_line_2: None,
            gln: None,
        }
    }

    pub fn set_country_code(mut self, country_code: &str) -> Self {
        self.country_code = Some(country_code.to_string());
        self
    }

    pub fn set_address_line_1(mut self, line: &str) -> Self {
        self.address_line_1 = Some(line.to_string());
        self
    }

    pub fn set_line2(mut self, line2: &str) -> Self {
        self.address_line_2 = Some(line2.to_string());
        self
    }

    pub fn set_gln(mut self, gln: &str) -> Self {
        self.gln = Some(gln.to_string());
        self
    }

    pub fn build(self) -> Address {
        Address {
            country_code: self.country_code
                .map(crate::shared::models::CountryCode::new)
                .expect("Country code is required for Address"),
            address_line_1: self.address_line_1.expect("Address line 1 is required for Address"),
            address_line_2: self.address_line_2,
            gln: self.gln,
        }
    }
}

pub struct FooterBuilder {
    footer_text: Option<String>,
}

impl FooterBuilder {
    pub fn new() -> Self {
        Self { footer_text: None }
    }

    pub fn set_footer_text(mut self, text: &str) -> Self {
        self.footer_text = Some(text.to_string());
        self
    }

    pub fn build(self) -> Footer {
        use super::models::{Footer, FooterInfo};
        Footer {
            informacje: self.footer_text.map(|txt| vec![FooterInfo { footer_text: Some(txt) }]),
        }
    }
}
