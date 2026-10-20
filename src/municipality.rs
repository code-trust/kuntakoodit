/// A Finnish municipality (kunta).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Municipality {
    /// Three-digit municipality code, e.g. `"091"` for Helsinki.
    pub code: &'static str,
    /// Finnish name, e.g. `"Helsinki"`.
    pub name_fi: &'static str,
    /// Swedish name, e.g. `"Helsingfors"`. Same as the Finnish name when the
    /// municipality has no separate Swedish name.
    pub name_sv: &'static str,
}
