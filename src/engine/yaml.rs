use crate::engine::Engine;
use crate::value::pod::Pod;
use yaml_rust::{Yaml, YamlLoader};

#[derive(PartialEq, Debug)]
pub struct YAML {}

impl Engine for YAML {
    fn new() -> Self {
        return YAML {};
    }

    fn parse(&self, content: &str) -> Pod {
        match YamlLoader::load_from_str(content) {
            Ok(docs) => {
                let mut doc = Pod::Null;
                if docs.len() > 0 {
                    doc = docs[0].clone().into();
                }
                doc
            }
            Err(..) => Pod::Null,
        }
    }
}

impl Into<Pod> for Yaml {
    fn into(self) -> Pod {
        match self {
            Yaml::Real(val) => Pod::Float(val.parse().unwrap_or(0 as f64)),
            Yaml::Integer(val) => Pod::Integer(val),
            Yaml::String(val) => Pod::String(val),
            Yaml::Boolean(val) => Pod::Boolean(val),
            Yaml::Array(val) => {
                let mut pod = Pod::new_array();
                for (index, item) in val.into_iter().enumerate() {
                    pod[index] = item.into();
                }
                pod
            }
            Yaml::Hash(val) => {
                let mut pod = Pod::new_hash();
                for (key, val) in val.into_iter() {
                    pod[key.as_str().unwrap()] = val.into();
                }
                pod
            }
            Yaml::Null => Pod::Null,
            _ => Pod::Null,
        }
    }
}
#[cfg(test)]
mod test {
    use crate::engine::yaml::YAML;
    use crate::entity::ParsedEntityStruct;
    use crate::matter::Matter;
    use serde::Deserialize;

    #[test]
    fn test_matter() {
        let matter: Matter<YAML> = Matter::new();
        let input = r#"---
one: foo
two: bar
three: baz
"#;
        #[derive(Deserialize, PartialEq)]
        struct FrontMatter {
            one: String,
            two: String,
            three: String,
        }
        let data_expected = FrontMatter {
            one: "foo".to_string(),
            two: "bar".to_string(),
            three: "baz".to_string(),
        };
        let result: ParsedEntityStruct<FrontMatter> = matter.matter_struct(input.to_string());
        assert_eq!(true, result.data == data_expected);
    }
}
