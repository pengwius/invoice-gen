[Polska wersja](README.md) / English version

# invoice-gen

## Installation

```rust
[dependencies]
invoice-gen = "0.0.5"
```

## Usage

### FA (3)

```rust
use invoice_gen::fa_3::builder::{SellerBuilder, BuyerBuilder, LineBuilder};
use invoice_gen::fa_3::models::Invoice;
use rust_decimal::Decimal;
use std::str::FromStr;

let seller = SellerBuilder::new("5261234567", "Firma Sprzedająca Sp. z o.o.")
    .set_address("PL", "ul. Przykładowa", "1", None, "Warszawa", "00-001")
    .build();

let buyer = BuyerBuilder::new("9876543210", "Klient S.A.")
    .set_address("PL", "ul. Kupiecka", "5", None, "Warszawa", "00-100")
    .build();

let line = LineBuilder::new(
    "Produkt",
    Decimal::new(10, 0),
    Decimal::from_str("25.00").unwrap(),
    invoice_gen::shared::models::TaxRate::from_str("23").unwrap(),
)
.build();

let mut invoice = Invoice::default();
invoice.subject1 = seller;
invoice.subject2 = buyer;
invoice.invoice_body.invoice_number = "FV/2026/0001".to_string();
invoice.invoice_body.issue_date = chrono::Local::now().format("%Y-%m-%d").to_string();
invoice.invoice_body.currency_code = invoice_gen::shared::models::CurrencyCode::new("PLN");
invoice.invoice_body.lines = vec![line];

let xml = invoice.to_xml().unwrap();
println!("{}", xml);
```

### FA (2)

```rust
use invoice_gen::fa_2::builder::{SellerBuilder, BuyerBuilder, LineBuilder, InvoiceBuilder};
use rust_decimal::Decimal;
use chrono::NaiveDate;

let seller = SellerBuilder::new("5261234567", "Firma").set_address("PL","U","1",None,"M","00-000").build();
let buyer  = BuyerBuilder::new("9876543210","Klient").set_address("PL","U","2",None,"M","00-000").build();
let l1 = LineBuilder::new("A","pcs",Decimal::from_str("1").unwrap(),Decimal::from_str("100").unwrap(),"23").build();

let inv = InvoiceBuilder::new()
    .set_invoice_number("FV/1")
    .set_issue_date(NaiveDate::from_ymd_opt(2023,1,1).unwrap())
    .set_currency("PLN")
    .set_seller(seller)
    .set_buyer(buyer)
    .add_lines(vec![l1])
    .build()
    .unwrap();

let xml = inv.to_xml().unwrap();
println!("{}", xml);
```

---

### FA_RR (VAT RR Invoice – Flat-rate Farmer)

Support for VAT RR invoices compliant with the Polish Ministry of Finance template (flat-rate farmer). Allows generating RR invoices in XML format according to the official XSD schema.

```rust
use invoice_gen::fa_rr::builder::{InvoiceRRBuilder, Subject1Builder, Subject2Builder, AddressBuilder, FooterBuilder};
use invoice_gen::fa_rr::models::Header;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use invoice_gen::shared::models::IdentificationData;

let subject1 = Subject1Builder::new(IdentificationData {
        nip: "5261234567".to_string(),
        name: "Jan Kowalski, Farmer".to_string(),
    })
    .set_address(
        AddressBuilder::new()
            .set_country_code("PL")
            .set_address_line_1("Wiejska 1")
            .set_line2("Farm 2")
            .build(),
    )
    .build();

let subject2 = Subject2Builder::new(IdentificationData {
        nip: "9876543210".to_string(),
        name: "Purchasing Company Ltd.".to_string(),
    })
    .set_address(
        AddressBuilder::new()
            .set_country_code("PL")
            .set_address_line_1("Industrial 5")
            .build(),
    )
    .build();

let footer = FooterBuilder::new()
    .set_footer_text("Thank you for your cooperation.")
    .build();

let invoice = InvoiceRRBuilder::new()
    .set_header(Header::default())
    .set_subject1(subject1)
    .set_subject2(subject2)
    .set_currency("PLN")
    .set_issue_date(NaiveDate::from_ymd_opt(2023, 10, 25).unwrap())
    .set_invoice_number("RR/2023/10/001")
    .set_net_value(Decimal::from_str("1000.00").unwrap())
    .set_tax_value(Decimal::from_str("65.00").unwrap())
    .set_total_value(Decimal::from_str("1065.00").unwrap())
    .set_total_value_pln(Decimal::from_str("1065.00").unwrap())
    .set_total_value_words("one thousand sixty-five zlotys 00/100")
    .set_invoice_type(invoice_gen::fa_rr::models::InvoiceType::VatRr)
    .set_footer(footer)
    .build();

let xml = invoice.to_xml().unwrap();
println!("{}", xml);
```

