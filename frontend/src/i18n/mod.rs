use crate::types::Language;
use shared_frontend::locale::{detect_browser_locale, set_saved_locale};

mod de;
mod en;
mod es;
mod fr;
mod ja;
mod pt;
mod ru;
mod zh;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq)]
pub struct Translations {
    pub placeholder: &'static str,
    pub lookup: &'static str,
    pub examples: &'static str,
    pub loading: &'static str,
    pub failed_title: &'static str,
    pub success_toast: &'static str,
    pub failed_toast: &'static str,
    pub empty_query_toast: &'static str,
    pub print_tooltip: &'static str,
    pub select_lang: &'static str,
    pub toggle_theme: &'static str,

    // Header descriptions
    pub source_whois: &'static str,
    pub source_ip: &'static str,
    pub source_asn: &'static str,

    // Section headers
    pub domain_info: &'static str,
    pub domain: &'static str,
    pub handle: &'static str,
    pub ip_addresses: &'static str,
    pub ipv4: &'static str,
    pub ipv6: &'static str,
    pub no_ipv4: &'static str,
    pub no_ipv6: &'static str,
    pub domain_status: &'static str,
    pub important_dates: &'static str,
    pub registration: &'static str,
    pub expiration: &'static str,
    pub last_updated: &'static str,
    pub nameservers: &'static str,
    pub registrar_info: &'static str,
    pub registrar: &'static str,

    // IP Specific
    pub ip_info: &'static str,
    pub ip_address: &'static str,
    pub network: &'static str,
    pub version: &'static str,
    pub org: &'static str,
    pub loc_info: &'static str,
    pub city: &'static str,
    pub region: &'static str,
    pub country: &'static str,
    pub postal: &'static str,
    pub continent: &'static str,
    pub timezone: &'static str,
    pub geo_coords: &'static str,
    pub lat: &'static str,
    pub lon: &'static str,
    pub add_info: &'static str,
    pub asn: &'static str,
    pub languages: &'static str,
    pub currency: &'static str,
    pub calling_code: &'static str,

    // ASN Specific
    pub asn_info: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub contact_info: &'static str,
    pub website: &'static str,
    pub email_contacts: &'static str,
    pub abuse_contacts: &'static str,
    pub owner_address: &'static str,
    pub registry_info: &'static str,
    pub rir_name: &'static str,
    pub allocated: &'static str,
    pub traffic_ratio: &'static str,
    pub logout_tooltip: &'static str,
    pub enter_pin: &'static str,
    pub locked_out: &'static str,
    pub invalid_pin: &'static str,
}

pub fn get_translations(lang: Language) -> Translations {
    match lang {
        Language::Chinese => zh::translations(),
        Language::Spanish => es::translations(),
        Language::German => de::translations(),
        Language::Japanese => ja::translations(),
        Language::French => fr::translations(),
        Language::Portuguese => pt::translations(),
        Language::Russian => ru::translations(),
        _ => en::translations(),
    }
}

pub fn get_saved_language() -> Language {
    Language::from_code(&detect_browser_locale())
}

pub fn save_language(lang: Language) {
    set_saved_locale(lang.code());
}
