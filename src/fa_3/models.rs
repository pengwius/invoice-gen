use crate::shared::models::{
    Address, ContactData, CurrencyCode, IdentificationData, TRolaPodmiotu3,
    TRolaPodmiotuUpowaznionego, TStatusInfoPodatnika, TaxRate,
};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::RoundingStrategy;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "Faktura")]
#[serde(rename_all = "PascalCase")]
pub struct Invoice {
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
    #[serde(rename = "Podmiot3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject3: Option<Subject3>,
    #[serde(rename = "PodmiotUpowazniony")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_subject: Option<AuthorizedSubject>,
    #[serde(rename = "KSeF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kse_f: Option<PaymentIdentifier>,
    #[serde(rename = "WarunkiTransakcji")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_terms: Option<TransactionTerms>,
    #[serde(rename = "Fa")]
    pub invoice_body: InvoiceBody,
    #[serde(rename = "Zamowienie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(rename = "Stopka")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Footer>,
    #[serde(rename = "Zalacznik")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment: Option<Attachment>,
}

impl Default for Invoice {
    fn default() -> Self {
        Self {
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            xmlns_xsd: "http://www.w3.org/2001/XMLSchema".to_string(),
            xmlns: "http://crd.gov.pl/wzor/2025/06/25/13775/".to_string(),
            header: Header::default(),
            subject1: Subject1::default(),
            subject2: Subject2::default(),
            subject3: None,
            authorized_subject: None,
            kse_f: None,
            transaction_terms: None,
            invoice_body: InvoiceBody::default(),
            order: None,
            footer: None,
            attachment: None,
        }
    }
}

