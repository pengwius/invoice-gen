use super::models::*;
use chrono::NaiveDate;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TaxRate {
    Rate23,
    Rate8,
    Rate5,
    Exempt,
    Custom(i32),
}

impl TaxRate {
    pub fn basis_points(&self) -> i32 {
        match *self {
            TaxRate::Rate23 => 2300,
            TaxRate::Rate8 => 800,
            TaxRate::Rate5 => 500,
            TaxRate::Exempt => 0,
            TaxRate::Custom(n) => n,
        }
    }

    pub fn from_str(s: &str) -> Result<TaxRate, ()> {
        match s {
            "23" => Ok(TaxRate::Rate23),
            "8" => Ok(TaxRate::Rate8),
            "5" => Ok(TaxRate::Rate5),
            "zw" | "ZW" | "Zw" => Ok(TaxRate::Exempt),
            other => {
                if let Ok(d) = Decimal::from_str(other) {
                    let bp_decimal = (d * Decimal::new(100, 0)).round_dp(0);
                    if let Some(bp_i32) = bp_decimal.to_i32() {
                        return Ok(match bp_i32 {
                            2300 => TaxRate::Rate23,
                            800 => TaxRate::Rate8,
                            500 => TaxRate::Rate5,
                            0 => TaxRate::Exempt,
                            other => TaxRate::Custom(other),
                        });
                    }
                }
                Err(())
            }
        }
    }
}

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
            country_code: country_code.to_string(),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        };
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
        s.identification_data = Some(IdentificationData {
            nip: nip.to_string(),
            name: name.to_string(),
        });
        s.nip = nip.to_string();
        s.name = Some(name.to_string());
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
            country_code: country_code.to_string(),
            address_line_1: line,
            address_line_2: None,
            gln: None,
        });
        self
    }

    pub fn build(self) -> Subject2 {
        self.subject
    }
}

pub struct LineBuilder {
    line: InvoiceLine,
}

impl LineBuilder {
    pub fn new(
        name: &str,
        measure_unit: &str,
        quantity: Decimal,
        net_price: Decimal,
        vat_rate: &str,
    ) -> Self {
        let net_value = (quantity * net_price).round_dp(2);
        let line = InvoiceLine {
            line_number: 0,
            uu_id: None,
            sale_date: None,
            name: Some(name.to_string()),
            index: None,
            gtin: None,
            pkwiu_code: None,
            cn_code: None,
            pkob_code: None,
            unit: Some(measure_unit.to_string()),
            quantity: Some(quantity),
            unit_net_price: Some(net_price),
            unit_gross_price: None,
            discount_amount: None,
            net_value: Some(net_value),
            gross_value: None,
            vat_value: None,
            tax_rate: Some(vat_rate.to_string()),
            tax_rate_procedure_xii: None,
            tax_rate_appendix_15: None,
        };
        Self { line }
    }

    pub fn build(self) -> InvoiceLine {
        self.line
    }
}

#[derive(Default)]
pub struct InvoiceBuilder {
    invoice: Invoice,
    lines: Vec<InvoiceLine>,
}

