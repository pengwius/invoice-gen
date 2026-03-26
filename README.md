Polska wersja / [English version](README.en.md)

# invoice-gen

## Instalacja

```rust
[dependencies]
invoice-gen = "0.0.5"
```

## Użycie

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

### FA_RR (Faktura VAT RR – rolnik ryczałtowy)

Obsługa faktur VAT RR zgodnych ze wzorem Ministerstwa Finansów (rolnik ryczałtowy). Pozwala na generowanie faktur RR w formacie XML zgodnym ze schemą XSD.

```rust
use invoice_gen::fa_rr::builder::{InvoiceRRBuilder, Subject1Builder, Subject2Builder, AddressBuilder, FooterBuilder};
use invoice_gen::fa_rr::models::Header;
use rust_decimal::Decimal;
use chrono::NaiveDate;
use invoice_gen::shared::models::IdentificationData;

let subject1 = Subject1Builder::new(IdentificationData {
        nip: "5261234567".to_string(),
        name: "Rolnik Jan Kowalski".to_string(),
    })
    .set_address(
        AddressBuilder::new()
            .set_country_code("PL")
            .set_address_line_1("Wiejska 1")
            .set_line2("Gospodarstwo 2")
            .build(),
    )
    .build();

let subject2 = Subject2Builder::new(IdentificationData {
        nip: "9876543210".to_string(),
        name: "Firma Skupująca Sp. z o.o.".to_string(),
    })
    .set_address(
        AddressBuilder::new()
            .set_country_code("PL")
            .set_address_line_1("Przemysłowa 5")
            .build(),
    )
    .build();

let footer = FooterBuilder::new()
    .set_footer_text("Dziękujemy za współpracę.")
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
    .set_total_value_words("jeden tysiąc sześćdziesiąt pięć złotych 00/100")
    .set_invoice_type(invoice_gen::fa_rr::models::InvoiceType::VatRr)
    .set_footer(footer)
    .build();

let xml = invoice.to_xml().unwrap();
println!("{}", xml);
```

#### InvoiceRRBuilder (FA_RR)

- `new()` – utwórz nowy builder faktury RR
- `set_header(self, header: Header)` – ustaw nagłówek
- `set_subject1(self, subject1: Subject1)` – ustaw dane rolnika
- `set_subject2(self, subject2: Subject2)` – ustaw dane nabywcy
- `set_currency(self, code: &str)` – kod waluty (np. "PLN")
- `set_issue_date(self, date: NaiveDate)` – data wystawienia
- `set_invoice_number(self, number: &str)` – numer faktury
- `set_net_value(self, value: Decimal)` – wartość netto
- `set_tax_value(self, value: Decimal)` – wartość VAT
- `set_total_value(self, value: Decimal)` – wartość brutto
- `set_total_value_pln(self, value: Decimal)` – wartość brutto w PLN (pole P_12_1W)
- `set_total_value_words(self, value: &str)` – wartość słownie (pole P_12_2)
- `set_invoice_type(self, typ: InvoiceType)` – typ faktury (VatRr lub KorVatRr)
- `set_footer(self, footer: Footer)` – stopka (obsługuje strukturę Stopka→Informacje→StopkaFaktury)
- `build(self)` – zbuduj fakturę

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

---

### PEF (UBL)

```rust
use invoice_gen::pef_3::builder::{
    InvoiceBuilder, InvoiceLineBuilder, MonetaryTotalBuilder, PartyBuilder,
    PostalAddressBuilder, TaxTotalBuilder,
};

let address = PostalAddressBuilder::new()
    .city_name("Warszawa")
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

#### InvoiceBuilder (PEF)

`new(id: impl Into<String>, issue_date: impl Into<String>, accounting_supplier_party: PartyType, accounting_customer_party: PartyType, legal_monetary_total: MonetaryTotal, invoice_line: Vec<InvoiceLine>)` -> `InvoiceBuilder`

`due_date(self, date: impl Into<String>)` -> `InvoiceBuilder`
* date: data płatności

`add_tax_total(self, total: TaxTotal)` -> `InvoiceBuilder`
* total: suma podatkowa (z TaxTotalBuilder)

`build(self)` -> `Invoice`

#### PartyBuilder

`new()` -> `PartyBuilder`

`party_legal_entity(self, registration_name: impl Into<String>, company_id: Option<Identifier>)` -> `PartyBuilder`
* registration_name: nazwa prawna (wymagane dla BR-06/BR-07)

`party_name(self, name: impl Into<String>)` -> `PartyBuilder`
* name: nazwa handlowa

`postal_address(self, address: PostalAddress)` -> `PartyBuilder`
* address: adres (z PostalAddressBuilder)

`party_tax_scheme(self, company_id: impl Into<String>, tax_scheme_id: impl Into<String>)` -> `PartyBuilder`
* company_id: numer podatkowy (np. NIP)
* tax_scheme_id: identyfikator schematu (np. VAT)

`build(self)` -> `PartyType`

#### MonetaryTotalBuilder

`new()` -> `MonetaryTotalBuilder`

`line_extension_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Suma wartości netto linii

`tax_exclusive_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Suma netto faktury

`tax_inclusive_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Suma brutto faktury

`payable_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `MonetaryTotalBuilder`
* Kwota do zapłaty

`build(self)` -> `MonetaryTotal`

#### InvoiceLineBuilder

`new(id: impl Into<String>)` -> `InvoiceLineBuilder`

`invoiced_quantity(self, unit_code: impl Into<String>, value: impl Into<String>)` -> `InvoiceLineBuilder`
* Ilość zafakturowana

`line_extension_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `InvoiceLineBuilder`
* Wartość netto linii

`item(self, name: impl Into<String>)` -> `InvoiceLineBuilder`
* Nazwa towaru/usługi

`price_amount(self, currency: impl Into<String>, value: impl Into<String>)` -> `InvoiceLineBuilder`
* Cena jednostkowa netto

`tax_category(self, id: impl Into<String>, percent: impl Into<String>, tax_scheme_id: impl Into<String>)` -> `InvoiceLineBuilder`
* Kategoria podatkowa (np. S, 23, VAT)

`build(self)` -> `InvoiceLine`
