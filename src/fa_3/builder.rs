use super::models::*;
use crate::shared::models::{
    Address, CountryCode, CurrencyCode, TRolaPodmiotu3, TRolaPodmiotuUpowaznionego, TaxRate,
};
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
pub struct SellerBuilder {
    subject: Subject1,
}

impl SellerBuilder {
    pub fn new(nip: &str, name: &str) -> Self {
        let mut s = Subject1::default();
        s.identification_data.nip = nip.to_string();
        s.identification_data.name = name.to_string();
        Self { subject: s }
    }

    pub fn set_address(
        mut self,
        country_code: &str,
        street: &str,
        building_number: &str,
        flat_number: Option<&str>,
        city: &str,
        postal_code: &str,
    ) -> Self {
        let mut line = format!("{} {}", street, building_number);
        if let Some(flat) = flat_number {
            line = format!("{} / {}", line, flat);
        }
        line = format!("{}, {} {}", line, postal_code, city);

        self.subject.address = Address {
            country_code: CountryCode::new(country_code),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        };
        self
    }

    pub fn add_contact_data(mut self, email: Option<&str>, phone: Option<&str>) -> Self {
        if email.is_some() || phone.is_some() {
            self.subject
                .contact_data
                .push(crate::shared::models::ContactData {
                    email: email.map(|s| s.to_string()),
                    phone: phone.map(|s| s.to_string()),
                });
        }
        self
    }

    pub fn build(self) -> Subject1 {
        self.subject
    }
}

pub struct BuyerBuilder {
    subject: Subject2,
}

impl BuyerBuilder {
    pub fn new(nip: &str, name: &str) -> Self {
        let mut s = Subject2::default();
        s.identification_data = Some(IdentificationData2 {
            nip: Some(nip.to_string()),
            name: Some(name.to_string()),
            ..Default::default()
        });
        Self { subject: s }
    }

    pub fn set_address(
        mut self,
        country_code: &str,
        street: &str,
        building_number: &str,
        flat_number: Option<&str>,
        city: &str,
        postal_code: &str,
    ) -> Self {
        let mut line = format!("{} {}", street, building_number);
        if let Some(flat) = flat_number {
            line = format!("{} / {}", line, flat);
        }
        line = format!("{}, {} {}", line, postal_code, city);

        self.subject.address = Some(Address {
            country_code: CountryCode::new(country_code),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        });
        self
    }

    pub fn set_contact_data(mut self, email: Option<&str>, phone: Option<&str>) -> Self {
        if email.is_some() || phone.is_some() {
            self.subject.contact_data = Some(crate::shared::models::ContactData {
                email: email.map(|s| s.to_string()),
                phone: phone.map(|s| s.to_string()),
            });
        }
        self
    }

    pub fn set_client_number(mut self, number: &str) -> Self {
        self.subject.client_number = Some(number.to_string());
        self
    }

    pub fn set_buyer_id(mut self, id: &str) -> Self {
        self.subject.buyer_id = Some(id.to_string());
        self
    }

    pub fn set_jst(mut self, jst: u8) -> Self {
        self.subject.jst = jst;
        self
    }

    pub fn set_gv(mut self, gv: u8) -> Self {
        self.subject.gv = gv;
        self
    }

    pub fn build(self) -> Subject2 {
        self.subject
    }
}

pub struct Subject3Builder {
    subject: Subject3,
}

impl Subject3Builder {
    pub fn new(nip: &str, name: &str) -> Self {
        let mut s = Subject3::default();
        s.identification_data.nip = Some(nip.to_string());
        s.identification_data.name = Some(name.to_string());
        Self { subject: s }
    }

    pub fn set_address(
        mut self,
        country_code: &str,
        street: &str,
        building_number: &str,
        flat_number: Option<&str>,
        city: &str,
        postal_code: &str,
    ) -> Self {
        let mut line = format!("{} {}", street, building_number);
        if let Some(flat) = flat_number {
            line = format!("{} / {}", line, flat);
        }
        line = format!("{}, {} {}", line, postal_code, city);

        self.subject.address = Some(Address {
            country_code: CountryCode::new(country_code),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        });
        self
    }

