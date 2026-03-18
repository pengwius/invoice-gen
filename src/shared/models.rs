use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaxRate {
    // Common numeric rates
    Rate23,
    Rate22,
    Rate8,
    Rate7,
    Rate5,
    Rate4,
    Rate3,
    // Special zero and non-numeric codes from XSD
    ZeroKR,
    ZeroWDT,
    ZeroEX,
    Zw,
    Oo,
    NpI,
    NpII,

    Custom(i32),
}

impl TaxRate {
    pub fn basis_points(&self) -> i32 {
        match *self {
            TaxRate::Rate23 => 2300,
            TaxRate::Rate22 => 2200,
            TaxRate::Rate8 => 800,
            TaxRate::Rate7 => 700,
            TaxRate::Rate5 => 500,
            TaxRate::Rate4 => 400,
            TaxRate::Rate3 => 300,
            TaxRate::ZeroKR
            | TaxRate::ZeroWDT
            | TaxRate::ZeroEX
            | TaxRate::Zw
            | TaxRate::Oo
            | TaxRate::NpI
            | TaxRate::NpII => 0,
            TaxRate::Custom(n) => n,
        }
    }

    pub fn from_str(s: &str) -> Result<TaxRate, ()> {
        let up = s.trim().to_uppercase();
        match up.as_str() {
            "23" => Ok(TaxRate::Rate23),
            "22" => Ok(TaxRate::Rate22),
            "8" => Ok(TaxRate::Rate8),
            "7" => Ok(TaxRate::Rate7),
            "5" => Ok(TaxRate::Rate5),
            "4" => Ok(TaxRate::Rate4),
            "3" => Ok(TaxRate::Rate3),
            "0 KR" | "0KR" => Ok(TaxRate::ZeroKR),
            "0 WDT" | "0WDT" => Ok(TaxRate::ZeroWDT),
            "0 EX" | "0EX" => Ok(TaxRate::ZeroEX),
            "ZW" => Ok(TaxRate::Zw),
            "OO" => Ok(TaxRate::Oo),
            "NP I" | "NPI" | "NP_I" => Ok(TaxRate::NpI),
            "NP II" | "NPII" | "NP_II" => Ok(TaxRate::NpII),
            other => {
                if let Ok(d) = Decimal::from_str(other) {
                    let bp_decimal = (d * Decimal::new(100, 0)).round_dp(0);
                    if let Some(bp_i32) = bp_decimal.to_i32() {
                        return Ok(match bp_i32 {
                            2300 => TaxRate::Rate23,
                            2200 => TaxRate::Rate22,
                            800 => TaxRate::Rate8,
                            700 => TaxRate::Rate7,
                            500 => TaxRate::Rate5,
                            400 => TaxRate::Rate4,
                            300 => TaxRate::Rate3,
                            0 => TaxRate::Zw,
                            other => TaxRate::Custom(other),
                        });
                    }
                }
                Err(())
            }
        }
    }
}

impl fmt::Display for TaxRate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaxRate::Rate23 => write!(f, "23"),
            TaxRate::Rate22 => write!(f, "22"),
            TaxRate::Rate8 => write!(f, "8"),
            TaxRate::Rate7 => write!(f, "7"),
            TaxRate::Rate5 => write!(f, "5"),
            TaxRate::Rate4 => write!(f, "4"),
            TaxRate::Rate3 => write!(f, "3"),
            TaxRate::ZeroKR => write!(f, "0 KR"),
            TaxRate::ZeroWDT => write!(f, "0 WDT"),
            TaxRate::ZeroEX => write!(f, "0 EX"),
            TaxRate::Zw => write!(f, "zw"),
            TaxRate::Oo => write!(f, "oo"),
            TaxRate::NpI => write!(f, "np I"),
            TaxRate::NpII => write!(f, "np II"),
            TaxRate::Custom(bp) => {
                let val = Decimal::new(*bp as i64, 2);
                let s = val.normalize().to_string();
                write!(f, "{}", s)
            }
        }
    }
}

impl serde::Serialize for TaxRate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for TaxRate {
    fn deserialize<D>(deserializer: D) -> Result<TaxRate, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        TaxRate::from_str(&s)
            .map_err(|_| serde::de::Error::custom(format!("invalid tax rate: {}", s)))
    }
}

