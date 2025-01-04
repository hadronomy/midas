#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "camelCase"))]
pub struct Config {
    pub(crate) name: String,
    pub(crate) version: String,
    pub(crate) description: String,
}