    pub fn add_contact_data(mut self, email: Option<&str>, phone: Option<&str>) -> Self {
        if email.is_some() || phone.is_some() {
            self.subject
                .contact_data
                .push(crate::shared::models::ContactData {
                    email: email.map(|s| s.to_string()),
                    phone: phone.map(|s| s.to_string()),
                });
        }
        self
    }

    pub fn set_role(mut self, role: TRolaPodmiotu3) -> Self {
        self.subject.role = Some(role);
        self.subject.other_role_flag = None;
        self.subject.role_description = None;
        self
    }

    pub fn set_other_role(mut self, description: &str) -> Self {
        self.subject.role = None;
        self.subject.other_role_flag = Some(1);
        self.subject.role_description = Some(description.to_string());
        self
    }

    pub fn set_share(mut self, share: &str) -> Self {
        self.subject.share = Some(share.to_string());
        self
    }

    pub fn set_client_number(mut self, number: &str) -> Self {
        self.subject.client_number = Some(number.to_string());
        self
    }

    pub fn build(self) -> Subject3 {
        self.subject
    }
}

pub struct AuthorizedSubjectBuilder {
    subject: AuthorizedSubject,
}

impl AuthorizedSubjectBuilder {
    pub fn new(nip: &str, name: &str, role: TRolaPodmiotuUpowaznionego) -> Self {
        let mut s = AuthorizedSubject::default();
        s.identification_data.nip = Some(nip.to_string());
        s.identification_data.name = Some(name.to_string());
        s.role = role;
        Self { subject: s }
    }

    pub fn set_address(
        mut self,
        country_code: &str,
        street: &str,
        building_number: &str,
        flat_number: Option<&str>,
        city: &str,
        postal_code: &str,
    ) -> Self {
        let mut line = format!("{} {}", street, building_number);
        if let Some(flat) = flat_number {
            line = format!("{} / {}", line, flat);
        }
        line = format!("{}, {} {}", line, postal_code, city);

        self.subject.address = Address {
            country_code: CountryCode::new(country_code),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        };
        self
    }

    pub fn add_contact_data(mut self, email: Option<&str>, phone: Option<&str>) -> Self {
        if email.is_some() || phone.is_some() {
            self.subject
                .contact_data
                .push(AuthorizedSubjectContactData {
                    email: email.map(|s| s.to_string()),
                    phone: phone.map(|s| s.to_string()),
                });
        }
        self
    }

    pub fn build(self) -> AuthorizedSubject {
        self.subject
    }
}

pub struct Subject1KBuilder {
    subject: Subject1K,
}

impl Subject1KBuilder {
    pub fn new(nip: &str, name: &str) -> Self {
        let mut s = Subject1K::default();
        s.identification_data.nip = nip.to_string();
        s.identification_data.name = name.to_string();
        Self { subject: s }
    }

    pub fn set_address(
        mut self,
        country_code: &str,
        street: &str,
        building_number: &str,
        flat_number: Option<&str>,
        city: &str,
        postal_code: &str,
    ) -> Self {
        let mut line = format!("{} {}", street, building_number);
        if let Some(flat) = flat_number {
            line = format!("{} / {}", line, flat);
        }
        line = format!("{}, {} {}", line, postal_code, city);

        self.subject.address = Address {
            country_code: crate::shared::models::CountryCode::new(country_code),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        };
        self
    }

    pub fn set_taxpayer_prefix(mut self, prefix: &str) -> Self {
        self.subject.taxpayer_prefix = Some(prefix.to_string());
        self
    }

    pub fn build(self) -> Subject1K {
        self.subject
    }
}

pub struct Subject2KBuilder {
    subject: Subject2K,
}

impl Subject2KBuilder {
    pub fn new() -> Self {
        Self {
            subject: Subject2K::default(),
        }
    }

    pub fn set_nip(mut self, nip: &str) -> Self {
        self.subject.identification_data.nip = Some(nip.to_string());
        self
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.subject.identification_data.name = Some(name.to_string());
        self
    }

