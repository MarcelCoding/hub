#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Meta {
  url: String,
  contact: String,
  header: Header,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Header {
  name: String,
  description: String,
}