#[derive(Debug, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct IdentificationData {
    #[serde(skip_serializing)]
    pub nip: String,
    #[serde(rename = "Nazwa")]
    pub name: String,
}

impl serde::Serialize for IdentificationData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("DaneIdentyfikacyjne", 2)?;

        let digits_only: String = self.nip.chars().filter(|c| c.is_ascii_digit()).collect();

        if !digits_only.is_empty() {
            state.serialize_field("NIP", &digits_only)?;
        } else {
            let fallback = self.nip.trim();
            if !fallback.is_empty() {
                state.serialize_field("NIP", &fallback)?;
            }
        }

        state.serialize_field("Nazwa", &self.name)?;
        state.end()
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct ContactData {
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "Telefon")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TRolaPodmiotu3 {
    #[serde(rename = "1")]
    Faktor,
    #[serde(rename = "2")]
    Odbiorca,
    #[serde(rename = "3")]
    PodmiotPierwotny,
    #[serde(rename = "4")]
    DodatkowyNabywca,
    #[serde(rename = "5")]
    WystawcaFaktury,
    #[serde(rename = "6")]
    DokonujacyPlatnosci,
    #[serde(rename = "7")]
    JednostkaSamorzaduWystawca,
    #[serde(rename = "8")]
    JednostkaSamorzaduOdbiorca,
    #[serde(rename = "9")]
    CzlonkekGrupyVATWystawca,
    #[serde(rename = "10")]
    CzlonkekGrupyVATOdbiorca,
    #[serde(rename = "11")]
    Pracownik,
}