    pub fn set_address(
        mut self,
        country_code: &str,
        street: &str,
        building_number: &str,
        flat_number: Option<&str>,
        city: &str,
        postal_code: &str,
    ) -> Self {
        let mut line = format!("{} {}", street, building_number);
        if let Some(flat) = flat_number {
            line = format!("{} / {}", line, flat);
        }
        line = format!("{}, {} {}", line, postal_code, city);

        self.subject.address = Some(Address {
            country_code: crate::shared::models::CountryCode::new(country_code),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        });
        self
    }

    pub fn set_buyer_id(mut self, id: &str) -> Self {
        self.subject.buyer_id = Some(id.to_string());
        self
    }

    pub fn build(self) -> Subject2K {
        self.subject
    }
}

pub struct LineBuilder {
    line: InvoiceLine,
}

impl LineBuilder {
    pub fn new(name: &str, quantity: Decimal, unit_net_price: Decimal, tax_rate: TaxRate) -> Self {
        // Convert the unit_net_price to integer cents to avoid rounding/floating point issues
        // when performing subsequent arithmetic. We keep the external API using `Decimal`
        // with two fractional digits, but compute net values via integer cents.
        //
        // Steps:
        // 1) unit_price_cents_i128: integer number of cents (unit_net_price * 100, rounded)
        // 2) unit_net_price_dec: normalized Decimal from cents (ensures a canonical two-decimal representation)
        // 3) net_cents = round(quantity * unit_price_cents)
        // 4) net_value = Decimal from net_cents with scale 2
        let unit_price_cents_i128 = (unit_net_price * Decimal::new(100, 0))
            .round_dp(0)
            .to_i128()
            .unwrap_or(0);

        let unit_net_price_dec = Decimal::from_i128_with_scale(unit_price_cents_i128, 2);

        let net_cents_decimal = Decimal::from_i128_with_scale(unit_price_cents_i128, 0) * quantity;
        let net_cents_i128 = net_cents_decimal.round_dp(0).to_i128().unwrap_or(0);
        let net_value = Decimal::from_i128_with_scale(net_cents_i128, 2);

        let line = InvoiceLine {
            line_number: 0,
            uu_id: None,
            delivery_date: None,
            name: Some(name.to_string()),
            index: None,
            gtin: None,
            pkwiu: None,
            cn: None,
            pkob: None,
            unit: Some("szt.".to_string()),
            quantity: Some(quantity),
            // store the canonical two-decimal Decimal reconstructed from integer cents
            unit_net_price: Some(unit_net_price_dec),
            unit_gross_price: None,
            discount: None,
            // net_value computed via integer-cent arithmetic then converted back to Decimal with 2 decimals
            net_value: Some(net_value),
            tax_rate: Some(tax_rate),
            tax_rate_procedure_xii: None,
            tax_rate_appendix_15: None,
            excise_amount: None,
            gtu: None,
            procedure: None,
            currency_rate: None,
            state_before: None,
        };
        Self { line }
    }

    pub fn set_unit(mut self, unit: &str) -> Self {
        self.line.unit = Some(unit.to_string());
        self
    }

    pub fn set_index(mut self, index: &str) -> Self {
        self.line.index = Some(index.to_string());
        self
    }

    pub fn set_pkwiu(mut self, pkwiu: &str) -> Self {
        self.line.pkwiu = Some(pkwiu.to_string());
        self
    }

    pub fn set_cn(mut self, cn: &str) -> Self {
        self.line.cn = Some(cn.to_string());
        self
    }

    pub fn set_pkob(mut self, pkob: &str) -> Self {
        self.line.pkob = Some(pkob.to_string());
        self
    }

    pub fn set_gtin(mut self, gtin: &str) -> Self {
        self.line.gtin = Some(gtin.to_string());
        self
    }

    pub fn set_delivery_date(mut self, date: &str) -> Self {
        self.line.delivery_date = Some(date.to_string());
        self
    }

    pub fn set_gtu(mut self, gtu: GTU) -> Self {
        self.line.gtu = Some(gtu);
        self
    }

    pub fn set_procedure(mut self, procedure: Procedure) -> Self {
        self.line.procedure = Some(procedure);
        self
    }

    pub fn set_uu_id(mut self, uu_id: &str) -> Self {
        self.line.uu_id = Some(uu_id.to_string());
        self
    }

