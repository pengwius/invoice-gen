Polska wersja / [English version](README.en.md)

# invoice-gen

## Instalacja

```rust
[dependencies]
invoice-gen = "0.2"
```

## Użycie

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
* nip: NIP sprzedawcy
* name: nazwa sprzedawcy

`set_address(self, 
  country_code: &str, 
  street: &str, 
  building_number: &str, 
  flat_number: Option<&str>, 
  city: &str, 
  postal_code: &str)` -> `SellerBuilder`
  
* country_code: kod kraju (PL)
* street: ulica
* building_number: numer budynku
* flat_number: numer lokalu (opcjonalny)
* city: miasto
* postal_code: kod pocztowy

`build(self)` -> `Subject1 (Obiekt reprezentujący sprzedawcę)`

### BuyerBuilder

`new(nip: &str, name: &str)` -> `BuyerBuilder`
* nip: NIP nabywcy
* name: nazwa nabywcy

`set_address(self, 
  country_code: &str, 
  street: &str, 
  building_number: &str, 
  flat_number: Option<&str>, 
  city: &str, 
  postal_code: &str)` -> `BuyerBuilder`
  
* country_code: kod kraju (PL)
* street: ulica
* building_number: numer budynku
* flat_number: numer lokalu (opcjonalny)
* city: miasto
* postal_code: kod pocztowy

`build(self)` -> `Subject2 (Obiekt reprezentujący nabywcę)`

### LineBuilder

`new(name: &str, measure_unit: &str, quantity: Decimal, net_price: Decimal, tax_rate: &str)` -> `LineBuilder`
* name: nazwa towaru lub usługi
* measure_unit: jednostka miary (np. "szt")
* quantity: ilość
* net_price: cena netto jednostkowa
* tax_rate: stawka VAT (np. "23", "8", "zw")

`build(self)` -> `InvoiceLine (Obiekt reprezentujący wiersz faktury)`

### InvoiceBuilder

`new()` -> `InvoiceBuilder`

`set_invoice_number(self, s: &str)` -> `InvoiceBuilder`
* s: numer faktury

`set_issue_date(self, date: NaiveDate)` -> `InvoiceBuilder`
* date: data wystawienia

`set_sale_date(self, date: NaiveDate)` -> `InvoiceBuilder`
* date: data sprzedaży

`set_currency(self, code: &str)` -> `InvoiceBuilder`
* code: kod waluty (np. "PLN")

`set_seller(self, seller: Subject1)` -> `InvoiceBuilder`
* seller: obiekt sprzedawcy (z SellerBuilder)

`set_buyer(self, buyer: Subject2)` -> `InvoiceBuilder`
* buyer: obiekt nabywcy (z BuyerBuilder)

`add_line(self, line: InvoiceLine)` -> `InvoiceBuilder`
* line: wiersz faktury (z LineBuilder)

`add_lines(self, lines: Vec<InvoiceLine>)` -> `InvoiceBuilder`
* lines: wektor wierszy faktury

`set_exemption(self, exempt_delivery: u8, exempt_delivery_none: Option<u8>, basis_text: Option<&str>, _legacy_basis: Option<&str>, basis_directive: Option<&str>)` -> `InvoiceBuilder`
* exempt_delivery: znacznik zwolnienia (1 - zwolnienie, 0 - brak)
* exempt_delivery_none: opcja (zazwyczaj None jeśli jest zwolnienie)
* basis_text: podstawa prawna zwolnienia
* basis_directive: dyrektywa unijna
* _legacy_basis: parametr przestarzały (nieużywany)

`set_new_transport(self, is_new_transport: u8, is_new_transport_intra_community: Option<u8>)` -> `InvoiceBuilder`
* is_new_transport: czy jest to nowy środek transportu
* is_new_transport_intra_community: WDT nowego środka transportu

`set_margin_procedures(self, tourism_services: Option<u8>, used_goods: Option<u8>, works_of_art: Option<u8>, collector_items: Option<u8>, antiques: Option<u8>)` -> `InvoiceBuilder`
* tourism_services: procedura marży dla biur podróży
* used_goods: procedura marży - towary używane
* works_of_art: procedura marży - dzieła sztuki
* collector_items: procedura marży - przedmioty kolekcjonerskie
* antiques: procedura marży - antyki

`build(self)` -> `Result<Invoice, String>`
