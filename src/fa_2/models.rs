use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub subject3: Vec<Subject3>,

    #[serde(rename = "Fa")]
    pub invoice_body: InvoiceBody,

    #[serde(rename = "Stopka")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<Footer>,
}

impl Default for Invoice {
    fn default() -> Self {
        Self {
            xmlns_xsi: "http://www.w3.org/2001/XMLSchema-instance".to_string(),
            xmlns_xsd: "http://www.w3.org/2001/XMLSchema".to_string(),
            xmlns: "http://crd.gov.pl/wzor/2023/06/29/12648/".to_string(),
            header: Header::default(),
            subject1: Subject1::default(),
            subject2: Subject2::default(),
            subject3: Vec::new(),
            invoice_body: InvoiceBody::default(),
            footer: None,
        }
    }
}

impl Invoice {
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
            form_variant: 2,
            creation_date: chrono::Local::now()
                .format("%Y-%m-%dT%H:%M:%SZ")
                .to_string(),
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
            system_code: "FA (2)".to_string(),
            schema_version: "1-0E".to_string(),
            value: "FA".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
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

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Subject2 {
    #[serde(rename = "NrEORI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori: Option<String>,

    pub nip: String,
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_vat_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_id_country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_id_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_id_flag: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub correspondence_address: Option<Address>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub contact_data: Vec<ContactData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_data: Option<IdentificationData>,
}

impl Default for Subject2 {
    fn default() -> Self {
        Self {
            eori: None,
            nip: String::new(),
            name: None,
            eu_country_code: None,
            eu_vat_number: None,
            other_id_country_code: None,
            other_id_number: None,
            no_id_flag: None,
            address: None,
            correspondence_address: None,
            contact_data: Vec::new(),
            client_number: None,
            buyer_id: None,
            identification_data: None,
        }
    }
}

impl serde::Serialize for Subject2 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("Podmiot2", 12)?;

        if let Some(eori) = &self.eori {
            state.serialize_field("NrEORI", eori)?;
        } else {
            if let Some(iddata) = &self.identification_data {
                state.serialize_field("DaneIdentyfikacyjne", iddata)?;
            } else {
                let temp = IdentificationData {
                    nip: self.nip.clone(),
                    name: self.name.clone().unwrap_or_default(),
                };
                if !temp.nip.is_empty() || !temp.name.is_empty() {
                    state.serialize_field("DaneIdentyfikacyjne", &temp)?;
                }
            }

            if let Some(eu_country_code) = &self.eu_country_code {
                state.serialize_field("KodUE", eu_country_code)?;
            }
            if let Some(eu_vat_number) = &self.eu_vat_number {
                state.serialize_field("NrVatUE", eu_vat_number)?;
            }
        }

