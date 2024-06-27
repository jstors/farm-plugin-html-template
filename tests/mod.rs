use std::{collections::HashMap, sync::Arc};

use farm_plugin_html_template::FarmPluginHtmlTemplate;
use farmfe_core::{
  config::Config,
  context::CompilationContext,
  module::ModuleType,
  plugin::{Plugin, PluginTransformHookParam},
};
use farmfe_testing_helpers::fixture;
use farmfe_toolkit::fs::read_file_utf8;

#[test]
fn test_replace_html() {
  fixture!("tests/fixtures/index.html", |file, _cwd| {
    let resolve_path = file.to_string_lossy().to_string();
    let config = Config {
      input: HashMap::from([("index".to_string(), resolve_path.clone())]),
      ..Default::default()
    };

    let plugin = Arc::new(FarmPluginHtmlTemplate::new(
      &config,
      r#"
      {
        "template": "/Users/liuqh/lqh/farm-plugin-html-template/tests/fixtures/index.html",
        "data": {
        "title":"farm",
        "arr":[1,2,3]
        }
      }
    "#
      .to_string(),
    ));
    let context = CompilationContext::new(config, vec![plugin.clone()]).unwrap();
    let content = read_file_utf8(&resolve_path).unwrap();

    let transformed = plugin
      .transform(
        &PluginTransformHookParam {
          content,
          module_id: resolve_path.clone(),
          module_type: ModuleType::Html,
          resolved_path: &resolve_path,
          query: vec![],
          meta: HashMap::new(),
          source_map_chain: vec![],
        },
        &Arc::new(context),
      )
      .unwrap()
      .unwrap();

    println!("result: {:?}", transformed);
  });
}
