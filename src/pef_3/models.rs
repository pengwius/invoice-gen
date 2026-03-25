use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename = "Invoice")]
pub struct Invoice {
    #[serde(rename = "@xmlns")]
    pub xmlns: Option<String>,
    #[serde(rename = "@xmlns:cac")]
    pub xmlns_cac: Option<String>,
    #[serde(rename = "@xmlns:cbc")]
    pub xmlns_cbc: Option<String>,
    #[serde(rename = "@xmlns:ext")]
    pub xmlns_ext: Option<String>,

    #[serde(rename = "ext:UBLExtensions", alias = "UBLExtensions", skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UblExtensions>,

    #[serde(rename = "cbc:UBLVersionID", alias = "UBLVersionID", skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<String>,

    #[serde(
        rename = "cbc:CustomizationID", alias = "CustomizationID",
        skip_serializing_if = "Option::is_none"
    )]
    pub customization_id: Option<String>,

    #[serde(rename = "cbc:ProfileID", alias = "ProfileID", skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<String>,

    #[serde(
        rename = "cbc:ProfileExecutionID", alias = "ProfileExecutionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub profile_execution_id: Option<String>,

    #[serde(rename = "cbc:ID", alias = "ID")]
    pub id: Option<Identifier>,

    #[serde(rename = "cbc:CopyIndicator", alias = "CopyIndicator", skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<bool>,

    #[serde(rename = "cbc:UUID", alias = "UUID", skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    #[serde(rename = "cbc:IssueDate", alias = "IssueDate")]
    pub issue_date: Option<String>,

    #[serde(rename = "cbc:IssueTime", alias = "IssueTime", skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<String>,

    #[serde(rename = "cbc:DueDate", alias = "DueDate", skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,

    #[serde(
        rename = "cbc:InvoiceTypeCode", alias = "InvoiceTypeCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub invoice_type_code: Option<Code>,

    #[serde(rename = "cbc:Note", alias = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<TextType>>,

    #[serde(rename = "cbc:TaxPointDate", alias = "TaxPointDate", skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<String>,

    #[serde(
        rename = "cbc:DocumentCurrencyCode", alias = "DocumentCurrencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub document_currency_code: Option<Code>,

    #[serde(
        rename = "cbc:TaxCurrencyCode", alias = "TaxCurrencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_currency_code: Option<Code>,

    #[serde(
        rename = "cbc:PricingCurrencyCode", alias = "PricingCurrencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub pricing_currency_code: Option<Code>,

    #[serde(
        rename = "cbc:PaymentCurrencyCode", alias = "PaymentCurrencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_currency_code: Option<Code>,

    #[serde(
        rename = "cbc:PaymentAlternativeCurrencyCode", alias = "PaymentAlternativeCurrencyCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_alternative_currency_code: Option<Code>,

    #[serde(
        rename = "cbc:AccountingCostCode", alias = "AccountingCostCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub accounting_cost_code: Option<String>,

    #[serde(rename = "cbc:AccountingCost", alias = "AccountingCost", skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<String>,

    #[serde(
        rename = "cbc:LineCountNumeric", alias = "LineCountNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_count_numeric: Option<i32>,

    #[serde(rename = "cbc:BuyerReference", alias = "BuyerReference", skip_serializing_if = "Option::is_none")]
    pub buyer_reference: Option<String>,

    #[serde(rename = "cac:InvoicePeriod", alias = "InvoicePeriod", skip_serializing_if = "Option::is_none")]
    pub invoice_period: Option<Vec<Period>>,

    #[serde(rename = "cac:OrderReference", alias = "OrderReference", skip_serializing_if = "Option::is_none")]
    pub order_reference: Option<OrderReference>,

    #[serde(
        rename = "cac:BillingReference", alias = "BillingReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub billing_reference: Option<Vec<BillingReference>>,

    #[serde(
        rename = "cac:DespatchDocumentReference", alias = "DespatchDocumentReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub despatch_document_reference: Option<Vec<DocumentReference>>,

    #[serde(
        rename = "cac:ReceiptDocumentReference", alias = "ReceiptDocumentReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub receipt_document_reference: Option<Vec<DocumentReference>>,

    #[serde(
        rename = "cac:StatementDocumentReference", alias = "StatementDocumentReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub statement_document_reference: Option<Vec<DocumentReference>>,

    #[serde(
        rename = "cac:OriginatorDocumentReference", alias = "OriginatorDocumentReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub originator_document_reference: Option<Vec<DocumentReference>>,

    #[serde(
        rename = "cac:ContractDocumentReference", alias = "ContractDocumentReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub contract_document_reference: Option<Vec<DocumentReference>>,

    #[serde(
        rename = "cac:AdditionalDocumentReference", alias = "AdditionalDocumentReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_document_reference: Option<Vec<DocumentReference>>,

    #[serde(
        rename = "cac:ProjectReference", alias = "ProjectReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub project_reference: Option<Vec<ProjectReference>>,

    #[serde(rename = "cac:Signature", alias = "Signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<Vec<Signature>>,

    #[serde(rename = "cac:AccountingSupplierParty", alias = "AccountingSupplierParty")]
    pub accounting_supplier_party: Option<PartyWrapper>,

    #[serde(rename = "cac:AccountingCustomerParty", alias = "AccountingCustomerParty")]
    pub accounting_customer_party: Option<PartyWrapper>,

    #[serde(rename = "cac:PayeeParty", alias = "PayeeParty", skip_serializing_if = "Option::is_none")]
    pub payee_party: Option<PartyType>,

    #[serde(
        rename = "cac:BuyerCustomerParty", alias = "BuyerCustomerParty",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyer_customer_party: Option<PartyType>,

    #[serde(
        rename = "cac:SellerSupplierParty", alias = "SellerSupplierParty",
        skip_serializing_if = "Option::is_none"
    )]
    pub seller_supplier_party: Option<PartyType>,

    #[serde(
        rename = "cac:TaxRepresentativeParty", alias = "TaxRepresentativeParty",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_representative_party: Option<PartyType>,

    #[serde(rename = "cac:Delivery", alias = "Delivery", skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Vec<Delivery>>,

    #[serde(rename = "cac:DeliveryTerms", alias = "DeliveryTerms", skip_serializing_if = "Option::is_none")]
    pub delivery_terms: Option<DeliveryTerms>,

    #[serde(rename = "cac:PaymentMeans", alias = "PaymentMeans", skip_serializing_if = "Option::is_none")]
    pub payment_means: Option<Vec<PaymentMeans>>,

    #[serde(rename = "cac:PaymentTerms", alias = "PaymentTerms", skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<Vec<PaymentTerms>>,

    #[serde(rename = "cac:PrepaidPayment", alias = "PrepaidPayment", skip_serializing_if = "Option::is_none")]
    pub prepaid_payment: Option<Vec<Payment>>,

    #[serde(
        rename = "cac:AllowanceCharge", alias = "AllowanceCharge",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowance_charge: Option<Vec<AllowanceCharge>>,

    #[serde(
        rename = "cac:TaxExchangeRate", alias = "TaxExchangeRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_exchange_rate: Option<ExchangeRate>,

    #[serde(
        rename = "cac:PricingExchangeRate", alias = "PricingExchangeRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub pricing_exchange_rate: Option<ExchangeRate>,

    #[serde(
        rename = "cac:PaymentExchangeRate", alias = "PaymentExchangeRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_exchange_rate: Option<ExchangeRate>,

    #[serde(
        rename = "cac:PaymentAlternativeExchangeRate", alias = "PaymentAlternativeExchangeRate",
        skip_serializing_if = "Option::is_none"
    )]
    pub payment_alternative_exchange_rate: Option<ExchangeRate>,

    #[serde(rename = "cac:TaxTotal", alias = "TaxTotal", skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<Vec<TaxTotal>>,

    #[serde(
        rename = "cac:WithholdingTaxTotal", alias = "WithholdingTaxTotal",
        skip_serializing_if = "Option::is_none"
    )]
    pub withholding_tax_total: Option<Vec<TaxTotal>>,

    #[serde(rename = "cac:LegalMonetaryTotal", alias = "LegalMonetaryTotal")]
    pub legal_monetary_total: Option<MonetaryTotal>,

    #[serde(rename = "cac:InvoiceLine", alias = "InvoiceLine")]
    pub invoice_line: Option<Vec<InvoiceLine>>,
}

