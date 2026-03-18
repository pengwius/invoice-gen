[Polska wersja](README.md) / English version

# invoice-gen

## Installation

```rust
[dependencies]
invoice-gen = "0.0.3"
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