impl Invoice {
    pub fn to_xml(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut inv = self.clone();

        use crate::shared::models::TaxRate;

        let mut net_13_1 = Decimal::ZERO; // 23%
        let mut tax_14_1 = Decimal::ZERO;
        let mut net_13_2 = Decimal::ZERO; // 8%
        let mut tax_14_2 = Decimal::ZERO;
        let mut net_13_3 = Decimal::ZERO; // 5%
        let mut tax_14_3 = Decimal::ZERO;
        let mut net_13_4 = Decimal::ZERO; // taxi / other mapped
        let mut tax_14_4 = Decimal::ZERO;
        let net_13_5 = Decimal::ZERO; // special / margin
        let tax_14_5 = Decimal::ZERO;
        let mut net_13_6_1 = Decimal::ZERO; // 0 domestic
        let mut net_13_6_2 = Decimal::ZERO; // 0 intra
        let mut net_13_6_3 = Decimal::ZERO; // 0 export
        let mut net_13_8 = Decimal::ZERO; // oo

        // Ensure invoice lines have sequential positive line numbers required by TNaturalny (MinExclusive > 0)
        for (idx, line) in inv.invoice_body.lines.iter_mut().enumerate() {
            line.line_number = (idx + 1) as u32;
        }
        let mut total = Decimal::ZERO;

        for line in &inv.invoice_body.lines {
            if let Some(net) = line.net_value {
                total += net;

                let vat = if let Some(rate) = &line.tax_rate {
                    let net_cents = (net * Decimal::new(100, 0))
                        .round_dp_with_strategy(0, rust_decimal::RoundingStrategy::MidpointAwayFromZero)
                        .to_i128()
                        .unwrap_or(0);
                    let vat_cents = (net_cents * rate.basis_points() as i128) / 10000;
                    let vat = Decimal::from_i128_with_scale(vat_cents, 2);
                    match rate {
                        TaxRate::Rate23 => {
                            net_13_1 += net;
                            tax_14_1 += vat;
                        }
                        TaxRate::Rate22 => {
                            net_13_1 += net;
                            tax_14_1 += vat;
                        }
                        TaxRate::Rate3 => {
                            net_13_3 += net;
                            tax_14_3 += vat;
                        }
                        TaxRate::Rate5 => {
                            net_13_3 += net;
                            tax_14_3 += vat;
                        }
                        TaxRate::Rate8 => {
                            net_13_2 += net;
                            tax_14_2 += vat;
                        }
                        TaxRate::Rate4 => {
                            net_13_4 += net;
                            tax_14_4 += vat;
                        }
                        TaxRate::Rate7 => {
                            net_13_2 += net;
                            tax_14_2 += vat;
                        }
                        TaxRate::ZeroKR => {
                            net_13_6_1 += net;
                        }
                        TaxRate::ZeroWDT => {
                            net_13_6_2 += net;
                        }
                        TaxRate::ZeroEX => {
                            net_13_6_3 += net;
                        }
                        TaxRate::Oo => {
                            net_13_8 += net;
                        }
                        TaxRate::Zw | TaxRate::NpI | TaxRate::NpII => {
                            net_13_1 += net;
                            tax_14_1 += vat;
                        }
                        TaxRate::Custom(_) => {
                            net_13_1 += net;
                            tax_14_1 += vat;
                        }
                    }
                    vat
                } else {
                    Decimal::ZERO
                };

                total += vat;
            }
        }

        inv.invoice_body.net_total_basic_rate = if net_13_1 != Decimal::ZERO {
            Some(net_13_1.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.tax_total_basic_rate = if tax_14_1 != Decimal::ZERO {
            Some(tax_14_1.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };

        inv.invoice_body.net_total_reduced_rate_1 = if net_13_2 != Decimal::ZERO {
            Some(net_13_2.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.tax_total_reduced_rate_1 = if tax_14_2 != Decimal::ZERO {
            Some(tax_14_2.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };

        inv.invoice_body.net_total_reduced_rate_2 = if net_13_3 != Decimal::ZERO {
            Some(net_13_3.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.tax_total_reduced_rate_2 = if tax_14_3 != Decimal::ZERO {
            Some(tax_14_3.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };

        inv.invoice_body.net_total_taxi = if net_13_4 != Decimal::ZERO {
            Some(net_13_4.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.tax_total_taxi = if tax_14_4 != Decimal::ZERO {
            Some(tax_14_4.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };

        inv.invoice_body.net_total_special = if net_13_5 != Decimal::ZERO {
            Some(net_13_5.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.tax_total_special = if tax_14_5 != Decimal::ZERO {
            Some(tax_14_5.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };

        inv.invoice_body.net_total_0_domestic = if net_13_6_1 != Decimal::ZERO {
            Some(net_13_6_1.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.net_total_0_intra = if net_13_6_2 != Decimal::ZERO {
            Some(net_13_6_2.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.net_total_0_export = if net_13_6_3 != Decimal::ZERO {
            Some(net_13_6_3.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };

        inv.invoice_body.net_total_oo = if net_13_8 != Decimal::ZERO {
            Some(net_13_8.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero))
        } else {
            None
        };
        inv.invoice_body.total_amount = total.round_dp_with_strategy(2, rust_decimal::RoundingStrategy::MidpointAwayFromZero);

        if inv.invoice_body.invoice_type.is_none() {
            inv.invoice_body.invoice_type = Some("VAT".to_string());
        }

        fn digits_only(s: &str) -> String {
            s.chars().filter(|c| c.is_ascii_digit()).collect()
        }

        if !inv.subject1.identification_data.nip.is_empty() {
            let d = digits_only(&inv.subject1.identification_data.nip);
            if !d.is_empty() {
                inv.subject1.identification_data.nip = d;
            }
        }

        if let Some(s1k) = &mut inv.invoice_body.subject1_k {
            let d = digits_only(&s1k.identification_data.nip);
            if !d.is_empty() {
                s1k.identification_data.nip = d;
            }
        }

        for s2k in &mut inv.invoice_body.subject2_k {
            if let Some(nip_opt) = &s2k.identification_data.nip {
                let d = digits_only(nip_opt);
                if !d.is_empty() {
                    s2k.identification_data.nip = Some(d);
                }
            }
        }

        if let Some(id2) = &mut inv.subject2.identification_data {
            if let Some(n) = &id2.nip {
                let d = digits_only(n);
                if !d.is_empty() {
                    id2.nip = Some(d);
                }
            }
        }

        if let Some(s3) = &mut inv.subject3 {
            if let Some(n) = &s3.identification_data.nip {
                let d = digits_only(n);
                if !d.is_empty() {
                    s3.identification_data.nip = Some(d);
                }
            }
        }
        if let Some(aut) = &mut inv.authorized_subject {
            if let Some(n) = &aut.identification_data.nip {
                let d = digits_only(n);
                if !d.is_empty() {
                    aut.identification_data.nip = Some(d);
                }
            }
        }

        let mut buffer = String::new();
        let serializer = quick_xml::se::Serializer::new(&mut buffer);
        inv.serialize(serializer).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        let xml_decl = "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n";
        let result = format!("{}{}", xml_decl, buffer);
        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Header {
    #[serde(rename = "KodFormularza")]
    pub form_code: FormCode,
    #[serde(rename = "WariantFormularza")]
    pub form_variant: u8,
    #[serde(rename = "DataWytworzeniaFa")]
    pub creation_date: String,
    #[serde(rename = "SystemInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_info: Option<String>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            form_code: FormCode::default(),
            form_variant: 3,
            creation_date: chrono::Local::now()
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
            system_info: Some("invoice-gen-rs".to_string()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
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
            system_code: "FA (3)".to_string(),
            schema_version: "1-0E".to_string(),
            value: "FA".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Subject1 {
    #[serde(rename = "PrefiksPodatnika")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxpayer_prefix: Option<String>,
    #[serde(rename = "NrEORI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori: Option<String>,
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
    #[serde(rename = "StatusInfoPodatnika")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxpayer_status: Option<TStatusInfoPodatnika>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Subject2 {
    #[serde(rename = "NrEORI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori: Option<String>,
    #[serde(rename = "DaneIdentyfikacyjne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_data: Option<IdentificationData2>,
    #[serde(rename = "Adres")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "AdresKoresp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<Address>,
    #[serde(rename = "DaneKontaktowe")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_data: Option<ContactData>,
    #[serde(rename = "NrKlienta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,
    #[serde(skip_serializing)]
    pub buyer_id: Option<String>,
    #[serde(rename = "JST")]
    pub jst: u8,
    #[serde(rename = "GV")]
    pub gv: u8,
}

impl Default for Subject2 {
    fn default() -> Self {
        Self {
            eori: None,
            identification_data: None,
            address: None,
            correspondence_address: None,
            contact_data: None,
            client_number: None,
            buyer_id: None,
            jst: 2,
            gv: 2,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Subject3 {
    #[serde(rename = "NrEORI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori: Option<String>,
    #[serde(rename = "DaneIdentyfikacyjne")]
    pub identification_data: IdentificationData2,
    #[serde(rename = "Adres")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "AdresKoresp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<Address>,
    #[serde(rename = "DaneKontaktowe")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_data: Vec<ContactData>,
    #[serde(rename = "Rola")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<TRolaPodmiotu3>,
    #[serde(rename = "RolaInna")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_role_flag: Option<u8>,
    #[serde(rename = "OpisRoli")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_description: Option<String>,
    #[serde(rename = "Udzial")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
    #[serde(rename = "NrKlienta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,
    #[serde(rename = "IDNabywcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AuthorizedSubject {
    #[serde(rename = "NrEORI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori: Option<String>,
    #[serde(rename = "DaneIdentyfikacyjne")]
    pub identification_data: IdentificationData2,
    #[serde(rename = "Adres")]
    pub address: Address,
    #[serde(rename = "AdresKoresp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<Address>,
    #[serde(rename = "DaneKontaktowe")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_data: Vec<AuthorizedSubjectContactData>,
    #[serde(rename = "RolaPU")]
    pub role: TRolaPodmiotuUpowaznionego,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct AuthorizedSubjectContactData {
    #[serde(rename = "EmailPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "TelefonPU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceBody {
    #[serde(rename = "KodWaluty")]
    pub currency_code: CurrencyCode,
    #[serde(rename = "P_1")]
    pub issue_date: String,
    #[serde(rename = "P_2")]
    pub invoice_number: String,

    // Basic Rate 23%
    #[serde(rename = "P_13_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_basic_rate: Option<Decimal>,
    #[serde(rename = "P_14_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_basic_rate: Option<Decimal>,
    #[serde(rename = "P_14_1W")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_basic_rate_foreign: Option<Decimal>,

    // Reduced Rate 8%
    #[serde(rename = "P_13_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_reduced_rate_1: Option<Decimal>,
    #[serde(rename = "P_14_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_reduced_rate_1: Option<Decimal>,
    #[serde(rename = "P_14_2W")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_reduced_rate_1_foreign: Option<Decimal>,

    // Reduced Rate 5%
    #[serde(rename = "P_13_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_reduced_rate_2: Option<Decimal>,
    #[serde(rename = "P_14_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_reduced_rate_2: Option<Decimal>,
    #[serde(rename = "P_14_3W")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_reduced_rate_2_foreign: Option<Decimal>,

    // Taxi Lump Sum
    #[serde(rename = "P_13_4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_taxi: Option<Decimal>,
    #[serde(rename = "P_14_4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_taxi: Option<Decimal>,
    #[serde(rename = "P_14_4W")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_taxi_foreign: Option<Decimal>,

    // Special Procedure (Margin)
    #[serde(rename = "P_13_5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_special: Option<Decimal>,
    #[serde(rename = "P_14_5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_special: Option<Decimal>,

    // 0% Rates
    #[serde(rename = "P_13_6_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_0_domestic: Option<Decimal>,
    #[serde(rename = "P_13_6_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_0_intra: Option<Decimal>,
    #[serde(rename = "P_13_6_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_0_export: Option<Decimal>,
    #[serde(rename = "P_13_8")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_oo: Option<Decimal>,
    #[serde(rename = "P_15")]
    pub total_amount: Decimal,
    #[serde(rename = "Adnotacje")]
    pub annotations: Annotations,
    #[serde(rename = "RodzajFaktury")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<String>,
    #[serde(rename = "FaWiersz")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub lines: Vec<InvoiceLine>,
    #[serde(rename = "Podmiot1K")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject1_k: Option<Subject1K>,
    #[serde(rename = "Podmiot2K")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject2_k: Vec<Subject2K>,
    #[serde(rename = "P_15ZK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_paid_before_correction: Option<Decimal>,
    #[serde(rename = "Rozliczenie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement: Option<Settlement>,
    #[serde(rename = "Platnosc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
}

impl Default for InvoiceBody {
    fn default() -> Self {
        Self {
            currency_code: CurrencyCode::new("PLN"),
            issue_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            invoice_number: String::new(),
            lines: Vec::new(),
            net_total_basic_rate: None,
            tax_total_basic_rate: None,
            tax_total_basic_rate_foreign: None,
            net_total_reduced_rate_1: None,
            tax_total_reduced_rate_1: None,
            tax_total_reduced_rate_1_foreign: None,
            net_total_reduced_rate_2: None,
            tax_total_reduced_rate_2: None,
            tax_total_reduced_rate_2_foreign: None,
            net_total_taxi: None,
            tax_total_taxi: None,
            tax_total_taxi_foreign: None,
            net_total_special: None,
            tax_total_special: None,
            net_total_0_domestic: None,
            net_total_0_intra: None,
            net_total_0_export: None,
            net_total_oo: None,
            total_amount: Decimal::ZERO,
            invoice_type: None,
            annotations: Annotations::default(),
            subject1_k: None,
            subject2_k: Vec::new(),
            amount_paid_before_correction: None,
            settlement: None,
            payment: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum GTU {
    #[serde(rename = "GTU_01")]
    Gtu01,
    #[serde(rename = "GTU_02")]
    Gtu02,
    #[serde(rename = "GTU_03")]
    Gtu03,
    #[serde(rename = "GTU_04")]
    Gtu04,
    #[serde(rename = "GTU_05")]
    Gtu05,
    #[serde(rename = "GTU_06")]
    Gtu06,
    #[serde(rename = "GTU_07")]
    Gtu07,
    #[serde(rename = "GTU_08")]
    Gtu08,
    #[serde(rename = "GTU_09")]
    Gtu09,
    #[serde(rename = "GTU_10")]
    Gtu10,
    #[serde(rename = "GTU_11")]
    Gtu11,
    #[serde(rename = "GTU_12")]
    Gtu12,
    #[serde(rename = "GTU_13")]
    Gtu13,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Procedure {
    #[serde(rename = "WSTO_EE")]
    WstoEe,
    #[serde(rename = "IED")]
    Ied,
    #[serde(rename = "TP")]
    Tp,
    #[serde(rename = "TT_WNT")]
    TtWnt,
    #[serde(rename = "TT_D")]
    TtD,
    #[serde(rename = "MR_T")]
    MrT,
    #[serde(rename = "MR_UZ")]
    MrUz,
    #[serde(rename = "I_42")]
    I42,
    #[serde(rename = "I_63")]
    I63,
    #[serde(rename = "B_SPV")]
    BSpv,
    #[serde(rename = "B_SPV_DOSTAWA")]
    BSpvDostawa,
    #[serde(rename = "B_MPV_PROWIZJA")]
    BMpvProwizja,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceLine {
    #[serde(rename = "NrWierszaFa")]
    pub line_number: u32,
    #[serde(rename = "UU_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uu_id: Option<String>,
    #[serde(rename = "P_6A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_date: Option<String>,
    #[serde(rename = "P_7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Indeks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "GTIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtin: Option<String>,
    #[serde(rename = "PKWiU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkwiu: Option<String>,
    #[serde(rename = "CN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn: Option<String>,
    #[serde(rename = "PKOB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkob: Option<String>,
    #[serde(rename = "P_8A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "P_8B")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(rename = "P_9A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_net_price: Option<Decimal>,
    #[serde(rename = "P_9B")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_gross_price: Option<Decimal>,
    #[serde(rename = "P_10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Decimal>,
    #[serde(rename = "P_11")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_value: Option<Decimal>,
    #[serde(rename = "P_12")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<TaxRate>,
    #[serde(rename = "P_12_XII")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_procedure_xii: Option<u8>,
    #[serde(rename = "P_12_Zal_15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_appendix_15: Option<u8>,
    #[serde(rename = "KwotaAkcyzy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excise_amount: Option<Decimal>,
    #[serde(rename = "GTU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtu: Option<GTU>,
    #[serde(rename = "Procedura")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure: Option<Procedure>,
    #[serde(rename = "KursWaluty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_rate: Option<Decimal>,
    #[serde(rename = "StanPrzed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_before: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Annotations {
    #[serde(rename = "P_16")]
    pub cash_accounting: u8,
    #[serde(rename = "P_17")]
    pub self_billing: u8,
    #[serde(rename = "P_18")]
    pub reverse_charge: u8,
    #[serde(rename = "P_18A")]
    pub split_payment: u8,
    #[serde(rename = "Zwolnienie")]
    pub exemption: Exemption,
    #[serde(rename = "NoweSrodkiTransportu")]
    pub new_transport: NewTransport,
    #[serde(rename = "P_23")]
    pub simplified_procedure: u8,
    #[serde(rename = "PMarzy")]
    pub margin_procedures: MarginProcedures,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Settlement {
    #[serde(rename = "Odliczenia")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub deductions: Vec<Deduction>,
    #[serde(rename = "SumaOdliczen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deductions_sum: Option<Decimal>,
    #[serde(rename = "DoZaplaty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_pay: Option<Decimal>,
    #[serde(rename = "DoRozliczenia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_settle: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Deduction {
    #[serde(rename = "Kwota")]
    pub amount: Decimal,
    #[serde(rename = "Powod")]
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    #[serde(rename = "Zaplacono")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid: Option<u8>,
    #[serde(rename = "DataZaplaty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_date: Option<String>,
    #[serde(rename = "ZnacznikZaplatyCzesciowej")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment_flag: Option<u8>,
    #[serde(rename = "ZaplataCzesciowa")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub partial_payments: Vec<PartialPayment>,
    #[serde(rename = "TerminPlatnosci")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub payment_terms: Vec<PaymentTerm>,
    #[serde(rename = "FormaPlatnosci")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<u8>,
    #[serde(rename = "PlatnoscInna")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_payment: Option<u8>,
    #[serde(rename = "OpisPlatnosci")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_description: Option<String>,
    #[serde(rename = "RachunekBankowy")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bank_accounts: Vec<BankAccount>,
    #[serde(rename = "RachunekBankowyFaktora")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub factor_bank_accounts: Vec<BankAccount>,
    #[serde(rename = "Skonto")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    #[serde(rename = "LinkDoPlatnosci")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<String>,
    #[serde(rename = "IPKSeF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ksef: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PartialPayment {
    #[serde(rename = "KwotaZaplatyCzesciowej")]
    pub amount: Decimal,
    #[serde(rename = "DataZaplatyCzesciowej")]
    pub date: String,
    #[serde(rename = "FormaPlatnosci")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<u8>,
    #[serde(rename = "PlatnoscInna")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_payment: Option<u8>,
    #[serde(rename = "OpisPlatnosci")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentTerm {
    #[serde(rename = "Termin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "TerminOpis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<PaymentTermDescription>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentTermDescription {
    #[serde(rename = "Ilosc")]
    pub amount: i64,
    #[serde(rename = "Jednostka")]
    pub unit: String,
    #[serde(rename = "ZdarzeniePoczatkowe")]
    pub start_event: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BankAccount {
    #[serde(rename = "NrRB")]
    pub account_number: String,
    #[serde(rename = "SWIFT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<String>,
    #[serde(rename = "RachunekWlasnyBanku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_account: Option<u8>,
    #[serde(rename = "NazwaBanku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "OpisRachunku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Discount {
    #[serde(rename = "WarunkiSkonta")]
    pub conditions: String,
    #[serde(rename = "WysokoscSkonta")]
    pub amount: String,
}

impl Default for Annotations {
    fn default() -> Self {
        Self {
            new_transport: NewTransport::default(),
            simplified_procedure: 2,
            margin_procedures: MarginProcedures::default(),
            cash_accounting: 2,
            self_billing: 2,
            reverse_charge: 2,
            split_payment: 2,
            exemption: Exemption::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Exemption {
    #[serde(rename = "P_19")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_delivery: Option<u8>,
    #[serde(rename = "P_19A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_legal: Option<String>,
    #[serde(rename = "P_19B")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_directive: Option<String>,
    #[serde(rename = "P_19C")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_other: Option<String>,
    #[serde(rename = "P_19N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_delivery_none: Option<u8>,
}

impl Default for Exemption {
    fn default() -> Self {
        Self {
            exempt_delivery: None,
            basis_legal: None,
            basis_directive: None,
            basis_other: None,
            exempt_delivery_none: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NewTransport {
    #[serde(rename = "NowySrodekTransportu")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<NewTransportItem>,
    #[serde(rename = "P_22N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<u8>,
}

impl Default for NewTransport {
    fn default() -> Self {
        Self {
            items: Vec::new(),
            none: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct NewTransportItem {
    #[serde(rename = "P_22B1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,
    #[serde(rename = "P_22B2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body_number: Option<String>,
    #[serde(rename = "P_22B3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chassis_number: Option<String>,
    #[serde(rename = "P_22B4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frame_number: Option<String>,
    #[serde(rename = "P_22BT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_type: Option<String>,
    #[serde(rename = "P_22C")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_water: Option<String>,
    #[serde(rename = "P_22C1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hull_number: Option<String>,
    #[serde(rename = "P_22D")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_air: Option<String>,
    #[serde(rename = "P_22D1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factory_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct MarginProcedures {
    #[serde(rename = "P_PMarzy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure_flag: Option<u8>,
    #[serde(rename = "P_PMarzy_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel: Option<u8>,
    #[serde(rename = "P_PMarzy_3_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_goods: Option<u8>,
    #[serde(rename = "P_PMarzy_3_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub art: Option<u8>,
    #[serde(rename = "P_PMarzy_3_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectors: Option<u8>,
    #[serde(rename = "P_PMarzyN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub none: Option<u8>,
}

impl Default for MarginProcedures {
    fn default() -> Self {
        Self {
            procedure_flag: None,
            travel: None,
            used_goods: None,
            art: None,
            collectors: None,
            none: Some(1),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Subject1K {
    #[serde(rename = "PrefiksPodatnika")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxpayer_prefix: Option<String>,
    #[serde(rename = "DaneIdentyfikacyjne")]
    pub identification_data: IdentificationData,
    #[serde(rename = "Adres")]
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Subject2K {
    #[serde(rename = "DaneIdentyfikacyjne")]
    pub identification_data: IdentificationData2,
    #[serde(rename = "Adres")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    #[serde(rename = "IDNabywcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct IdentificationData2 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_vat_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_id: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl serde::Serialize for IdentificationData2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("DaneIdentyfikacyjne", 6)?;

        if let Some(n) = &self.nip {
            let digits: String = n.chars().filter(|c| c.is_ascii_digit()).collect();
            if !digits.is_empty() {
                state.serialize_field("NIP", &digits)?;
            }
        }

        if let Some(k) = &self.eu_code {
            state.serialize_field("KodUE", k)?;
        }
        if let Some(v) = &self.eu_vat_number {
            state.serialize_field("NrVatUE", v)?;
        }
        if let Some(id) = &self.id_number {
            state.serialize_field("NrID", id)?;
        }
        if let Some(b) = &self.no_id {
            state.serialize_field("BrakID", b)?;
        }
        if let Some(nm) = &self.name {
            state.serialize_field("Nazwa", nm)?;
        }

        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Order {
    #[serde(rename = "WartoscZamowienia")]
    pub order_value: Decimal,
    #[serde(rename = "ZamowienieWiersz")]
    pub lines: Vec<OrderLine>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OrderLine {
    #[serde(rename = "NrWierszaZam")]
    pub line_number: u32,
    #[serde(rename = "UU_IDZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uu_id: Option<String>,
    #[serde(rename = "P_7Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "IndeksZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,
    #[serde(rename = "GTINZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtin: Option<String>,
    #[serde(rename = "PKWiUZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkwiu_code: Option<String>,
    #[serde(rename = "CNZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_code: Option<String>,
    #[serde(rename = "PKOBZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkob_code: Option<String>,
    #[serde(rename = "P_8AZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "P_8BZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<Decimal>,
    #[serde(rename = "P_9AZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_net_price: Option<Decimal>,
    #[serde(rename = "P_11NettoZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_value: Option<Decimal>,
    #[serde(rename = "P_11VatZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_value: Option<Decimal>,
    #[serde(rename = "P_12Z")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<TaxRate>,
    #[serde(rename = "P_12Z_XII")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_procedure_xii: Option<Decimal>,
    #[serde(rename = "P_12Z_Zal_15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_appendix_15: Option<u8>,
    #[serde(rename = "GTUZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtu: Option<GTU>,
    #[serde(rename = "ProceduraZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub procedure: Option<Procedure>,
    #[serde(rename = "KwotaAkcyzyZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excise_amount: Option<Decimal>,
    #[serde(rename = "StanPrzedZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_before: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentIdentifier {
    #[serde(rename = "NrKSeF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ksef_number: Option<String>,
    #[serde(rename = "URL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "IPKSeF")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_ksef: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionTerms {
    #[serde(rename = "Umowy")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contracts: Vec<TransactionContract>,
    #[serde(rename = "Zamowienia")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub orders: Vec<TransactionOrder>,
    #[serde(rename = "NrPartiiTowaru")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub batch_numbers: Vec<String>,
    #[serde(rename = "WarunkiDostawy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<String>,
    #[serde(rename = "KursUmowny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_exchange_rate: Option<Decimal>,
    #[serde(rename = "WalutaUmowna")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_currency: Option<CurrencyCode>,
    #[serde(rename = "Transport")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transport: Vec<Transport>,
    #[serde(rename = "PodmiotPosredniczacy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intermediary_subject: Option<u8>,
}

impl Default for TransactionTerms {
    fn default() -> Self {
        Self {
            contracts: Vec::new(),
            orders: Vec::new(),
            batch_numbers: Vec::new(),
            delivery_terms: None,
            contract_exchange_rate: None,
            contract_currency: None,
            transport: Vec::new(),
            intermediary_subject: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionContract {
    #[serde(rename = "DataUmowy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "NrUmowy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionOrder {
    #[serde(rename = "DataZamowienia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "NrZamowienia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TRodzajTransportu {
    #[serde(rename = "1")]
    Morski,
    #[serde(rename = "2")]
    Kolejowy,
    #[serde(rename = "3")]
    Drogowy,
    #[serde(rename = "4")]
    Lotniczy,
    #[serde(rename = "5")]
    Pocztowa,
    #[serde(rename = "7")]
    InstalacjePrzesylowe,
    #[serde(rename = "8")]
    ZeglugaSrodladowa,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TLadunek {
    #[serde(rename = "1")]
    Banka,
    #[serde(rename = "2")]
    Beczka,
    #[serde(rename = "3")]
    Butla,
    #[serde(rename = "4")]
    Karton,
    #[serde(rename = "5")]
    Kanister,
    #[serde(rename = "6")]
    Klatka,
    #[serde(rename = "7")]
    Kontener,
    #[serde(rename = "8")]
    Kosz,
    #[serde(rename = "9")]
    Lubianka,
    #[serde(rename = "10")]
    OpakowanieZbiorcze,
    #[serde(rename = "11")]
    Paczka,
    #[serde(rename = "12")]
    Pakiet,
    #[serde(rename = "13")]
    Paleta,
    #[serde(rename = "14")]
    Pojemnik,
    #[serde(rename = "15")]
    PojemnikDoLadunkowMasowychStalych,
    #[serde(rename = "16")]
    PojemnikDoLadunkowMasowychPlynn,
    #[serde(rename = "17")]
    Pudelko,
    #[serde(rename = "18")]
    Puszka,
    #[serde(rename = "19")]
    Skrzynia,
    #[serde(rename = "20")]
    Worek,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Transport {
    #[serde(rename = "RodzajTransportu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_type: Option<TRodzajTransportu>,
    #[serde(rename = "TransportInny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_transport_flag: Option<u8>,
    #[serde(rename = "OpisInnegoTransportu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_transport_desc: Option<String>,
    #[serde(rename = "Przewoznik")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<Carrier>,
    #[serde(rename = "NrZleceniaTransportu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_order_number: Option<String>,
    #[serde(rename = "OpisLadunku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cargo_desc: Option<TLadunek>,
    #[serde(rename = "LadunekInny")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_cargo_flag: Option<u8>,
    #[serde(rename = "OpisInnegoLadunku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_cargo_desc: Option<String>,
    #[serde(rename = "JednostkaOpakowania")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_unit: Option<String>,
    #[serde(rename = "DataGodzRozpTransportu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_start: Option<String>,
    #[serde(rename = "DataGodzZakTransportu")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transport_end: Option<String>,
    #[serde(rename = "WysylkaZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_from: Option<Address>,
    #[serde(rename = "WysylkaPrzez")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub shipment_via: Vec<Address>,
    #[serde(rename = "WysylkaDo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipment_to: Option<Address>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Carrier {
    #[serde(rename = "DaneIdentyfikacyjne")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_data: Option<IdentificationData2>,
    #[serde(rename = "AdresPrzewoznika")]
    pub address: Address,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Footer {
    #[serde(rename = "Informacje")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub information: Vec<FooterInfo>,
    #[serde(rename = "Rejestry")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub registers: Vec<FooterRegister>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FooterInfo {
    #[serde(rename = "StopkaFaktury")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct FooterRegister {
    #[serde(rename = "PelnaNazwa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    #[serde(rename = "KRS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub krs: Option<String>,
    #[serde(rename = "REGON")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regon: Option<String>,
    #[serde(rename = "BDO")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bdo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Attachment {
    #[serde(rename = "BlokDanych")]
    pub data_blocks: Vec<DataBlock>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct DataBlock {
    #[serde(rename = "ZNaglowek")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(rename = "MetaDane")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<Metadata>,
    #[serde(rename = "Tekst")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextBlock>,
    #[serde(rename = "Tabela")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tables: Vec<Table>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Metadata {
    #[serde(rename = "ZKlucz")]
    pub key: String,
    #[serde(rename = "ZWartosc")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TextBlock {
    #[serde(rename = "Akapit")]
    pub paragraphs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct Table {
    #[serde(rename = "TMetaDane")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<TableMetadata>,
    #[serde(rename = "Opis")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "TNaglowek")]
    pub header: TableHeader,
    #[serde(rename = "Wiersz")]
    pub rows: Vec<TableRow>,
    #[serde(rename = "Suma")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<TableSummary>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TableMetadata {
    #[serde(rename = "TKlucz")]
    pub key: String,
    #[serde(rename = "TWartosc")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TableHeader {
    #[serde(rename = "Kol")]
    pub columns: Vec<TableColumn>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TableColumnType {
    Date,
    Datetime,
    Dec,
    Int,
    Time,
    Txt,
}

impl Default for TableColumnType {
    fn default() -> Self {
        TableColumnType::Txt
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TableColumn {
    #[serde(rename = "NKom")]
    pub content: String,
    #[serde(rename = "@Typ")]
    pub type_: TableColumnType,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TableRow {
    #[serde(rename = "WKom")]
    pub cells: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TableSummary {
    #[serde(rename = "SKom")]
    pub cells: Vec<String>,
}
