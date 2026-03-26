use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use chrono::Local;
use crate::shared::models::{Address, ContactData, CurrencyCode, IdentificationData};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename = "Faktura")]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceRR {
    #[serde(rename = "@xmlns:xsi")]
    pub xmlns_xsi: String,
    #[serde(rename = "@xmlns:xsd")]
    pub xmlns_xsd: String,
    #[serde(rename = "@xmlns")]
    pub xmlns: String,

    #[serde(rename = "Naglowek")]
    pub header: Header,

    #[serde(rename = "Podmiot1")]
    pub subject1: Subject1,
    #[serde(rename = "Podmiot2")]
    pub subject2: Subject2,

    #[serde(rename = "FakturaRR")]
    pub invoice_body: InvoiceBody,

    #[serde(rename = "Stopka")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Footer>,
}

impl Default for InvoiceRR {
    fn default() -> Self {
        Self {
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            xmlns_xsd: "http://www.w3.org/2001/XMLSchema".to_string(),
            xmlns: "http://crd.gov.pl/wzor/2026/03/06/14189/".to_string(),
            header: Header::default(),
            subject1: Subject1::default(),
            subject2: Subject2::default(),
            invoice_body: InvoiceBody::default(),
            footer: None,
        }
    }
}

impl InvoiceRR {
    pub fn to_xml(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        let serializer = quick_xml::se::Serializer::new(&mut buffer);
        self.serialize(serializer)?;

        let xml_decl = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n";
        let result = format!("{}{}", xml_decl, buffer);

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    #[serde(rename = "KodFormularza")]
    pub form_code: FormCode,
    #[serde(rename = "WariantFormularza")]
    pub form_variant: u8,
    #[serde(rename = "DataWytworzeniaFa")]
    pub creation_date: String, // ISO8601
    #[serde(rename = "SystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<String>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            form_code: FormCode::default(),
            form_variant: 1,
            creation_date: Local::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
            system_info: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FormCode {
    #[serde(rename = "@kodSystemowy")]
    pub system_code: String,
    #[serde(rename = "@wersjaSchemy")]
    pub schema_version: String,
    #[serde(rename = "$value")]
    pub value: String,
}

impl Default for FormCode {
    fn default() -> Self {
        Self {
            system_code: "FA_RR (1)".to_string(),
            schema_version: "1-1E".to_string(),
            value: "FA_RR".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Subject1 {
    #[serde(rename = "DaneIdentyfikacyjne")]
    pub identification_data: IdentificationData,
    #[serde(rename = "Adres")]
    pub address: Address,
    #[serde(rename = "AdresKoresp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<Address>,
    #[serde(rename = "DaneKontaktowe")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_data: Vec<ContactData>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Subject2 {
    #[serde(rename = "DaneIdentyfikacyjne")]
    pub identification_data: IdentificationData,
    #[serde(rename = "Adres")]
    pub address: Address,
    #[serde(rename = "AdresKoresp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<Address>,
    #[serde(rename = "DaneKontaktowe")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_data: Vec<ContactData>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceBody {
    #[serde(rename = "KodWaluty")]
    pub currency_code: CurrencyCode,
    #[serde(rename = "P_4B")]
    pub issue_date: String,
    #[serde(rename = "P_4C")]
    pub invoice_number: String,
    #[serde(rename = "P_11_1")]
    pub net_value: Decimal,
    #[serde(rename = "P_11_2")]
    pub tax_value: Decimal,
    #[serde(rename = "P_12_1")]
    pub total_value: Decimal,
    #[serde(rename = "P_12_1W", skip_serializing_if = "Option::is_none")]
    pub total_value_pln: Option<Decimal>,
    #[serde(rename = "P_12_2")]
    pub total_value_words: String,
    #[serde(rename = "RodzajFaktury")]
    pub invoice_type: InvoiceType,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum InvoiceType {
    #[serde(rename = "VAT_RR")]
    VatRr,
    #[serde(rename = "KOR_VAT_RR")]
    KorVatRr,
}

impl Default for InvoiceType {
    fn default() -> Self {
        InvoiceType::VatRr
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Footer {
    #[serde(rename = "Informacje", skip_serializing_if = "Option::is_none")]
    pub informacje: Option<Vec<FooterInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FooterInfo {
    #[serde(rename = "StopkaFaktury", skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
}
