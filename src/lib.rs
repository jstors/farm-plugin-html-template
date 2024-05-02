#![deny(clippy::all)]

use farmfe_core::{
  config::Config,
  module::ModuleType,
  plugin::{Plugin, PluginTransformHookResult},
  regex::Regex,
  serde_json::{self, Value},
};
use farmfe_macro_plugin::farm_plugin;
use serde;
use std::collections::HashMap;

#[farm_plugin]
pub struct FarmPluginHtmlTemplate {
  options: Options,
}

#[derive(serde::Deserialize, Debug)]
pub struct Options {
  template: String,
  data: HashMap<String, Value>,
}

impl FarmPluginHtmlTemplate {
  fn new(config: &Config, options: String) -> Self {
    let my_options: Options = serde_json::from_str(&options)
      .expect(&format!("failed to parse template options: {:?}", options));
    println!("传入的参数={:?}", my_options);
    Self {
      options: my_options,
    }
  }
}

impl Plugin for FarmPluginHtmlTemplate {
  fn name(&self) -> &str {
    "FarmPluginHtmlTemplate"
  }

  fn transform(
    &self,
    param: &farmfe_core::plugin::PluginTransformHookParam,
    context: &std::sync::Arc<farmfe_core::context::CompilationContext>,
  ) -> farmfe_core::error::Result<Option<farmfe_core::plugin::PluginTransformHookResult>> {
    let re = Regex::new(r"\$\{(\w+)\}\$").unwrap();
    // only handle html file
    if ModuleType::Html == param.module_type {
      for capture in re.captures_iter(&param.content) {
        let matched_str = capture.get(0).unwrap().as_str();
        let captured_var = capture.get(1).unwrap().as_str();
        println!("Matched: {}, Captured: {}", matched_str, captured_var);
      }
    }

    Ok(Some(PluginTransformHookResult {
      content: param.content.clone(),
      module_type: Some(param.module_type.clone()),
      ..Default::default()
    }))
  }
}