impl Default for TRolaPodmiotu3 {
    fn default() -> Self {
        TRolaPodmiotu3::Faktor
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TRolaPodmiotuUpowaznionego {
    #[serde(rename = "1")]
    OrganEgzekucyjny,
    #[serde(rename = "2")]
    KomornikSadowy,
    #[serde(rename = "3")]
    PrzedstawicielPodatkowy,
}

impl Default for TRolaPodmiotuUpowaznionego {
    fn default() -> Self {
        TRolaPodmiotuUpowaznionego::PrzedstawicielPodatkowy
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TStatusInfoPodatnika {
    #[serde(rename = "1")]
    WStanieLikwidacji,
    #[serde(rename = "2")]
    Restrukturyzacja,
    #[serde(rename = "3")]
    WStanieUpadlosci,
    #[serde(rename = "4")]
    PrzedsiebiorstwoWSpadku,
}

impl Default for TStatusInfoPodatnika {
    fn default() -> Self {
        TStatusInfoPodatnika::WStanieLikwidacji
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TTypKorekty {
    #[serde(rename = "1")]
    SkutekUjeciaFakturyPierwotnej,
    #[serde(rename = "2")]
    SkutekDatyWystawienia,
    #[serde(rename = "3")]
    SkutekDatyInnej,
}

impl Default for TTypKorekty {
    fn default() -> Self {
        TTypKorekty::SkutekUjeciaFakturyPierwotnej
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct CountryCode(pub String);

impl CountryCode {
    pub fn new<S: Into<String>>(s: S) -> Self {
        let up = s.into().trim().to_uppercase();
        CountryCode(up)
    }

    pub fn is_valid(s: &str) -> bool {
        let up = s.trim().to_uppercase();
        up.len() == 2 && up.chars().all(|c| c.is_ascii_uppercase())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for CountryCode {
    type Err = ();

    fn from_str(s: &str) -> Result<CountryCode, ()> {
        let up = s.trim().to_uppercase();
        if up.len() == 2 && up.chars().all(|c| c.is_ascii_uppercase()) {
            Ok(CountryCode(up))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for CountryCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
pub struct CurrencyCode(pub String);

impl CurrencyCode {
    pub fn new<S: Into<String>>(s: S) -> Self {
        let up = s.into().trim().to_uppercase();
        CurrencyCode(up)
    }

    pub const ALL_CURRENCY_CODES: &'static [&'static str] = &[
        "AED", "AFN", "ALL", "AMD", "ANG", "AOA", "ARS", "AUD", "AWG", "AZN", "BAM", "BBD", "BDT",
        "BGN", "BHD", "BIF", "BMD", "BND", "BOB", "BOV", "BRL", "BSD", "BTN", "BWP", "BYN", "BZD",
        "CAD", "CDF", "CHE", "CHF", "CHW", "CLF", "CLP", "CNY", "COP", "COU", "CRC", "CUC", "CUP",
        "CVE", "CZK", "DJF", "DKK", "DOP", "DZD", "EGP", "ERN", "ETB", "EUR", "FJD", "FKP", "GBP",
        "GEL", "GHS", "GIP", "GMD", "GNF", "GTQ", "GYD", "HNL", "HRK", "HTG", "HUF", "IDR", "ILS",
        "INR", "IQD", "IRR", "ISK", "JMD", "JOD", "JPY", "KES", "KGS", "KHR", "KMF", "KPW", "KRW",
        "KWD", "KYD", "KZT", "LAK", "LBP", "LKR", "LRD", "LSL", "LYD", "MAD", "MDL", "MGA", "MKD",
        "MMK", "MNT", "MOP", "MRU", "MUR", "MVR", "MWK", "MXN", "MXV", "MYR", "MZN", "NAD", "NGN",
        "NIO", "NOK", "NPR", "NZD", "OMR", "PAB", "PEN", "PGK", "PHP", "PKR", "PLN", "PYG", "QAR",
        "RON", "RSD", "RUB", "RWF", "SAR", "SBD", "SCR", "SDG", "SEK", "SGD", "SHP", "SLL", "SOS",
        "SRD", "SSP", "STN", "SVC", "SYP", "SZL", "THB", "TJS", "TMT", "TND", "TOP", "TRY", "TTD",
        "TWD", "TZS", "UAH", "UGX", "USD", "USN", "UYI", "UYU", "UYW", "UZS", "VES", "VND", "VUV",
        "WST", "XAF", "XAG", "XAU", "XBA", "XBB", "XBC", "XBD", "XCD", "XCG", "XDR", "XOF", "XPD",
        "XPF", "XPT", "XSU", "XUA", "XXX", "YER", "ZAR", "ZMW", "ZWL",
    ];

    pub fn is_valid(s: &str) -> bool {
        let up = s.trim().to_uppercase();
        if up.len() != 3 || !up.chars().all(|c| c.is_ascii_uppercase()) {
            return false;
        }
        Self::ALL_CURRENCY_CODES.contains(&up.as_str())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn allowed_codes() -> &'static [&'static str] {
        Self::ALL_CURRENCY_CODES
    }
}

impl FromStr for CurrencyCode {
    type Err = ();

    fn from_str(s: &str) -> Result<CurrencyCode, ()> {
        let up = s.trim().to_uppercase();
        if CurrencyCode::is_valid(&up) {
            Ok(CurrencyCode(up))
        } else {
            Err(())
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn is_valid_swift(s: &str) -> bool {
    let s = s.trim().to_uppercase();
    let len = s.len();
    if !(len == 8 || len == 11) {
        return false;
    }
    let bytes = s.as_bytes();
    if !bytes.iter().take(6).all(|b| (b'A'..=b'Z').contains(b)) {
        return false;
    }
    if !bytes
        .iter()
        .skip(6)
        .all(|b| (b'A'..=b'Z').contains(b) || (b'0'..=b'9').contains(b))
    {
        return false;
    }
    true
}

pub fn is_valid_gln(s: &str) -> bool {
    let s = s.trim();
    let len = s.len();
    if len == 0 || len > 13 {
        return false;
    }
    s.chars().all(|c| c.is_ascii_digit())
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum TFormaPlatnosci {
    #[serde(rename = "1")]
    Gotowka,
    #[serde(rename = "2")]
    Karta,
    #[serde(rename = "3")]
    Bon,
    #[serde(rename = "4")]
    Czek,
    #[serde(rename = "5")]
    Kredyt,
    #[serde(rename = "6")]
    Przelew,
    #[serde(rename = "7")]
    Mobilna,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    #[serde(rename = "KodKraju")]
    pub country_code: CountryCode,
    #[serde(rename = "AdresL1")]
    pub address_line_1: String,
    #[serde(rename = "AdresL2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    #[serde(rename = "GLN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gln: Option<String>,
}
