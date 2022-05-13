use std::fs::File;
use std::net::SocketAddr;

use config::{Config, Environment};
use log::{LevelFilter, set_max_level};
use simplelog::{ColorChoice, ConfigBuilder, LevelPadding, TerminalMode, TermLogger};
use warp::{Filter, path};
use warp::fs::dir;
use warp::reply::json;

use crate::meta::Meta;
use crate::service::Groups;

mod meta;
mod service;

#[derive(Debug, serde::Deserialize, Clone)]
struct HubConfig {
  pub log_level: Option<LogLevel>,
  pub web_root: Option<String>,
  pub addr: Option<String>,
  pub data_dir: Option<String>,
}

#[derive(Debug, serde::Deserialize, Copy, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LogLevel {
  Off,
  Error,
  Warn,
  Info,
  Debug,
  Trace,
}

impl From<LogLevel> for LevelFilter {
  fn from(level: LogLevel) -> Self {
    match level {
      LogLevel::Off => LevelFilter::Off,
      LogLevel::Error => LevelFilter::Error,
      LogLevel::Warn => LevelFilter::Warn,
      LogLevel::Info => LevelFilter::Info,
      LogLevel::Debug => LevelFilter::Debug,
      LogLevel::Trace => LevelFilter::Trace,
    }
  }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  TermLogger::init(
    LevelFilter::Info,
    ConfigBuilder::new()
      .set_level_padding(LevelPadding::Right)
      .build(),
    TerminalMode::Mixed,
    ColorChoice::Auto,
  )?;

  let config = Config::builder()
    .add_source(Environment::with_prefix("HUB"))
    .build()?
    .try_deserialize::<HubConfig>()?;

  if let Some(level) = &config.log_level {
    set_max_level((*level).into());
  }

  let web_root = if let Some(web_root) = config.web_root {
    web_root
  } else {
    "./web/dist".to_string()
  };
  let data_dir = if let Some(data_dir) = config.data_dir {
    data_dir
  } else {
    "./data".to_string()
  };

  let meta: Meta = serde_yaml::from_reader(File::open(format!("{}/meta.yaml", data_dir))?)?;
  let groups: Groups = serde_yaml::from_reader(File::open(format!("{}/services.yaml", data_dir))?)?;

  let route = dir(web_root)
    .or(path!("meta").map(move || Ok(json(&meta.clone()))))
    .or(path!("services").map(move || Ok(json(&groups.clone()))));

  let addr = if let Some(addr) = config.addr {
    addr
  } else {
    "127.0.0.1:3030".to_string()
  };

  warp::serve(route).run(addr.parse::<SocketAddr>()?).await;

  Ok(())
}