    pub fn set_unit_gross_price(mut self, price: Decimal) -> Self {
        self.line.unit_gross_price = Some(price);
        self
    }

    pub fn set_discount(mut self, discount: Decimal) -> Self {
        self.line.discount = Some(discount);
        self
    }

    pub fn set_net_value(mut self, net_value: Decimal) -> Self {
        self.line.net_value = Some(net_value);
        self
    }

    pub fn set_excise_amount(mut self, amount: Decimal) -> Self {
        self.line.excise_amount = Some(amount);
        self
    }

    pub fn set_tax_rate_procedure_xii(mut self, rate: u8) -> Self {
        self.line.tax_rate_procedure_xii = Some(rate);
        self
    }

    pub fn set_tax_rate_appendix_15(mut self, rate: u8) -> Self {
        self.line.tax_rate_appendix_15 = Some(rate);
        self
    }

    pub fn set_currency_rate(mut self, rate: Decimal) -> Self {
        self.line.currency_rate = Some(rate);
        self
    }

    pub fn set_state_before(mut self, state: u8) -> Self {
        self.line.state_before = Some(state);
        self
    }

    pub fn build(self) -> InvoiceLine {
        self.line
    }
}

pub struct OrderLineBuilder {
    line: OrderLine,
}

impl OrderLineBuilder {
    pub fn new(
        name: &str,
        measure_unit: &str,
        quantity: Decimal,
        net_price: Decimal,
        tax_rate: TaxRate,
    ) -> Self {
        // Convert the unit net price to integer cents to avoid rounding issues,
        // then reconstruct canonical Decimal values with scale 2 for storage.
        let unit_price_cents_i128 = (net_price * Decimal::new(100, 0))
            .round_dp(0)
            .to_i128()
            .unwrap_or(0);

        // canonical unit price Decimal with two decimals reconstructed from cents
        let unit_net_price = Decimal::from_i128_with_scale(unit_price_cents_i128, 2);

        // Compute net value in cents: (unit_price_cents * quantity), round to nearest cent
        let net_cents_decimal = Decimal::from_i128_with_scale(unit_price_cents_i128, 0) * quantity;
        let net_cents_i128 = net_cents_decimal.round_dp(0).to_i128().unwrap_or(0);
        let net_value = Decimal::from_i128_with_scale(net_cents_i128, 2);

        // Compute VAT in cents from net_cents_i128 and tax rate basis points
        let vat_value: Option<Decimal> = {
            let vat_cents = (net_cents_i128 * tax_rate.basis_points() as i128) / 10000;
            Some(Decimal::from_i128_with_scale(vat_cents, 2))
        };

        let line = OrderLine {
            line_number: 0,
            uu_id: None,
            name: Some(name.to_string()),
            index: None,
            gtin: None,
            pkwiu_code: None,
            cn_code: None,
            pkob_code: None,
            unit: Some(measure_unit.to_string()),
            quantity: Some(quantity),
            // store the canonical two-decimal Decimal reconstructed from integer cents
            unit_net_price: Some(unit_net_price),
            // net_value computed via integer-cent arithmetic then converted back to Decimal with 2 decimals
            net_value: Some(net_value),
            vat_value,
            tax_rate: Some(tax_rate),
            tax_rate_procedure_xii: None,
            tax_rate_appendix_15: None,
            gtu: None,
            procedure: None,
            excise_amount: None,
            state_before: None,
        };
        Self { line }
    }

    pub fn set_gtu(mut self, gtu: GTU) -> Self {
        self.line.gtu = Some(gtu);
        self
    }

    pub fn set_gtin(mut self, gtin: &str) -> Self {
        self.line.gtin = Some(gtin.to_string());
        self
    }

    pub fn set_pkwiu(mut self, pkwiu: &str) -> Self {
        self.line.pkwiu_code = Some(pkwiu.to_string());
        self
    }

    pub fn build(self) -> OrderLine {
        self.line
    }
}

pub struct OrderBuilder {
    order: Order,
    lines: Vec<OrderLine>,
}

impl OrderBuilder {
    pub fn new() -> Self {
        Self {
            order: Order::default(),
            lines: Vec::new(),
        }
    }

