use crate::Municipality;
use crate::normalize::normalize;

include!(concat!(env!("OUT_DIR"), "/municipalities.rs"));

/// All municipalities in the classification, ordered by code.
pub fn all() -> &'static [Municipality] {
    &MUNICIPALITIES
}

/// Finds a municipality by its Finnish or Swedish name.
///
/// Matching ignores case and surrounding whitespace.
pub fn by_name(name: &str) -> Option<&'static Municipality> {
    let name = normalize(name);
    NAMES
        .binary_search_by(|(key, _)| (*key).cmp(name.as_str()))
        .ok()
        .map(|index| NAMES[index].1)
}

/// Finds a municipality by its three-digit code, e.g. `"091"`.
pub fn by_code(code: &str) -> Option<&'static Municipality> {
    let code = code.trim();
    MUNICIPALITIES
        .binary_search_by(|municipality| municipality.code.cmp(code))
        .ok()
        .map(|index| &MUNICIPALITIES[index])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kuntakoodi(name: &str) -> Option<&'static str> {
        by_name(name).map(|municipality| municipality.code)
    }

    #[test]
    fn finds_by_finnish_name() {
        assert_eq!(kuntakoodi("Helsinki"), Some("091"));
        assert_eq!(kuntakoodi("Espoo"), Some("049"));
        assert_eq!(kuntakoodi("Akaa"), Some("020"));
    }

    #[test]
    fn finds_by_swedish_name() {
        assert_eq!(kuntakoodi("Helsingfors"), Some("091"));
        assert_eq!(kuntakoodi("Esbo"), Some("049"));
    }

    #[test]
    fn ignores_case_and_whitespace() {
        assert_eq!(kuntakoodi("  hELSINKI "), Some("091"));
        assert_eq!(kuntakoodi("ÄÄNEKOSKI"), Some("992"));
    }

    #[test]
    fn finds_parts_of_bilingual_name() {
        let code = kuntakoodi("Maarianhamina - Mariehamn");
        assert!(code.is_some());
        assert_eq!(kuntakoodi("Maarianhamina"), code);
        assert_eq!(kuntakoodi("Mariehamn"), code);
    }

    #[test]
    fn unknown_name_is_none() {
        assert_eq!(kuntakoodi("Atlantis"), None);
        assert_eq!(kuntakoodi(""), None);
    }

    #[test]
    fn finds_by_code() {
        let helsinki = by_code("091").expect("Helsinki exists");
        assert_eq!(helsinki.name_fi, "Helsinki");
        assert_eq!(helsinki.name_sv, "Helsingfors");
        assert_eq!(by_code("000"), None);
    }

    #[test]
    fn every_name_resolves_to_its_own_municipality() {
        for municipality in all() {
            for name in [municipality.name_fi, municipality.name_sv] {
                assert_eq!(
                    kuntakoodi(name),
                    Some(municipality.code),
                    "{name} resolves to the wrong municipality"
                );
            }
        }
    }

    #[test]
    fn data_is_consistent() {
        assert!(all().len() > 250, "suspiciously few municipalities");
        assert!(
            all()
                .iter()
                .all(|m| m.code.len() == 3 && m.code.bytes().all(|b| b.is_ascii_digit()))
        );
    }
}
