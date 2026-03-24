use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CountryCode {
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AX")]
    Ax,
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "AS")]
    As_,
    #[serde(rename = "AD")]
    Ad,
    #[serde(rename = "AO")]
    Ao,
    #[serde(rename = "AI")]
    Ai,
    #[serde(rename = "AQ")]
    Aq,
    #[serde(rename = "AG")]
    Ag,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AW")]
    Aw,
    #[serde(rename = "AU")]
    Au,
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BD")]
    Bd,
    #[serde(rename = "BB")]
    Bb,
    #[serde(rename = "BY")]
    By,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BZ")]
    Bz,
    #[serde(rename = "BJ")]
    Bj,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "BT")]
    Bt,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "BQ")]
    Bq,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BW")]
    Bw,
    #[serde(rename = "BV")]
    Bv,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "BF")]
    Bf,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "KH")]
    Kh,
    #[serde(rename = "CM")]
    Cm,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "CF")]
    Cf,
    #[serde(rename = "TD")]
    Td,
    #[serde(rename = "CL")]
    Cl,
    #[serde(rename = "CN")]
    Cn,
    #[serde(rename = "CX")]
    Cx,
    #[serde(rename = "CC")]
    Cc,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "CG")]
    Cg,
    #[serde(rename = "CD")]
    Cd,
    #[serde(rename = "CK")]
    Ck,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CI")]
    Ci,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "CU")]
    Cu,
    #[serde(rename = "CW")]
    Cw,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "DJ")]
    Dj,
    #[serde(rename = "DM")]
    Dm,
    #[serde(rename = "DO")]
    Do_,
    #[serde(rename = "EC")]
    Ec,
    #[serde(rename = "EG")]
    Eg,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "GQ")]
    Gq,
    #[serde(rename = "ER")]
    Er,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "EL")]
    El,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "FK")]
    Fk,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "GF")]
    Gf,
    #[serde(rename = "PF")]
    Pf,
    #[serde(rename = "TF")]
    Tf,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GM")]
    Gm,
    #[serde(rename = "GE")]
    Ge,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "GH")]
    Gh,
    #[serde(rename = "GI")]
    Gi,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GP")]
    Gp,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "GT")]
    Gt,
    #[serde(rename = "GG")]
    Gg,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GW")]
    Gw,
    #[serde(rename = "GY")]
    Gy,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HM")]
    Hm,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "HN")]
    Hn,
    #[serde(rename = "HK")]
    Hk,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "IS")]
    Is_,
    #[serde(rename = "IN")]
    In_,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IR")]
    Ir,
    #[serde(rename = "IQ")]
    Iq,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IM")]
    Im,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JM")]
    Jm,
    #[serde(rename = "JP")]
    Jp,
    #[serde(rename = "JE")]
    Je,
    #[serde(rename = "JO")]
    Jo,
    #[serde(rename = "KZ")]
    Kz,
    #[serde(rename = "KE")]
    Ke,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "KP")]
    Kp,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LS")]
    Ls,
    #[serde(rename = "LR")]
    Lr,
    #[serde(rename = "LY")]
    Ly,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MW")]
    Mw,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "MV")]
    Mv,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "MQ")]
    Mq,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MU")]
    Mu,
    #[serde(rename = "YT")]
    Yt,
    #[serde(rename = "MX")]
    Mx,
    #[serde(rename = "FM")]
    Fm,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "MC")]
    Mc,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MZ")]
    Mz,
    #[serde(rename = "MM")]
    Mm,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "NP")]
    Np,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "NZ")]
    Nz,
    #[serde(rename = "NI")]
    Ni,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NU")]
    Nu,
    #[serde(rename = "NF")]
    Nf,
    #[serde(rename = "MP")]
    Mp,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "PK")]
    Pk,
    #[serde(rename = "PW")]
    Pw,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "PG")]
    Pg,
    #[serde(rename = "PY")]
    Py,
    #[serde(rename = "PE")]
    Pe,
    #[serde(rename = "PH")]
    Ph,
    #[serde(rename = "PN")]
    Pn,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PR")]
    Pr,
    #[serde(rename = "QA")]
    Qa,
    #[serde(rename = "RE")]
    Re,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "BL")]
    Bl,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "LC")]
    Lc,
    #[serde(rename = "MF")]
    Mf,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "VC")]
    Vc,
    #[serde(rename = "WS")]
    Ws,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "RS")]
    Rs,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SX")]
    Sx,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SB")]
    Sb,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "GS")]
    Gs,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "LK")]
    Lk,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SJ")]
    Sj,
    #[serde(rename = "SZ")]
    Sz,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "SY")]
    Sy,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "TJ")]
    Tj,
    #[serde(rename = "TZ")]
    Tz,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TM")]
    Tm,
    #[serde(rename = "TC")]
    Tc,
    #[serde(rename = "TV")]
    Tv,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UA")]
    Ua,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "GB")]
    Gb,
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "UM")]
    Um,
    #[serde(rename = "UY")]
    Uy,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VU")]
    Vu,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VN")]
    Vn,
    #[serde(rename = "VG")]
    Vg,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "WF")]
    Wf,
    #[serde(rename = "EH")]
    Eh,
    #[serde(rename = "YE")]
    Ye,
    #[serde(rename = "ZM")]
    Zm,
    #[serde(rename = "ZW")]
    Zw,
    #[serde(rename = "XK")]
    Xk,
    #[serde(rename = "XX")]
    Xx,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum CurrCode {
    #[serde(rename = "AED")]
    Aed,
    #[serde(rename = "AFN")]
    Afn,
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "AMD")]
    Amd,
    #[serde(rename = "ANG")]
    Ang,
    #[serde(rename = "AOA")]
    Aoa,
    #[serde(rename = "ARS")]
    Ars,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "AWG")]
    Awg,
    #[serde(rename = "AZN")]
    Azn,
    #[serde(rename = "BAM")]
    Bam,
    #[serde(rename = "BBD")]
    Bbd,
    #[serde(rename = "BDT")]
    Bdt,
    #[serde(rename = "BGN")]
    Bgn,
    #[serde(rename = "BHD")]
    Bhd,
    #[serde(rename = "BIF")]
    Bif,
    #[serde(rename = "BMD")]
    Bmd,
    #[serde(rename = "BND")]
    Bnd,
    #[serde(rename = "BOB")]
    Bob,
    #[serde(rename = "BOV")]
    Bov,
    #[serde(rename = "BRL")]
    Brl,
    #[serde(rename = "BSD")]
    Bsd,
    #[serde(rename = "BTN")]
    Btn,
    #[serde(rename = "BWP")]
    Bwp,
    #[serde(rename = "BYN")]
    Byn,
    #[serde(rename = "BZD")]
    Bzd,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "CDF")]
    Cdf,
    #[serde(rename = "CHE")]
    Che,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "CHW")]
    Chw,
    #[serde(rename = "CLF")]
    Clf,
    #[serde(rename = "CLP")]
    Clp,
    #[serde(rename = "CNY")]
    Cny,
    #[serde(rename = "COP")]
    Cop,
    #[serde(rename = "COU")]
    Cou,
    #[serde(rename = "CRC")]
    Crc,
    #[serde(rename = "CUC")]
    Cuc,
    #[serde(rename = "CUP")]
    Cup,
    #[serde(rename = "CVE")]
    Cve,
    #[serde(rename = "CZK")]
    Czk,
    #[serde(rename = "DJF")]
    Djf,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "DOP")]
    Dop,
    #[serde(rename = "DZD")]
    Dzd,
    #[serde(rename = "EGP")]
    Egp,
    #[serde(rename = "ERN")]
    Ern,
    #[serde(rename = "ETB")]
    Etb,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "FJD")]
    Fjd,
    #[serde(rename = "FKP")]
    Fkp,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "GEL")]
    Gel,
    #[serde(rename = "GHS")]
    Ghs,
    #[serde(rename = "GIP")]
    Gip,
    #[serde(rename = "GMD")]
    Gmd,
    #[serde(rename = "GNF")]
    Gnf,
    #[serde(rename = "GTQ")]
    Gtq,
    #[serde(rename = "GYD")]
    Gyd,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "HNL")]
    Hnl,
    #[serde(rename = "HRK")]
    Hrk,
    #[serde(rename = "HTG")]
    Htg,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "IDR")]
    Idr,
    #[serde(rename = "ILS")]
    Ils,
    #[serde(rename = "INR")]
    Inr,
    #[serde(rename = "IQD")]
    Iqd,
    #[serde(rename = "IRR")]
    Irr,
    #[serde(rename = "ISK")]
    Isk,
    #[serde(rename = "JMD")]
    Jmd,
    #[serde(rename = "JOD")]
    Jod,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "KES")]
    Kes,
    #[serde(rename = "KGS")]
    Kgs,
    #[serde(rename = "KHR")]
    Khr,
    #[serde(rename = "KMF")]
    Kmf,
    #[serde(rename = "KPW")]
    Kpw,
    #[serde(rename = "KRW")]
    Krw,
    #[serde(rename = "KWD")]
    Kwd,
    #[serde(rename = "KYD")]
    Kyd,
    #[serde(rename = "KZT")]
    Kzt,
    #[serde(rename = "LAK")]
    Lak,
    #[serde(rename = "LBP")]
    Lbp,
    #[serde(rename = "LKR")]
    Lkr,
    #[serde(rename = "LRD")]
    Lrd,
    #[serde(rename = "LSL")]
    Lsl,
    #[serde(rename = "LYD")]
    Lyd,
    #[serde(rename = "MAD")]
    Mad,
    #[serde(rename = "MDL")]
    Mdl,
    #[serde(rename = "MGA")]
    Mga,
    #[serde(rename = "MKD")]
    Mkd,
    #[serde(rename = "MMK")]
    Mmk,
    #[serde(rename = "MNT")]
    Mnt,
    #[serde(rename = "MOP")]
    Mop,
    #[serde(rename = "MRU")]
    Mru,
    #[serde(rename = "MUR")]
    Mur,
    #[serde(rename = "MVR")]
    Mvr,
    #[serde(rename = "MWK")]
    Mwk,
    #[serde(rename = "MXN")]
    Mxn,
    #[serde(rename = "MXV")]
    Mxv,
    #[serde(rename = "MYR")]
    Myr,
    #[serde(rename = "MZN")]
    Mzn,
    #[serde(rename = "NAD")]
    Nad,
    #[serde(rename = "NGN")]
    Ngn,
    #[serde(rename = "NIO")]
    Nio,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "NPR")]
    Npr,
    #[serde(rename = "NZD")]
    Nzd,
    #[serde(rename = "OMR")]
    Omr,
    #[serde(rename = "PAB")]
    Pab,
    #[serde(rename = "PEN")]
    Pen,
    #[serde(rename = "PGK")]
    Pgk,
    #[serde(rename = "PHP")]
    Php,
    #[serde(rename = "PKR")]
    Pkr,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "PYG")]
    Pyg,
    #[serde(rename = "QAR")]
    Qar,
    #[serde(rename = "RON")]
    Ron,
    #[serde(rename = "RSD")]
    Rsd,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "RWF")]
    Rwf,
    #[serde(rename = "SAR")]
    Sar,
    #[serde(rename = "SBD")]
    Sbd,
    #[serde(rename = "SCR")]
    Scr,
    #[serde(rename = "SDG")]
    Sdg,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "SGD")]
    Sgd,
    #[serde(rename = "SHP")]
    Shp,
    #[serde(rename = "SLL")]
    Sll,
    #[serde(rename = "SOS")]
    Sos,
    #[serde(rename = "SRD")]
    Srd,
    #[serde(rename = "SSP")]
    Ssp,
    #[serde(rename = "STN")]
    Stn,
    #[serde(rename = "SVC")]
    Svc,
    #[serde(rename = "SYP")]
    Syp,
    #[serde(rename = "SZL")]
    Szl,
    #[serde(rename = "THB")]
    Thb,
    #[serde(rename = "TJS")]
    Tjs,
    #[serde(rename = "TMT")]
    Tmt,
    #[serde(rename = "TND")]
    Tnd,
    #[serde(rename = "TOP")]
    Top,
    #[serde(rename = "TRY")]
    Try,
    #[serde(rename = "TTD")]
    Ttd,
    #[serde(rename = "TWD")]
    Twd,
    #[serde(rename = "TZS")]
    Tzs,
    #[serde(rename = "UAH")]
    Uah,
    #[serde(rename = "UGX")]
    Ugx,
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "USN")]
    Usn,
    #[serde(rename = "UYI")]
    Uyi,
    #[serde(rename = "UYU")]
    Uyu,
    #[serde(rename = "UYW")]
    Uyw,
    #[serde(rename = "UZS")]
    Uzs,
    #[serde(rename = "VES")]
    Ves,
    #[serde(rename = "VND")]
    Vnd,
    #[serde(rename = "VUV")]
    Vuv,
    #[serde(rename = "WST")]
    Wst,
    #[serde(rename = "XAF")]
    Xaf,
    #[serde(rename = "XAG")]
    Xag,
    #[serde(rename = "XAU")]
    Xau,
    #[serde(rename = "XBA")]
    Xba,
    #[serde(rename = "XBB")]
    Xbb,
    #[serde(rename = "XBC")]
    Xbc,
    #[serde(rename = "XBD")]
    Xbd,
    #[serde(rename = "XCD")]
    Xcd,
    #[serde(rename = "XDR")]
    Xdr,
    #[serde(rename = "XOF")]
    Xof,
    #[serde(rename = "XPD")]
    Xpd,
    #[serde(rename = "XPF")]
    Xpf,
    #[serde(rename = "XPT")]
    Xpt,
    #[serde(rename = "XSU")]
    Xsu,
    #[serde(rename = "XUA")]
    Xua,
    #[serde(rename = "XXX")]
    Xxx,
    #[serde(rename = "YER")]
    Yer,
    #[serde(rename = "ZAR")]
    Zar,
    #[serde(rename = "ZMW")]
    Zmw,
    #[serde(rename = "ZWL")]
    Zwl,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum MSCountryCode {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "CZ")]
    Cz,
    #[serde(rename = "DK")]
    Dk,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "EL")]
    El,
    #[serde(rename = "GR")]
    Gr,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "SE")]
    Se,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum LanguageCode {
    #[serde(rename = "AA")]
    Aa,
    #[serde(rename = "AB")]
    Ab,
    #[serde(rename = "AF")]
    Af,
    #[serde(rename = "AK")]
    Ak,
    #[serde(rename = "SQ")]
    Sq,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "AN")]
    An,
    #[serde(rename = "HY")]
    Hy,
    #[serde(rename = "AS")]
    As_,
    #[serde(rename = "AV")]
    Av,
    #[serde(rename = "AE")]
    Ae,
    #[serde(rename = "AY")]
    Ay,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "BA")]
    Ba,
    #[serde(rename = "BM")]
    Bm,
    #[serde(rename = "EU")]
    Eu,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "BN")]
    Bn,
    #[serde(rename = "BH")]
    Bh,
    #[serde(rename = "BI")]
    Bi,
    #[serde(rename = "BS")]
    Bs,
    #[serde(rename = "BR")]
    Br,
    #[serde(rename = "BG")]
    Bg,
    #[serde(rename = "MY")]
    My,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CH")]
    Ch,
    #[serde(rename = "CE")]
    Ce,
    #[serde(rename = "ZH")]
    Zh,
    #[serde(rename = "CU")]
    Cu,
    #[serde(rename = "CV")]
    Cv,
    #[serde(rename = "KW")]
    Kw,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CR")]
    Cr,
    #[serde(rename = "CS")]
    Cs,
    #[serde(rename = "DA")]
    Da,
    #[serde(rename = "DV")]
    Dv,
    #[serde(rename = "NL")]
    Nl,
    #[serde(rename = "DZ")]
    Dz,
    #[serde(rename = "EN")]
    En,
    #[serde(rename = "EO")]
    Eo,
    #[serde(rename = "ET")]
    Et,
    #[serde(rename = "EE")]
    Ee,
    #[serde(rename = "FO")]
    Fo,
    #[serde(rename = "FJ")]
    Fj,
    #[serde(rename = "FI")]
    Fi,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "FY")]
    Fy,
    #[serde(rename = "FF")]
    Ff,
    #[serde(rename = "KA")]
    Ka,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "GD")]
    Gd,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "GL")]
    Gl,
    #[serde(rename = "GV")]
    Gv,
    #[serde(rename = "EL")]
    El,
    #[serde(rename = "GN")]
    Gn,
    #[serde(rename = "GU")]
    Gu,
    #[serde(rename = "HT")]
    Ht,
    #[serde(rename = "HA")]
    Ha,
    #[serde(rename = "HE")]
    He,
    #[serde(rename = "HZ")]
    Hz,
    #[serde(rename = "HI")]
    Hi,
    #[serde(rename = "HO")]
    Ho,
    #[serde(rename = "HR")]
    Hr,
    #[serde(rename = "HU")]
    Hu,
    #[serde(rename = "IG")]
    Ig,
    #[serde(rename = "IS")]
    Is_,
    #[serde(rename = "IO")]
    Io,
    #[serde(rename = "II")]
    Ii,
    #[serde(rename = "IU")]
    Iu,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "IA")]
    Ia,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IK")]
    Ik,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "JV")]
    Jv,
    #[serde(rename = "JA")]
    Ja,
    #[serde(rename = "KL")]
    Kl,
    #[serde(rename = "KN")]
    Kn,
    #[serde(rename = "KS")]
    Ks,
    #[serde(rename = "KR")]
    Kr,
    #[serde(rename = "KK")]
    Kk,
    #[serde(rename = "KM")]
    Km,
    #[serde(rename = "KI")]
    Ki,
    #[serde(rename = "RW")]
    Rw,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "KV")]
    Kv,
    #[serde(rename = "KG")]
    Kg,
    #[serde(rename = "KO")]
    Ko,
    #[serde(rename = "KJ")]
    Kj,
    #[serde(rename = "KU")]
    Ku,
    #[serde(rename = "LO")]
    Lo,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "LV")]
    Lv,
    #[serde(rename = "LI")]
    Li,
    #[serde(rename = "LN")]
    Ln,
    #[serde(rename = "LT")]
    Lt,
    #[serde(rename = "LB")]
    Lb,
    #[serde(rename = "LU")]
    Lu,
    #[serde(rename = "LG")]
    Lg,
    #[serde(rename = "MK")]
    Mk,
    #[serde(rename = "MH")]
    Mh,
    #[serde(rename = "ML")]
    Ml,
    #[serde(rename = "MI")]
    Mi,
    #[serde(rename = "MR")]
    Mr,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MG")]
    Mg,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "NA")]
    Na,
    #[serde(rename = "NV")]
    Nv,
    #[serde(rename = "NR")]
    Nr,
    #[serde(rename = "ND")]
    Nd,
    #[serde(rename = "NG")]
    Ng,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NN")]
    Nn,
    #[serde(rename = "NB")]
    Nb,
    #[serde(rename = "NO")]
    No,
    #[serde(rename = "NY")]
    Ny,
    #[serde(rename = "OC")]
    Oc,
    #[serde(rename = "OJ")]
    Oj,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "OM")]
    Om,
    #[serde(rename = "OS")]
    Os,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "FA")]
    Fa,
    #[serde(rename = "PI")]
    Pi,
    #[serde(rename = "PL")]
    Pl,
    #[serde(rename = "PT")]
    Pt,
    #[serde(rename = "PS")]
    Ps,
    #[serde(rename = "QU")]
    Qu,
    #[serde(rename = "RM")]
    Rm,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RN")]
    Rn,
    #[serde(rename = "RU")]
    Ru,
    #[serde(rename = "SG")]
    Sg,
    #[serde(rename = "SA")]
    Sa,
    #[serde(rename = "SI")]
    Si,
    #[serde(rename = "SK")]
    Sk,
    #[serde(rename = "SL")]
    Sl,
    #[serde(rename = "SE")]
    Se,
    #[serde(rename = "SM")]
    Sm,
    #[serde(rename = "SN")]
    Sn,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "SO")]
    So,
    #[serde(rename = "ST")]
    St,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SR")]
    Sr,
    #[serde(rename = "SS")]
    Ss,
    #[serde(rename = "SU")]
    Su,
    #[serde(rename = "SW")]
    Sw,
    #[serde(rename = "SV")]
    Sv,
    #[serde(rename = "TY")]
    Ty,
    #[serde(rename = "TA")]
    Ta,
    #[serde(rename = "TT")]
    Tt,
    #[serde(rename = "TE")]
    Te,
    #[serde(rename = "TG")]
    Tg,
    #[serde(rename = "TL")]
    Tl,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "BO")]
    Bo,
    #[serde(rename = "TI")]
    Ti,
    #[serde(rename = "TO")]
    To,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TS")]
    Ts,
    #[serde(rename = "TK")]
    Tk,
    #[serde(rename = "TR")]
    Tr,
    #[serde(rename = "TW")]
    Tw,
    #[serde(rename = "UG")]
    Ug,
    #[serde(rename = "UK")]
    Uk,
    #[serde(rename = "UR")]
    Ur,
    #[serde(rename = "UZ")]
    Uz,
    #[serde(rename = "VE")]
    Ve,
    #[serde(rename = "VI")]
    Vi,
    #[serde(rename = "VO")]
    Vo,
    #[serde(rename = "CY")]
    Cy,
    #[serde(rename = "WA")]
    Wa,
    #[serde(rename = "WO")]
    Wo,
    #[serde(rename = "XH")]
    Xh,
    #[serde(rename = "YI")]
    Yi,
    #[serde(rename = "YO")]
    Yo,
    #[serde(rename = "ZA")]
    Za,
    #[serde(rename = "ZU")]
    Zu,
}
