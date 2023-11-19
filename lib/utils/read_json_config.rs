use serde::{Deserialize, Serialize};
use tracing::trace;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub dependencies: Vec<Dependencies>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Dependencies {
    pub title: String,
    pub default: Vec<String>,
    pub list: Vec<DependenciesItem>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DependenciesItem {
    pub name: String,
    pub features: Option<Vec<String>>,
}

pub(crate) fn read_json_config() -> Config {
    let config_file = include_str!("../../config.json");
    serde_json::from_str(config_file).unwrap()
}

impl Dependencies {
    pub fn multi_select(&self) -> Vec<&DependenciesItem> {
        let prompt = format!("请选择要安装的{}依赖:", self.title);
        let options = self
            .list
            .iter()
            .map(|item| item.name.clone())
            .collect::<Vec<_>>();
        let defaults = options
            .iter()
            .map(|item| self.default.contains(item))
            .collect::<Vec<_>>();
        trace!("{:?},{:?}", options, defaults);
        
        let selected =
            super::dialogue::multi_select_return_usize(&prompt.as_str(), &options, &defaults);
        let selected_items = selected
            .iter()
            .map(|i| self.list.get(*i).unwrap())
            .collect::<Vec<_>>();
        trace!("{:?}", selected_items);
        selected_items
    }
}
