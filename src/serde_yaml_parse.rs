extern crate serde_yaml;

use serde_yaml::Value;

#[derive(Deserialize, Debug)]
pub struct PresetConfig {
    name: String,
    options: Value
}

#[derive(Deserialize, Debug)]
pub struct ProgramConfig {
    presets: Vec<PresetConfig>
}

#[derive(Debug, Default, Eq, PartialEq)]
pub struct M2PresetOptions {
    url: String,
    require_path: String,
}

impl From<Value> for M2PresetOptions {
    fn from(v: Value) -> Self {
        let out = M2PresetOptions{..Default::default()};
        out.add_from_value(v)
    }
}

impl M2PresetOptions {
    pub fn add_from_value(mut self, value: Value) -> M2PresetOptions {
        if let Value::Mapping(m) = value {
            for (key, value) in m {
                if let (Value::String(key), Value::String(value)) = (key, value) {
                    match key.as_str() {
                        "url" => self.url = value,
                        "require_path" => self.require_path = value,
                        _ => { /* not supported */ }
                    }
                }
            }
        }
        self
    }
}

pub fn main() {
    let input = r#"
        presets:
          - name: static
            options:
          - name: m2
            options:
              url: https://example.com
              require_path: "/js/some/path"
    "#;

    let p: Result<ProgramConfig, _> = serde_yaml::from_str(&input);

    let nested_opts: Option<M2PresetOptions> = match p {
        Ok(pc) => {
            if let Some(m_2) = pc.presets.iter().find(|pre| pre.name == "m2") {
                let output = m_2.options.clone();
                let preset_opts: M2PresetOptions = output.into();
                Some(preset_opts)
            } else {
                None
            }
        },
        Err(e) => None
    };

    assert_eq!(nested_opts.unwrap(), M2PresetOptions{
        url: String::from("https://example.com"),
        require_path: String::from("/js/some/path"),
    });
}