impl Invoice {
    pub fn to_xml(&self) -> Result<String, Box<dyn std::error::Error>> {
        quick_xml::se::to_string(self).map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct UblExtensions {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Period {
    #[serde(rename = "cbc:StartDate", alias = "StartDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "cbc:StartTime", alias = "StartTime", skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(rename = "cbc:EndDate", alias = "EndDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "cbc:EndTime", alias = "EndTime", skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(
        rename = "cbc:DurationMeasure", alias = "DurationMeasure",
        skip_serializing_if = "Option::is_none"
    )]
    pub duration_measure: Option<String>,
    #[serde(
        rename = "cbc:DescriptionCode", alias = "DescriptionCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub description_code: Option<Vec<String>>,
    #[serde(rename = "cbc:Description", alias = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct OrderReference {
    #[serde(rename = "cbc:ID", alias = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<Identifier>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct BillingReference {
    #[serde(
        rename = "cac:InvoiceDocumentReference",
        alias = "InvoiceDocumentReference",
        skip_serializing_if = "Option::is_none"
    )]
    pub invoice_document_reference: Option<DocumentReference>,
}

impl From<DocumentReference> for BillingReference {
    fn from(doc_ref: DocumentReference) -> Self {
        BillingReference {
            invoice_document_reference: Some(doc_ref),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct DocumentReference {
    #[serde(rename = "cbc:ID", alias = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<Identifier>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ProjectReference {
    #[serde(rename = "cbc:ID", alias = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<Identifier>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Signature {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PartyIdentification {
    #[serde(rename = "cbc:ID", alias = "ID")]
    pub id: Identifier,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PartyType {
    #[serde(rename = "cbc:EndpointID", alias = "EndpointID", skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<Identifier>,
    #[serde(rename = "cac:PartyIdentification", alias = "PartyIdentification", skip_serializing_if = "Option::is_none")]
    pub party_identification: Option<Vec<PartyIdentification>>,
    #[serde(rename = "cac:PartyName", alias = "PartyName", skip_serializing_if = "Option::is_none")]
    pub party_name: Option<Vec<PartyName>>,
    #[serde(rename = "cac:PostalAddress", alias = "PostalAddress", skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<PostalAddress>,
    #[serde(rename = "cac:PartyTaxScheme", alias = "PartyTaxScheme", skip_serializing_if = "Option::is_none")]
    pub party_tax_scheme: Option<Vec<PartyTaxScheme>>,
    #[serde(
        rename = "cac:PartyLegalEntity", alias = "PartyLegalEntity",
        skip_serializing_if = "Option::is_none"
    )]
    pub party_legal_entity: Option<PartyLegalEntity>,
    #[serde(rename = "cac:Contact", alias = "Contact", skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PartyName {
    #[serde(rename = "cbc:Name", alias = "Name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PostalAddress {
    #[serde(rename = "cbc:ID", alias = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<Identifier>,
    #[serde(rename = "cbc:StreetName", alias = "StreetName", skip_serializing_if = "Option::is_none")]
    pub street_name: Option<String>,
    #[serde(
        rename = "cbc:AdditionalStreetName", alias = "AdditionalStreetName",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_street_name: Option<String>,
    #[serde(rename = "cbc:CityName", alias = "CityName", skip_serializing_if = "Option::is_none")]
    pub city_name: Option<String>,
    #[serde(rename = "cbc:PostalZone", alias = "PostalZone", skip_serializing_if = "Option::is_none")]
    pub postal_zone: Option<String>,
    #[serde(rename = "cac:Country", alias = "Country", skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Country {
    #[serde(
        rename = "cbc:IdentificationCode", alias = "IdentificationCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub identification_code: Option<Code>,
    #[serde(rename = "cbc:Name", alias = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Country {
    pub fn new(identification_code: impl Into<String>) -> Self {
        Self {
            identification_code: Some(Code::new(identification_code)),
            name: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PartyTaxScheme {
    #[serde(rename = "cbc:CompanyID", alias = "CompanyID", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Identifier>,
    #[serde(rename = "cac:TaxScheme", alias = "TaxScheme")]
    pub tax_scheme: TaxScheme,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TaxScheme {
    #[serde(rename = "cbc:ID", alias = "ID")]
    pub id: Option<Identifier>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PartyLegalEntity {
    #[serde(
        rename = "cbc:RegistrationName", alias = "RegistrationName",
        skip_serializing_if = "Option::is_none"
    )]
    pub registration_name: Option<String>,
    #[serde(rename = "cbc:CompanyID", alias = "CompanyID", skip_serializing_if = "Option::is_none")]
    pub company_id: Option<Identifier>,
    #[serde(
        rename = "cbc:CompanyLegalForm", alias = "CompanyLegalForm",
        skip_serializing_if = "Option::is_none"
    )]
    pub company_legal_form: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Contact {
    #[serde(rename = "cbc:Name", alias = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "cbc:Telephone", alias = "Telephone", skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    #[serde(rename = "cbc:ElectronicMail", alias = "ElectronicMail", skip_serializing_if = "Option::is_none")]
    pub electronic_mail: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Delivery {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct DeliveryTerms {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PaymentMeans {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PaymentTerms {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Payment {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct AllowanceCharge {
    #[serde(
        rename = "cbc:ChargeIndicator", alias = "ChargeIndicator",
        skip_serializing_if = "Option::is_none"
    )]
    pub charge_indicator: Option<bool>,
    #[serde(
        rename = "cbc:AllowanceChargeReasonCode", alias = "AllowanceChargeReasonCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowance_charge_reason_code: Option<String>,
    #[serde(
        rename = "cbc:AllowanceChargeReason", alias = "AllowanceChargeReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowance_charge_reason: Option<String>,
    #[serde(
        rename = "cbc:MultiplierFactorNumeric", alias = "MultiplierFactorNumeric",
        skip_serializing_if = "Option::is_none"
    )]
    pub multiplier_factor_numeric: Option<String>,
    #[serde(rename = "cbc:Amount", alias = "Amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Amount>,
    #[serde(rename = "cbc:BaseAmount", alias = "BaseAmount", skip_serializing_if = "Option::is_none")]
    pub base_amount: Option<Amount>,
    #[serde(rename = "cac:TaxCategory", alias = "TaxCategory", skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<TaxCategory>,
    #[serde(rename = "cac:TaxTotal", alias = "TaxTotal", skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TaxTotal>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ExchangeRate {}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TaxTotal {
    #[serde(rename = "cbc:TaxAmount", alias = "TaxAmount", skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Amount>,
    #[serde(
        rename = "cbc:TaxInclusiveAmount", alias = "TaxInclusiveAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_inclusive_amount: Option<Amount>,
    #[serde(rename = "cac:TaxSubtotal", alias = "TaxSubtotal", skip_serializing_if = "Option::is_none")]
    pub tax_subtotal: Option<Vec<TaxSubtotal>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TaxSubtotal {
    #[serde(
        rename = "cbc:TaxInclusiveAmount", alias = "TaxInclusiveAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_inclusive_amount: Option<Amount>,
    #[serde(rename = "cbc:TaxableAmount", alias = "TaxableAmount", skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<Amount>,
    #[serde(rename = "cbc:TaxAmount", alias = "TaxAmount", skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Amount>,
    #[serde(rename = "cac:TaxCategory", alias = "TaxCategory")]
    pub tax_category: TaxCategory,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TaxCategory {
    #[serde(rename = "cbc:ID", alias = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<Identifier>,
    #[serde(rename = "cbc:Percent", alias = "Percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<String>,
    #[serde(
        rename = "cbc:TaxExemptionReasonCode", alias = "TaxExemptionReasonCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_exemption_reason_code: Option<Code>,
    #[serde(
        rename = "cbc:TaxExemptionReason", alias = "TaxExemptionReason",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_exemption_reason: Option<String>,
    #[serde(rename = "cac:TaxScheme", alias = "TaxScheme")]
    pub tax_scheme: TaxScheme,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct MonetaryTotal {
    #[serde(
        rename = "cbc:LineExtensionAmount", alias = "LineExtensionAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_extension_amount: Option<Amount>,
    #[serde(
        rename = "cbc:TaxExclusiveAmount", alias = "TaxExclusiveAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_exclusive_amount: Option<Amount>,
    #[serde(
        rename = "cbc:TaxInclusiveAmount", alias = "TaxInclusiveAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_inclusive_amount: Option<Amount>,
    #[serde(
        rename = "cbc:AllowanceTotalAmount", alias = "AllowanceTotalAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowance_total_amount: Option<Amount>,
    #[serde(
        rename = "cbc:ChargeTotalAmount", alias = "ChargeTotalAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub charge_total_amount: Option<Amount>,
    #[serde(rename = "cbc:PrepaidAmount", alias = "PrepaidAmount", skip_serializing_if = "Option::is_none")]
    pub prepaid_amount: Option<Amount>,
    #[serde(
        rename = "cbc:PayableRoundingAmount", alias = "PayableRoundingAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub payable_rounding_amount: Option<Amount>,
    #[serde(rename = "cbc:PayableAmount", alias = "PayableAmount", skip_serializing_if = "Option::is_none")]
    pub payable_amount: Option<Amount>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Amount {
    #[serde(rename = "@currencyID")]
    pub currency_id: String,
    #[serde(
        rename = "@currencyCodeListVersionID",
        skip_serializing_if = "Option::is_none"
    )]
    pub currency_code_list_version_id: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

impl Amount {
    pub fn new(currency_id: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            currency_id: currency_id.into(),
            currency_code_list_version_id: None,
            value: value.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct InvoiceLine {
    #[serde(rename = "cbc:ID", alias = "ID")]
    pub id: Option<Identifier>,
    #[serde(rename = "cbc:Note", alias = "Note", skip_serializing_if = "Option::is_none")]
    pub note: Option<Vec<TextType>>,
    #[serde(
        rename = "cbc:InvoicedQuantity", alias = "InvoicedQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub invoiced_quantity: Option<Quantity>,
    #[serde(
        rename = "cbc:LineExtensionAmount", alias = "LineExtensionAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_extension_amount: Option<Amount>,
    #[serde(
        rename = "cbc:LineExtensionGrossAmount", alias = "LineExtensionGrossAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub line_extension_gross_amount: Option<Amount>,
    #[serde(rename = "cbc:AccountingCost", alias = "AccountingCost", skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<String>,
    #[serde(rename = "cac:InvoicePeriod", alias = "InvoicePeriod", skip_serializing_if = "Option::is_none")]
    pub invoice_period: Option<Period>,
    #[serde(
        rename = "cac:AllowanceCharge", alias = "AllowanceCharge",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowance_charge: Option<Vec<AllowanceCharge>>,
    #[serde(rename = "cac:TaxTotal", alias = "TaxTotal", skip_serializing_if = "Option::is_none")]
    pub tax_total: Option<TaxTotal>,
    #[serde(rename = "cac:Item", alias = "Item", skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(rename = "cac:Price", alias = "Price", skip_serializing_if = "Option::is_none")]
    pub price: Option<Price>,
    #[serde(rename = "cac:SubInvoiceLine", alias = "SubInvoiceLine", skip_serializing_if = "Option::is_none")]
    pub sub_invoice_line: Option<Vec<SubInvoiceLine>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Quantity {
    #[serde(rename = "@unitCode", skip_serializing_if = "Option::is_none")]
    pub unit_code: Option<String>,
    #[serde(rename = "@unitCodeListID", skip_serializing_if = "Option::is_none")]
    pub unit_code_list_id: Option<String>,
    #[serde(
        rename = "@unitCodeListAgencyID",
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_code_list_agency_id: Option<String>,
    #[serde(
        rename = "@unitCodeListAgencyName",
        skip_serializing_if = "Option::is_none"
    )]
    pub unit_code_list_agency_name: Option<String>,
    #[serde(rename = "$value")]
    pub value: String,
}

impl Quantity {
    pub fn new(value: impl Into<String>, unit_code: impl Into<String>) -> Self {
        Self {
            unit_code: Some(unit_code.into()),
            unit_code_list_id: None,
            unit_code_list_agency_id: None,
            unit_code_list_agency_name: None,
            value: value.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Item {
    #[serde(rename = "cbc:Description", alias = "Description", skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<String>>,
    #[serde(rename = "cbc:Name", alias = "Name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(
        rename = "cac:BuyersItemIdentification", alias = "BuyersItemIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub buyers_item_identification: Option<ItemIdentification>,
    #[serde(
        rename = "cac:SellersItemIdentification", alias = "SellersItemIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub sellers_item_identification: Option<ItemIdentification>,
    #[serde(
        rename = "cac:StandardItemIdentification", alias = "StandardItemIdentification",
        skip_serializing_if = "Option::is_none"
    )]
    pub standard_item_identification: Option<ItemIdentification>,
    #[serde(rename = "cac:OriginCountry", alias = "OriginCountry", skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<Country>,
    #[serde(
        rename = "cac:CommodityClassification", alias = "CommodityClassification",
        skip_serializing_if = "Option::is_none"
    )]
    pub commodity_classification: Option<Vec<CommodityClassification>>,
    #[serde(
        rename = "cac:ClassifiedTaxCategory", alias = "ClassifiedTaxCategory",
        skip_serializing_if = "Option::is_none"
    )]
    pub classified_tax_category: Option<Vec<ClassifiedTaxCategory>>,
    #[serde(
        rename = "cac:AdditionalItemProperty", alias = "AdditionalItemProperty",
        skip_serializing_if = "Option::is_none"
    )]
    pub additional_item_property: Option<Vec<AdditionalItemProperty>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Price {
    #[serde(rename = "cbc:PriceAmount", alias = "PriceAmount", skip_serializing_if = "Option::is_none")]
    pub price_amount: Option<Amount>,
    #[serde(
        rename = "cbc:GrossPriceAmount", alias = "GrossPriceAmount",
        skip_serializing_if = "Option::is_none"
    )]
    pub gross_price_amount: Option<Amount>,
    #[serde(rename = "cbc:BaseQuantity", alias = "BaseQuantity", skip_serializing_if = "Option::is_none")]
    pub base_quantity: Option<Quantity>,
    #[serde(
        rename = "cac:AllowanceCharge", alias = "AllowanceCharge",
        skip_serializing_if = "Option::is_none"
    )]
    pub allowance_charge: Option<Vec<AllowanceCharge>>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct SubInvoiceLine {
    #[serde(rename = "cbc:ID", alias = "ID")]
    pub id: Option<Identifier>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ItemIdentification {
    #[serde(rename = "cbc:ID", alias = "ID")]
    pub id: Option<Identifier>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct CommodityClassification {
    #[serde(
        rename = "cbc:ItemClassificationCode", alias = "ItemClassificationCode",
        skip_serializing_if = "Option::is_none"
    )]
    pub item_classification_code: Option<Code>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct ClassifiedTaxCategory {
    #[serde(rename = "cbc:ID", alias = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<Identifier>,
    #[serde(rename = "cbc:Percent", alias = "Percent", skip_serializing_if = "Option::is_none")]
    pub percent: Option<String>,
    #[serde(rename = "cac:TaxScheme", alias = "TaxScheme")]
    pub tax_scheme: TaxScheme,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct AdditionalItemProperty {
    #[serde(rename = "cbc:Name", alias = "Name")]
    pub name: String,
    #[serde(rename = "cbc:Value", alias = "Value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Identifier {
    #[serde(rename = "@schemeID", skip_serializing_if = "Option::is_none")]
    pub scheme_id: Option<String>,
    #[serde(rename = "@schemeAgencyID", skip_serializing_if = "Option::is_none")]
    pub scheme_agency_id: Option<String>,
    #[serde(rename = "$value", default)]
    pub value: String,
}

impl Identifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            scheme_id: None,
            scheme_agency_id: None,
            value: value.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct Code {
    #[serde(rename = "@listID", skip_serializing_if = "Option::is_none")]
    pub list_id: Option<String>,
    #[serde(rename = "@listAgencyID", skip_serializing_if = "Option::is_none")]
    pub list_agency_id: Option<String>,
    #[serde(rename = "$value", default)]
    pub value: String,
}

impl Code {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            list_id: None,
            list_agency_id: None,
            value: value.into(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct TextType {
    #[serde(rename = "@languageID", skip_serializing_if = "Option::is_none")]
    pub language_id: Option<String>,
    #[serde(rename = "$value", default)]
    pub value: String,
}

impl From<String> for Identifier {
    fn from(value: String) -> Self {
        Identifier {
            value,
            ..Default::default()
        }
    }
}

impl From<&str> for Identifier {
    fn from(value: &str) -> Self {
        Identifier {
            value: value.to_owned(),
            ..Default::default()
        }
    }
}

impl From<String> for Code {
    fn from(value: String) -> Self {
        Code {
            value,
            ..Default::default()
        }
    }
}

impl From<&str> for Code {
    fn from(value: &str) -> Self {
        Code {
            value: value.to_owned(),
            ..Default::default()
        }
    }
}

impl From<String> for TextType {
    fn from(value: String) -> Self {
        TextType {
            value,
            ..Default::default()
        }
    }
}

impl From<&str> for TextType {
    fn from(value: &str) -> Self {
        TextType {
            value: value.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct PartyWrapper {
    #[serde(rename = "cac:Party", alias = "Party")]
    pub party: PartyType,
}