        if let Some(k) = &self.other_id_country_code {
            state.serialize_field("KodKraju", k)?;
        }
        if let Some(n) = &self.other_id_number {
            state.serialize_field("NrID", n)?;
        }
        if let Some(b) = &self.no_id_flag {
            state.serialize_field("BrakID", b)?;
        }
        if let Some(addr) = &self.address {
            state.serialize_field("Adres", addr)?;
        }
        if let Some(caddr) = &self.correspondence_address {
            state.serialize_field("AdresKoresp", caddr)?;
        }
        if !self.contact_data.is_empty() {
            state.serialize_field("DaneKontaktowe", &self.contact_data)?;
        }
        if let Some(cn) = &self.client_number {
            state.serialize_field("NrKlienta", cn)?;
        }
        if let Some(id) = &self.buyer_id {
            state.serialize_field("IDNabywcy", id)?;
        }

        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Subject3 {
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
    #[serde(rename = "NrKlienta")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_number: Option<String>,
}

pub use crate::shared::models::{
    Address, ContactData, IdentificationData, TStatusInfoPodatnika, TTypKorekty,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceBody {
    #[serde(rename = "KodWaluty")]
    pub currency_code: String,

    #[serde(rename = "P_1")]
    pub issue_date: String,
    #[serde(rename = "P_1M")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_issue: Option<String>,
    #[serde(rename = "P_2")]
    pub invoice_number: String,

    #[serde(rename = "WZ")]
    #[serde(default)]
    pub wz_docs: Vec<String>,

    #[serde(rename = "P_6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_date: Option<String>,
    #[serde(rename = "OkresFa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<Period>,

    #[serde(rename = "P_13_1")]
    #[serde(default)]
    pub net_total_basic_rate: Decimal,
    #[serde(rename = "P_14_1")]
    #[serde(default)]
    pub tax_total_basic_rate: Decimal,
    #[serde(rename = "P_14_1W")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_basic_rate_foreign: Option<Decimal>,

    #[serde(rename = "P_13_2")]
    #[serde(default)]
    pub net_total_reduced_rate_1: Decimal,
    #[serde(rename = "P_14_2")]
    #[serde(default)]
    pub tax_total_reduced_rate_1: Decimal,
    #[serde(rename = "P_14_2W")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_reduced_rate_1_foreign: Option<Decimal>,

    #[serde(rename = "P_13_3")]
    #[serde(default)]
    pub net_total_reduced_rate_2: Decimal,
    #[serde(rename = "P_14_3")]
    #[serde(default)]
    pub tax_total_reduced_rate_2: Decimal,
    #[serde(rename = "P_14_3W")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_reduced_rate_2_foreign: Option<Decimal>,

    #[serde(rename = "P_13_4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_taxi: Option<Decimal>,
    #[serde(rename = "P_14_4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_taxi: Option<Decimal>,

    #[serde(rename = "P_13_5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_total_special: Option<Decimal>,
    #[serde(rename = "P_14_5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_total_special: Option<Decimal>,

    #[serde(rename = "P_15")]
    pub total_gross: Decimal,

    #[serde(rename = "KursWalutyZ")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exchange_rate: Option<Decimal>,

    #[serde(rename = "Adnotacje")]
    pub annotations: Annotations,

    #[serde(rename = "RodzajFaktury")]
    pub invoice_type: String, // maps to TRodzajFaktury (e.g. "VAT")

    #[serde(rename = "PrzyczynaKorekty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_reason: Option<String>,
    #[serde(rename = "TypKorekty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correction_type: Option<TTypKorekty>,

    #[serde(rename = "FaWiersz")]
    #[serde(default)]
    pub lines: Vec<InvoiceLine>,

    #[serde(rename = "Rozliczenie")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement: Option<Settlement>,

    #[serde(rename = "Platnosc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,

    #[serde(rename = "RachunekBankowy")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub bank_accounts: Vec<BankAccount>,
}

impl Default for InvoiceBody {
    fn default() -> Self {
        Self {
            currency_code: "PLN".to_string(),
            issue_date: chrono::Local::now().format("%Y-%m-%d").to_string(),
            place_of_issue: Some("Polska".to_string()),
            invoice_number: String::new(),
            wz_docs: Vec::new(),
            sale_date: None,
            billing_period: None,
            net_total_basic_rate: Decimal::ZERO,
            tax_total_basic_rate: Decimal::ZERO,
            tax_total_basic_rate_foreign: None,
            net_total_reduced_rate_1: Decimal::ZERO,
            tax_total_reduced_rate_1: Decimal::ZERO,
            tax_total_reduced_rate_1_foreign: None,
            net_total_reduced_rate_2: Decimal::ZERO,
            tax_total_reduced_rate_2: Decimal::ZERO,
            tax_total_reduced_rate_2_foreign: None,
            net_total_taxi: None,
            tax_total_taxi: None,
            net_total_special: None,
            tax_total_special: None,
            total_gross: Decimal::ZERO,
            tax_exchange_rate: None,
            annotations: Annotations::default(),
            invoice_type: "VAT".to_string(),
            correction_reason: None,
            correction_type: None,
            lines: Vec::new(),
            settlement: None,
            payment: None,
            bank_accounts: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Period {
    #[serde(rename = "P_6_Od")]
    pub from: String,
    #[serde(rename = "P_6_Do")]
    pub to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NewTransportItem {
    #[serde(rename = "P_22A")]
    pub admission_date: String,
    #[serde(rename = "P_NrWierszaNST")]
    pub line_number: u8,
    #[serde(rename = "P_22BMK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(rename = "P_22BMD")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(rename = "P_22BK")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(rename = "P_22BNR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    #[serde(rename = "P_22BRP")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_year: Option<String>,
    #[serde(rename = "P_22B")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mileage: Option<String>,
    #[serde(rename = "P_22B1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vin: Option<String>,
    #[serde(rename = "P_22C")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_sailed: Option<u8>,
    #[serde(rename = "P_22D")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hours_flown: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct NewTransport {
    #[serde(rename = "P_22")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_transport: Option<u8>,
    #[serde(rename = "P_42_5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_transport_intra_community: Option<u8>,
    #[serde(rename = "NowySrodekTransportu")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    #[serde(default)]
    pub transport_items: Vec<NewTransportItem>,
    #[serde(rename = "P_22N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_new_transport_none: Option<u8>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct MarginProceduresOutput {
    #[serde(rename = "P_PMarzy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_procedure_flag: Option<u8>,
    #[serde(rename = "P_PMarzy_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_goods: Option<u8>,
    #[serde(rename = "P_PMarzy_3_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_of_art: Option<u8>,
    #[serde(rename = "P_PMarzy_3_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_items: Option<u8>,
    #[serde(rename = "P_PMarzy_3_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antiques: Option<u8>,
    #[serde(rename = "P_PMarzyN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_none: Option<u8>,
}

impl From<MarginProcedures> for MarginProceduresOutput {
    fn from(val: MarginProcedures) -> Self {
        let has_details = val.used_goods.is_some()
            || val.works_of_art.is_some()
            || val.collector_items.is_some()
            || val.antiques.is_some();
        let margin_procedure_flag = if has_details { Some(1) } else { None };

        MarginProceduresOutput {
            margin_procedure_flag,
            used_goods: val.used_goods,
            works_of_art: val.works_of_art,
            collector_items: val.collector_items,
            antiques: val.antiques,
            flag_none: val.flag_none,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(into = "MarginProceduresOutput")]
pub struct MarginProcedures {
    #[serde(rename = "P_PMarzy_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_goods: Option<u8>,
    #[serde(rename = "P_PMarzy_3_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub works_of_art: Option<u8>,
    #[serde(rename = "P_PMarzy_3_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector_items: Option<u8>,
    #[serde(rename = "P_PMarzy_3_3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antiques: Option<u8>,
    #[serde(rename = "P_PMarzyN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_none: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Settlement {
    #[serde(rename = "Obciazenia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Vec<Charge>>,
    #[serde(rename = "SumaObciazen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_charges: Option<Decimal>,
    #[serde(rename = "Odliczenia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deductions: Option<Vec<Charge>>,
    #[serde(rename = "SumaOdliczen")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_deductions: Option<Decimal>,
    #[serde(rename = "DoZaplaty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_pay: Option<Decimal>,
    #[serde(rename = "DoRozliczenia")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_reconcile: Option<Decimal>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Charge {
    #[serde(rename = "Kwota")]
    pub amount: Decimal,
    #[serde(rename = "Powod")]
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Payment {
    #[serde(rename = "Zaplacono")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_flag: Option<u8>,
    #[serde(rename = "DataZaplaty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<String>,
    #[serde(rename = "ZnacznikZaplatyCzesciowej")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payment_flag: Option<u8>,
    #[serde(rename = "ZaplataCzesciowa")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_payments: Option<Vec<PartialPayment>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PartialPayment {
    #[serde(rename = "KwotaZaplatyCzesciowej")]
    pub amount: Decimal,
    #[serde(rename = "DataZaplatyCzesciowej")]
    pub date: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BankAccount {
    #[serde(rename = "NrRB")]
    pub account_number: String,
    #[serde(rename = "SWIFT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<String>,
    #[serde(rename = "RachunekWlasnyBanku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_bank_account_flag: Option<u8>,
    #[serde(rename = "NazwaBanku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "OpisRachunku")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TaxExemption {
    #[serde(rename = "P_19")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_delivery: Option<u8>,
    #[serde(rename = "P_19N")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exempt_delivery_none: Option<u8>,
    #[serde(rename = "P_19A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_text: Option<String>,
    #[serde(rename = "P_19C")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basis_directive: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    pub exemption: TaxExemption,
    #[serde(rename = "NoweSrodkiTransportu")]
    pub new_transport: NewTransport,
    #[serde(rename = "P_23")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_procedure_travel: Option<u8>,
    #[serde(rename = "PMarzy")]
    pub margin_procedures: MarginProcedures,
}

impl Default for Annotations {
    fn default() -> Self {
        Self {
            cash_accounting: 2,
            self_billing: 2,
            reverse_charge: 2,
            split_payment: 2,
            exemption: TaxExemption {
                exempt_delivery: None,
                exempt_delivery_none: Some(1),
                basis_text: None,
                basis_directive: None,
            },
            new_transport: NewTransport {
                is_new_transport: None,
                is_new_transport_intra_community: None,
                transport_items: Vec::new(),
                is_new_transport_none: Some(1),
            },
            margin_procedure_travel: Some(2),
            margin_procedures: MarginProcedures {
                used_goods: None,
                works_of_art: None,
                collector_items: None,
                antiques: None,
                flag_none: Some(1),
            },
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct InvoiceLine {
    #[serde(rename = "NrWierszaFa")]
    pub line_number: u32,

    #[serde(rename = "UU_ID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uu_id: Option<String>,

    #[serde(rename = "P_6A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_date: Option<String>,

    #[serde(rename = "P_7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(rename = "Indeks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<String>,

    #[serde(rename = "GTIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtin: Option<String>, // Global Trade Item Number

    #[serde(rename = "PKWiU")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkwiu_code: Option<String>, // Polish Classification of Goods and Services (PKWiU)

    #[serde(rename = "CN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_code: Option<String>, // Combined Nomenclature (CN)

    #[serde(rename = "PKOB")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pkob_code: Option<String>, // Polish Classification of Types of Constructions (PKOB)

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
    pub discount_amount: Option<Decimal>,

    #[serde(rename = "P_11")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_value: Option<Decimal>,

    #[serde(rename = "P_11A")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_value: Option<Decimal>,

    #[serde(rename = "P_11Vat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_value: Option<Decimal>,

    #[serde(rename = "P_12")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<String>,

    #[serde(rename = "P_12_XII")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_procedure_xii: Option<Decimal>,

    #[serde(rename = "P_12_Zal_15")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate_appendix_15: Option<u8>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Footer {
    #[serde(rename = "Informacje")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub information: Option<Vec<FooterInfo>>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct FooterInfo {
    #[serde(rename = "StopkaFaktury")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer_text: Option<String>,
}
