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
  re: Regex,
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
    Self {
      options: my_options,
      re: Regex::new(r"\$\{(\w+)\}\$").unwrap(),
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
    // only handle html file
    if ModuleType::Html == param.module_type && self.options.template == param.resolved_path {
      println!("{:?}", param.resolved_path);

      let content = param.content.clone();
      let mut result = String::new();
      for capture in self.re.captures_iter(&content) {
        let captured_var = capture.get(1).unwrap().as_str();
        let custom_data = &self.options.data;

        // If the user configures the data, it will be replaced.
        if custom_data.contains_key(captured_var) {
          if let Some(replacement) = custom_data.get(captured_var) {
            let trim_quote_str = replacement.as_str().unwrap().trim_matches('"');
            result = self.re.replace(&content, trim_quote_str).to_string();
          }
        }
      }

      return Ok(Some(PluginTransformHookResult {
        content: result,
        module_type: Some(param.module_type.clone()),
        ..Default::default()
      }));
    }

    Ok(None)
  }
}
