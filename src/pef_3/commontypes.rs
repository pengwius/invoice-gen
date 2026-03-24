use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PSPRole {
    #[serde(rename = "PSPRoleType")]
    pub psp_role_type: PSPRoleType,
    #[serde(rename = "PSPRoleOther", skip_serializing_if = "Option::is_none")]
    pub psp_role_other: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PSPIdType {
    #[serde(rename = "BIC")]
    BIC,
    #[serde(rename = "Other")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PSPRoleType {
    #[serde(rename = "Three party card scheme")]
    ThreePartyCardScheme,
    #[serde(rename = "Four party card scheme")]
    FourPartyCardScheme,
    #[serde(rename = "E-money provider")]
    EMoneyProvider,
    #[serde(rename = "Acquirer")]
    Acquirer,
    #[serde(rename = "e-Wallet provider")]
    EWalletProvider,
    #[serde(rename = "Money Transfer operator")]
    MoneyTransferOperator,
    #[serde(rename = "Issuer of payment instruments")]
    IssuerOfPaymentInstruments,
    #[serde(rename = "Payment Processor")]
    PaymentProcessor,
    #[serde(rename = "E-payment")]
    EPayment,
    #[serde(rename = "Payment collector")]
    PaymentCollector,
    #[serde(rename = "Other")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct VATId {
    #[serde(rename = "@issuedBy")]
    pub issued_by: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TAXId {
    #[serde(rename = "@issuedBy")]
    pub issued_by: String,
    #[serde(rename = "@type")]
    pub tax_type: TAXIdType,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TAXIdType {
    #[serde(rename = "UNCONFIRMED_VAT")]
    UnconfirmedVat,
    #[serde(rename = "TIN")]
    TIN,
    #[serde(rename = "IOSS")]
    IOSS,
    #[serde(rename = "OTHER")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Amount {
    #[serde(rename = "@currency")]
    pub currency: String,
    #[serde(rename = "$value")]
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Name {
    #[serde(rename = "@nameType")]
    pub name_type: NameType,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum AccountIdentifierType {
    #[serde(rename = "IBAN")]
    IBAN,
    #[serde(rename = "OBAN")]
    OBAN,
    #[serde(rename = "Other")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum NameType {
    #[serde(rename = "BUSINESS")]
    Business,
    #[serde(rename = "TRADE")]
    Trade,
    #[serde(rename = "LEGAL")]
    Legal,
    #[serde(rename = "PERSON")]
    Person,
    #[serde(rename = "OTHER")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct PaymentMethod {
    #[serde(rename = "PaymentMethodType")]
    pub payment_method_type: PaymentMethodType,
    #[serde(rename = "PaymentMethodOther", skip_serializing_if = "Option::is_none")]
    pub payment_method_other: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum PaymentMethodType {
    #[serde(rename = "Card payment")]
    CardPayment,
    #[serde(rename = "Bank transfer")]
    BankTransfer,
    #[serde(rename = "Direct debit")]
    DirectDebit,
    #[serde(rename = "E-money")]
    EMoney,
    #[serde(rename = "Money Remittance")]
    MoneyRemittance,
    #[serde(rename = "Marketplace")]
    Marketplace,
    #[serde(rename = "Intermediary")]
    Intermediary,
    #[serde(rename = "Other")]
    Other,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct TransactionDate {
    #[serde(rename = "@transactionDateType")]
    pub transaction_date_type: TransactionDateType,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TransactionDateType {
    #[serde(rename = "CESOP701")]
    CESOP701,
    #[serde(rename = "CESOP702")]
    CESOP702,
    #[serde(rename = "CESOP703")]
    CESOP703,
    #[serde(rename = "CESOP704")]
    CESOP704,
    #[serde(rename = "CESOP709")]
    CESOP709,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct AddressFix {
    #[serde(rename = "Street", skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(rename = "BuildingIdentifier", skip_serializing_if = "Option::is_none")]
    pub building_identifier: Option<String>,
    #[serde(rename = "SuiteIdentifier", skip_serializing_if = "Option::is_none")]
    pub suite_identifier: Option<String>,
    #[serde(rename = "FloorIdentifier", skip_serializing_if = "Option::is_none")]
    pub floor_identifier: Option<String>,
    #[serde(rename = "DistrictName", skip_serializing_if = "Option::is_none")]
    pub district_name: Option<String>,
    #[serde(rename = "POB", skip_serializing_if = "Option::is_none")]
    pub pob: Option<String>,
    #[serde(rename = "PostCode", skip_serializing_if = "Option::is_none")]
    pub post_code: Option<String>,
    #[serde(rename = "City", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "CountrySubentity", skip_serializing_if = "Option::is_none")]
    pub country_subentity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Address {
    #[serde(rename = "@legalAddressType", skip_serializing_if = "Option::is_none")]
    pub legal_address_type: Option<LegalAddressType>,
    #[serde(rename = "CountryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "AddressFix", skip_serializing_if = "Option::is_none")]
    pub address_fix: Option<AddressFix>,
    #[serde(rename = "AddressFree", skip_serializing_if = "Option::is_none")]
    pub address_free: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum LegalAddressType {
    #[serde(rename = "CESOP301")]
    CESOP301,
    #[serde(rename = "CESOP302")]
    CESOP302,
    #[serde(rename = "CESOP303")]
    CESOP303,
    #[serde(rename = "CESOP304")]
    CESOP304,
    #[serde(rename = "CESOP309")]
    CESOP309,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct DocSpec {
    #[serde(rename = "DocTypeIndic")]
    pub doc_type_indic: DocTypeIndic,
    #[serde(rename = "DocRefId")]
    pub doc_ref_id: String,
    #[serde(rename = "CorrMessageRefId", skip_serializing_if = "Option::is_none")]
    pub corr_message_ref_id: Option<String>,
    #[serde(rename = "CorrDocRefId", skip_serializing_if = "Option::is_none")]
    pub corr_doc_ref_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum DocTypeIndic {
    #[serde(rename = "CESOP1")]
    CESOP1,
    #[serde(rename = "CESOP2")]
    CESOP2,
    #[serde(rename = "CESOP3")]
    CESOP3,
}