#### InvoiceRRBuilder (FA_RR)

- `new()` – create a new RR invoice builder
- `set_header(self, header: Header)` – set the header
- `set_subject1(self, subject1: Subject1)` – set farmer data
- `set_subject2(self, subject2: Subject2)` – set buyer data
- `set_currency(self, code: &str)` – currency code (e.g. "PLN")
- `set_issue_date(self, date: NaiveDate)` – issue date
- `set_invoice_number(self, number: &str)` – invoice number
- `set_net_value(self, value: Decimal)` – net value
- `set_tax_value(self, value: Decimal)` – VAT value
- `set_total_value(self, value: Decimal)` – gross value
- `set_total_value_pln(self, value: Decimal)` – gross value in PLN (field P_12_1W)
- `set_total_value_words(self, value: &str)` – value in words (field P_12_2)
- `set_invoice_type(self, typ: InvoiceType)` – invoice type (VatRr or KorVatRr)
- `set_footer(self, footer: Footer)` – footer (supports Stopka→Informacje→StopkaFaktury structure)
- `build(self)` – build the invoice

---

### SellerBuilder

`new(nip: &str, name: &str)` -> `SellerBuilder`
* nip: VAT ID (NIP) of the seller
* name: name of the seller

`set_address(self, 
  country_code: &str, 
  street: &str, 
  building_number: &str, 
  flat_number: Option<&str>, 
  city: &str, 
  postal_code: &str)` -> `SellerBuilder`
  
* country_code: country code (PL)
* street: street
* building_number: building number
* flat_number: flat number (optional)
* city: city
* postal_code: postal code

`build(self)` -> `Subject1 (Object representing the seller)`

### BuyerBuilder

`new(nip: &str, name: &str)` -> `BuyerBuilder`
* nip: VAT ID (NIP) of the buyer
* name: name of the buyer

`set_address(self, 
  country_code: &str, 
  street: &str, 
  building_number: &str, 
  flat_number: Option<&str>, 
  city: &str, 
  postal_code: &str)` -> `BuyerBuilder`
  
* country_code: country code (PL)
* street: street
* building_number: building number
* flat_number: flat number (optional)
* city: city
* postal_code: postal code

`build(self)` -> `Subject2 (Object representing the buyer)`

### LineBuilder

`new(name: &str, measure_unit: &str, quantity: Decimal, net_price: Decimal, tax_rate: &str)` -> `LineBuilder`
* name: name of the good or service
* measure_unit: unit of measure (e.g. "pcs")
* quantity: quantity
* net_price: net unit price
* tax_rate: VAT rate (e.g. "23", "8", "zw")

`build(self)` -> `InvoiceLine (Object representing an invoice line)`

### InvoiceBuilder

`new()` -> `InvoiceBuilder`

`set_invoice_number(self, s: &str)` -> `InvoiceBuilder`
* s: invoice number

`set_issue_date(self, date: NaiveDate)` -> `InvoiceBuilder`
* date: issue date

`set_sale_date(self, date: NaiveDate)` -> `InvoiceBuilder`
* date: sale date

`set_currency(self, code: &str)` -> `InvoiceBuilder`
* code: currency code (e.g. "PLN")

`set_seller(self, seller: Subject1)` -> `InvoiceBuilder`
* seller: seller object (from SellerBuilder)

`set_buyer(self, buyer: Subject2)` -> `InvoiceBuilder`
* buyer: buyer object (from BuyerBuilder)

`add_line(self, line: InvoiceLine)` -> `InvoiceBuilder`
* line: invoice line (from LineBuilder)

`add_lines(self, lines: Vec<InvoiceLine>)` -> `InvoiceBuilder`
* lines: vector of invoice lines

`set_exemption(self, exempt_delivery: u8, exempt_delivery_none: Option<u8>, basis_text: Option<&str>, _legacy_basis: Option<&str>, basis_directive: Option<&str>)` -> `InvoiceBuilder`
* exempt_delivery: exemption flag (1 - exempt, 0 - none)
* exempt_delivery_none: option (usually None if exempt)
* basis_text: legal basis for exemption
* basis_directive: EU directive
* _legacy_basis: deprecated parameter (unused)