    pub fn add_line(mut self, mut line: OrderLine) -> Self {
        let line_number = (self.lines.len() + 1) as u32;
        line.line_number = line_number;
        self.lines.push(line);
        self
    }

    pub fn build(mut self) -> Order {
        let mut total_value = Decimal::ZERO;
        for line in &self.lines {
            if let Some(net) = line.net_value {
                total_value += net;
            }
            if let Some(vat) = line.vat_value {
                total_value += vat;
            }
        }
        self.order.order_value = total_value;
        self.order.lines = self.lines;
        self.order
    }
}

#[derive(Default)]
pub struct InvoiceBuilder {
    invoice: Invoice,
}

impl InvoiceBuilder {
    pub fn new() -> Self {
        Self {
            invoice: Invoice::default(),
        }
    }

    pub fn set_issue_date(mut self, date: NaiveDate) -> Self {
        self.invoice.invoice_body.issue_date = date.format("%Y-%m-%d").to_string();
        self
    }

    pub fn set_invoice_number(mut self, number: &str) -> Self {
        self.invoice.invoice_body.invoice_number = number.to_string();
        self
    }

    pub fn add_line(mut self, mut line: InvoiceLine) -> Self {
        line.line_number = (self.invoice.invoice_body.lines.len() + 1) as u32;
        self.invoice.invoice_body.lines.push(line);
        self
    }

    pub fn set_currency(mut self, currency: &str) -> Self {
        self.invoice.invoice_body.currency_code = CurrencyCode::new(currency);
        self
    }

    pub fn set_seller(mut self, seller: Subject1) -> Self {
        self.invoice.subject1 = seller;
        self
    }

    pub fn set_buyer(mut self, buyer: Subject2) -> Self {
        self.invoice.subject2 = buyer;
        self
    }

    pub fn set_subject3(mut self, subject3: Subject3) -> Self {
        self.invoice.subject3 = Some(subject3);
        self
    }

    pub fn set_authorized_subject(mut self, authorized_subject: AuthorizedSubject) -> Self {
        self.invoice.authorized_subject = Some(authorized_subject);
        self
    }

    pub fn set_order(mut self, order: Order) -> Self {
        self.invoice.order = Some(order);
        self
    }

    pub fn set_ksef(mut self, ksef: PaymentIdentifier) -> Self {
        self.invoice.kse_f = Some(ksef);
        self
    }

    pub fn set_transaction_terms(mut self, terms: TransactionTerms) -> Self {
        self.invoice.transaction_terms = Some(terms);
        self
    }

    pub fn set_basic_rate_totals(
        mut self,
        net: Decimal,
        tax: Decimal,
        tax_foreign: Option<Decimal>,
    ) -> Self {
        self.invoice.invoice_body.net_total_basic_rate = Some(net);
        self.invoice.invoice_body.tax_total_basic_rate = Some(tax);
        self.invoice.invoice_body.tax_total_basic_rate_foreign = tax_foreign;
        self
    }

    pub fn set_reduced_rate_1_totals(
        mut self,
        net: Decimal,
        tax: Decimal,
        tax_foreign: Option<Decimal>,
    ) -> Self {
        self.invoice.invoice_body.net_total_reduced_rate_1 = Some(net);
        self.invoice.invoice_body.tax_total_reduced_rate_1 = Some(tax);
        self.invoice.invoice_body.tax_total_reduced_rate_1_foreign = tax_foreign;
        self
    }

    pub fn set_settlement(mut self, settlement: Settlement) -> Self {
        self.invoice.invoice_body.settlement = Some(settlement);
        self
    }

    pub fn set_payment(mut self, payment: Payment) -> Self {
        self.invoice.invoice_body.payment = Some(payment);
        self
    }

    pub fn set_reduced_rate_2_totals(
        mut self,
        net: Decimal,
        tax: Decimal,
        tax_foreign: Option<Decimal>,
    ) -> Self {
        self.invoice.invoice_body.net_total_reduced_rate_2 = Some(net);
        self.invoice.invoice_body.tax_total_reduced_rate_2 = Some(tax);
        self.invoice.invoice_body.tax_total_reduced_rate_2_foreign = tax_foreign;
        self
    }

