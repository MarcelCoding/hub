pub(crate) type Groups = Vec<Group>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Group {
  name: String,
  services: Services,
}

pub(crate) type Services = Vec<Service>;

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub(crate) struct Service {
  id: String,
  url: String,
  description: String,
}
