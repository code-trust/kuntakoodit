use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::Write as _;
use std::path::Path;
use std::{
    env,
    fs,
};

use serde::Deserialize;

#[path = "src/normalize.rs"]
mod normalize;

use normalize::normalize;

const KUNNAT_FI: &str = "data/kunnat_fi.json";
const KUNNAT_SV: &str = "data/kunnat_sv.json";

/// Bilingual names are written as `"Maarianhamina - Mariehamn"`; each part is
/// also indexed on its own.
const BILINGUAL_SEPARATOR: &str = " - ";

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ClassificationItem {
    code: String,
    classification_item_names: Vec<ItemName>,
}

#[derive(Deserialize)]
struct ItemName {
    name: String,
}

struct Municipality {
    code: String,
    name_fi: String,
    name_sv: String,
}

fn read_names(path: &str) -> Result<BTreeMap<String, String>, Box<dyn Error>> {
    println!("cargo:rerun-if-changed={path}");
    let items: Vec<ClassificationItem> =
        serde_json::from_str(&fs::read_to_string(path)?).map_err(|err| format!("{path}: {err}"))?;

    let mut names = BTreeMap::new();
    for item in items {
        let name = item
            .classification_item_names
            .into_iter()
            .next()
            .ok_or_else(|| format!("{path}: municipality {} has no name", item.code))?
            .name;
        if names.insert(item.code.clone(), name).is_some() {
            return Err(format!("{path}: duplicate municipality code {}", item.code).into());
        }
    }
    Ok(names)
}

fn main() -> Result<(), Box<dyn Error>> {
    let fi = read_names(KUNNAT_FI)?;
    let mut sv = read_names(KUNNAT_SV)?;

    if fi.is_empty() {
        return Err(format!("{KUNNAT_FI}: no municipalities").into());
    }

    // BTreeMap keeps municipalities ordered by code for binary search.
    let mut municipalities = Vec::with_capacity(fi.len());
    for (code, name_fi) in fi {
        let name_sv = sv
            .remove(&code)
            .ok_or_else(|| format!("{KUNNAT_SV}: municipality {code} is missing"))?;
        municipalities.push(Municipality {
            code,
            name_fi,
            name_sv,
        });
    }
    if let Some(code) = sv.keys().next() {
        return Err(format!("{KUNNAT_FI}: municipality {code} is missing").into());
    }

    let mut names: BTreeMap<String, usize> = BTreeMap::new();
    for (index, municipality) in municipalities.iter().enumerate() {
        for name in [&municipality.name_fi, &municipality.name_sv] {
            for key in std::iter::once(name.as_str()).chain(name.split(BILINGUAL_SEPARATOR)) {
                let existing = *names.entry(normalize(key)).or_insert(index);
                if existing != index {
                    return Err(format!(
                        "name {key:?} matches both {} and {}",
                        municipalities[existing].code, municipality.code
                    )
                    .into());
                }
            }
        }
    }

    let mut out = String::new();
    writeln!(
        out,
        "pub(crate) static MUNICIPALITIES: [Municipality; {}] = [",
        municipalities.len()
    )?;
    for m in &municipalities {
        writeln!(
            out,
            "    Municipality {{ code: {:?}, name_fi: {:?}, name_sv: {:?} }},",
            m.code, m.name_fi, m.name_sv
        )?;
    }
    writeln!(out, "];")?;
    writeln!(
        out,
        "pub(crate) static NAMES: [(&str, &Municipality); {}] = [",
        names.len()
    )?;
    for (name, index) in &names {
        writeln!(out, "    ({name:?}, &MUNICIPALITIES[{index}]),")?;
    }
    writeln!(out, "];")?;

    fs::write(
        Path::new(&env::var("OUT_DIR")?).join("municipalities.rs"),
        out,
    )?;
    Ok(())
}