impl InvoiceBuilder {
    pub fn new() -> Self {
        Self {
            invoice: Invoice::default(),
            lines: Vec::new(),
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

    pub fn set_sale_date(mut self, date: NaiveDate) -> Self {
        self.invoice.invoice_body.sale_date = Some(date.format("%Y-%m-%d").to_string());
        self
    }

    pub fn set_currency(mut self, currency: &str) -> Self {
        self.invoice.invoice_body.currency_code = currency.to_string();
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

    pub fn set_exemption(
        mut self,
        exempt_delivery: u8,
        exempt_delivery_none: Option<u8>,
        basis_text: Option<&str>,
        _legacy_basis: Option<&str>,
        basis_directive: Option<&str>,
    ) -> Self {
        let resolved_p_19n =
            exempt_delivery_none.or_else(|| if exempt_delivery == 2 { Some(1) } else { None });

        self.invoice.invoice_body.annotations.exemption = TaxExemption {
            exempt_delivery,
            exempt_delivery_none: resolved_p_19n,
            basis_text: basis_text.map(|s| s.to_string()),
            basis_directive: basis_directive.map(|s| s.to_string()),
        };
        self
    }

    pub fn set_new_transport(
        mut self,
        is_new_transport: u8,
        is_new_transport_intra_community: Option<u8>,
    ) -> Self {
        let item = NewTransportItem {
            admission_date: "2023-10-26".to_string(),
            line_number: 1,
            brand: None,
            model: None,
            color: None,
            registration_number: None,
            production_year: None,
            mileage: Some("100".to_string()),
            vin: None,
            hours_sailed: None,
            hours_flown: None,
        };

        self.invoice.invoice_body.annotations.new_transport = NewTransport {
            is_new_transport: Some(is_new_transport),
            is_new_transport_intra_community: is_new_transport_intra_community,
            transport_items: vec![item],
            is_new_transport_none: None,
        };
        self
    }

    pub fn set_margin_procedures(
        mut self,
        tourism_services: Option<u8>,
        used_goods: Option<u8>,
        works_of_art: Option<u8>,
        collector_items: Option<u8>,
        antiques: Option<u8>,
    ) -> Self {
        self.invoice
            .invoice_body
            .annotations
            .margin_procedure_travel = tourism_services;

        let flag_none = if used_goods.is_some()
            || works_of_art.is_some()
            || collector_items.is_some()
            || antiques.is_some()
        {
            None
        } else {
            Some(1)
        };

        self.invoice.invoice_body.annotations.margin_procedures = MarginProcedures {
            used_goods,
            works_of_art,
            collector_items,
            antiques,
            flag_none,
        };
        self
    }

    pub fn add_line(mut self, mut line: InvoiceLine) -> Self {
        let line_number = (self.lines.len() + 1) as u32;
        line.line_number = line_number;
        self.lines.push(line);
        self
    }

    pub fn add_lines(mut self, new_lines: Vec<InvoiceLine>) -> Self {
        for mut line in new_lines.into_iter() {
            let line_number = (self.lines.len() + 1) as u32;
            line.line_number = line_number;
            self.lines.push(line);
        }
        self
    }

    pub fn build(mut self) -> Result<Invoice, String> {
        let mut totals = std::collections::HashMap::new();
        let multiplier = Decimal::new(100, 0);

        for line in &self.lines {
            let net = line.net_value.unwrap_or(Decimal::ZERO);
            let net_cents = (net * multiplier).round_dp(0).to_i128().unwrap_or(0);

            let rate_str = line.tax_rate.as_deref().unwrap_or("zw");
            let rate = TaxRate::from_str(rate_str)
                .map_err(|_| format!("Unknown tax rate: {}", rate_str))?;

            let vat_cents = (net_cents * rate.basis_points() as i128) / 10000;

            let entry = totals.entry(rate).or_insert((0i128, 0i128));
            entry.0 += net_cents;
            entry.1 += vat_cents;
        }

        let get_total = |rate: TaxRate| {
            let (n, v) = totals.get(&rate).cloned().unwrap_or((0, 0));
            (
                Decimal::from_i128_with_scale(n, 2),
                Decimal::from_i128_with_scale(v, 2),
            )
        };

        let (p13_1, p14_1) = get_total(TaxRate::Rate23);
        self.invoice.invoice_body.net_total_basic_rate = p13_1;
        self.invoice.invoice_body.tax_total_basic_rate = p14_1;

        let (p13_2, p14_2) = get_total(TaxRate::Rate8);
        self.invoice.invoice_body.net_total_reduced_rate_1 = p13_2;
        self.invoice.invoice_body.tax_total_reduced_rate_1 = p14_2;

        let (p13_3, p14_3) = get_total(TaxRate::Rate5);
        self.invoice.invoice_body.net_total_reduced_rate_2 = p13_3;
        self.invoice.invoice_body.tax_total_reduced_rate_2 = p14_3;

        let total_net_cents: i128 = totals.values().map(|(n, _)| *n).sum();
        let total_vat_cents: i128 = totals.values().map(|(_, v)| *v).sum();

        self.invoice.invoice_body.total_gross =
            Decimal::from_i128_with_scale(total_net_cents + total_vat_cents, 2);

        self.invoice.invoice_body.lines = self.lines;

        Ok(self.invoice)
    }
}