`set_new_transport(self, is_new_transport: u8, is_new_transport_intra_community: Option<u8>)` -> `InvoiceBuilder`
* is_new_transport: is it a new means of transport
* is_new_transport_intra_community: Intra-Community Supply of a new means of transport

`set_margin_procedures(self, tourism_services: Option<u8>, used_goods: Option<u8>, works_of_art: Option<u8>, collector_items: Option<u8>, antiques: Option<u8>)` -> `InvoiceBuilder`
* tourism_services: margin procedure for travel agents
* used_goods: margin procedure - used goods
* works_of_art: margin procedure - works of art
* collector_items: margin procedure - collector's items
* antiques: margin procedure - antiques

`build(self)` -> `Result<Invoice, String>`

---

### PEF (UBL)

```rust
use invoice_gen::pef_3::builder::{
    InvoiceBuilder, InvoiceLineBuilder, MonetaryTotalBuilder, PartyBuilder,
    PostalAddressBuilder, TaxTotalBuilder,
};

let address = PostalAddressBuilder::new()
    .city_name("Warsaw")
    .country("PL")
    .build();

let supplier = PartyBuilder::new()
    .party_legal_entity("Seller Name Ltd.", None)
    .postal_address(address.clone())
    .party_tax_scheme("PL5261234567", "VAT")
    .build();

let customer = PartyBuilder::new()
    .party_legal_entity("Buyer Name S.A.", None)
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
    .item("Item Name")
    .price_amount("PLN", "100.00")
    .tax_category("S", "23", "VAT")
    .build();

let invoice = InvoiceBuilder::new(
    "INV-001",
    "2024-01-01",
    supplier,
    customer,
    total,
    vec![line],
)
.due_date("2024-01-15")
.add_tax_total(tax_total)
.build();

let xml = invoice.to_xml().unwrap();
println!("{}", xml);
```

---

### PEF Builders

#### InvoiceBuilder (PEF)

`new(id: impl Into<String>, issue_date: impl Into<String>, accounting_supplier_party: PartyType, accounting_customer_party: PartyType, legal_monetary_total: MonetaryTotal, invoice_line: Vec<InvoiceLine>)` -> `InvoiceBuilder`

`due_date(self, date: impl Into<String>)` -> `InvoiceBuilder`
* date: payment due date

`add_tax_total(self, total: TaxTotal)` -> `InvoiceBuilder`
* total: tax total (from TaxTotalBuilder)

`build(self)` -> `Invoice`

#### PartyBuilder

`new()` -> `PartyBuilder`

`party_legal_entity(self, registration_name: impl Into<String>, company_id: Option<Identifier>)` -> `PartyBuilder`
* registration_name: legal name (required for BR-06/BR-07)

`party_name(self, name: impl Into<String>)` -> `PartyBuilder`
* name: trading name

`postal_address(self, address: PostalAddress)` -> `PartyBuilder`
* address: address (from PostalAddressBuilder)

`party_tax_scheme(self, company_id: impl Into<String>, tax_scheme_id: impl Into<String>)` -> `PartyBuilder`
* company_id: tax ID (e.g. NIP)
* tax_scheme_id: scheme identifier (e.g. VAT)

`build(self)` -> `PartyType`

#### MonetaryTotalBuilder

`new()` -> `MonetaryTotalBuilder`

`line_extension_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Sum of line net amounts

`tax_exclusive_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Invoice net amount

`tax_inclusive_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Invoice gross amount

`payable_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Amount due for payment

`build(self)` -> `MonetaryTotal`

#### InvoiceLineBuilder

`new(id: impl Into<String>)` -> `InvoiceLineBuilder`

`invoiced_quantity(self, unit_code: impl Into<String>, value: impl Into<String>)` -> `InvoiceLineBuilder`
* Invoiced quantity

`line_extension_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `InvoiceLineBuilder`
* Line net amount

`item(self, name: impl Into<String>)` -> `InvoiceLineBuilder`
* Item name

`price_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `InvoiceLineBuilder`
* Net unit price

`tax_category(self, id: impl Into<String>, percent: impl Into<String>, tax_scheme_id: impl Into<String>)` -> `InvoiceLineBuilder`
* Tax category (e.g. S, 23, VAT)

`build(self)` -> `InvoiceLine`