    pub fn set_taxi_totals(
        mut self,
        net: Decimal,
        tax: Decimal,
        tax_foreign: Option<Decimal>,
    ) -> Self {
        self.invoice.invoice_body.net_total_taxi = Some(net);
        self.invoice.invoice_body.tax_total_taxi = Some(tax);
        self.invoice.invoice_body.tax_total_taxi_foreign = tax_foreign;
        self
    }

    pub fn set_special_procedure_totals(mut self, net: Decimal, tax: Decimal) -> Self {
        self.invoice.invoice_body.net_total_special = Some(net);
        self.invoice.invoice_body.tax_total_special = Some(tax);
        self
    }

    pub fn set_zero_rate_totals(
        mut self,
        domestic: Option<Decimal>,
        intra: Option<Decimal>,
        export: Option<Decimal>,
    ) -> Self {
        self.invoice.invoice_body.net_total_0_domestic = domestic;
        self.invoice.invoice_body.net_total_0_intra = intra;
        self.invoice.invoice_body.net_total_0_export = export;
        self
    }

    pub fn set_margin_procedures(
        mut self,
        tourism_services: Option<u8>,
        used_goods: Option<u8>,
        works_of_art: Option<u8>,
        collector_items: Option<u8>,
    ) -> Self {
        let any_procedure = tourism_services.is_some()
            || used_goods.is_some()
            || works_of_art.is_some()
            || collector_items.is_some();

        let mp = &mut self.invoice.invoice_body.annotations.margin_procedures;
        mp.procedure_flag = if any_procedure { Some(1) } else { None };
        mp.travel = tourism_services;
        mp.used_goods = used_goods;
        mp.art = works_of_art;
        mp.collectors = collector_items;
        mp.none = if any_procedure { None } else { Some(1) };
        self
    }

    pub fn set_simplified_procedure(mut self, is_simplified: bool) -> Self {
        self.invoice.invoice_body.annotations.simplified_procedure =
            if is_simplified { 1 } else { 2 };
        self
    }

    pub fn set_subject1_k(mut self, subject1_k: Subject1K) -> Self {
        self.invoice.invoice_body.subject1_k = Some(subject1_k);
        self
    }

    pub fn add_subject2_k(mut self, subject2_k: Subject2K) -> Self {
        self.invoice.invoice_body.subject2_k.push(subject2_k);
        self
    }

    pub fn set_amount_paid_before_correction(mut self, amount: Decimal) -> Self {
        self.invoice.invoice_body.amount_paid_before_correction = Some(amount);
        self
    }

    pub fn set_cash_accounting(mut self, is_cash_accounting: bool) -> Self {
        self.invoice.invoice_body.annotations.cash_accounting =
            if is_cash_accounting { 1 } else { 2 };
        self
    }

    pub fn set_self_billing(mut self, is_self_billing: bool) -> Self {
        self.invoice.invoice_body.annotations.self_billing = if is_self_billing { 1 } else { 2 };
        self
    }

    pub fn set_reverse_charge(mut self, is_reverse_charge: bool) -> Self {
        self.invoice.invoice_body.annotations.reverse_charge =
            if is_reverse_charge { 1 } else { 2 };
        self
    }

    pub fn set_split_payment(mut self, is_split_payment: bool) -> Self {
        self.invoice.invoice_body.annotations.split_payment = if is_split_payment { 1 } else { 2 };
        self
    }

    pub fn set_exemption(
        mut self,
        exempt_delivery: Option<u8>,
        basis_legal: Option<&str>,
        basis_directive: Option<&str>,
        basis_other: Option<&str>,
    ) -> Self {
        let exempt_delivery_none = if exempt_delivery.is_some() {
            None
        } else {
            Some(1)
        };

        self.invoice.invoice_body.annotations.exemption = Exemption {
            exempt_delivery,
            basis_legal: basis_legal.map(|s| s.to_string()),
            basis_directive: basis_directive.map(|s| s.to_string()),
            basis_other: basis_other.map(|s| s.to_string()),
            exempt_delivery_none,
        };
        self
    }

    pub fn build(self) -> Result<Invoice, String> {
        Ok(self.invoice)
    }
}
