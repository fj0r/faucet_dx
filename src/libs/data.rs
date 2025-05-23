use dioxus::prelude::*;
use itertools::{
    EitherOrBoth::{Both, Left},
    Itertools,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value, Map};
use time::OffsetDateTime;
use time::serde::rfc3339;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Created(#[serde(with = "rfc3339")]OffsetDateTime);

impl Default for Created {
    fn default() -> Self {
        Self (OffsetDateTime::now_utc())
    }
}

type Session = String;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Message {
    pub sender: Session,
    pub created: Option<Created>,
    pub content: Content,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Outflow {
    pub event: String,
    pub id: Option<String>,
    pub data: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(tag = "action")]
pub enum Content {
    #[allow(non_camel_case_types)]
    create(Influx),

    #[allow(non_camel_case_types)]
    tmpl(InfluxTmpl),

    #[allow(non_camel_case_types)]
    fill(InfluxFill),

    #[allow(non_camel_case_types)]
    merge(Influx),

    #[allow(non_camel_case_types)]
    join(Influx),

    #[allow(non_camel_case_types)]
    #[default]
    empty,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfluxTmpl {
    pub name: String,
    pub data: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfluxFill{
    pub name: String,
    pub data: Value,
}

#[derive(Debug, Clone, Props, PartialEq, Serialize, Deserialize, Default)]
pub struct Influx {
    pub event: String,
    pub data: Layout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bind {
    Event {
        event: String,
        #[serde(rename = "type")]
        // number, bool, [text]
        kind: Option<String>,
        // Abandon
        local: Option<String>
    },
    Field {
        field: String,
        #[serde(rename = "type")]
        // number, bool, [text]
        kind: Option<String>,
        payload: Option<Value>,
        #[allow(dead_code)]
        #[serde(skip)]
        signal: Option<Signal<Value>>
    },
    Submit {
        submit: bool,
        #[allow(dead_code)]
        #[serde(skip)]
        signal: Option<Signal<Value>>
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Attrs {
    pub class: Option<String>,
    // for selector
    pub kind: Option<String>,
    pub horizontal: Option<bool>,
    #[serde(flatten)]
    pub settings: Option<Settings>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TextFold {
    #[allow(non_camel_case_types)]
    begin,
    #[allow(non_camel_case_types)]
    end,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Settings {
    Container(Container),
    Rack {
        scroll: bool
    },
    Text {
        format: String,
        fold: Option<TextFold>
    },
    Item {
        selector: String
    },
    Button {
        oneshot: bool
    },
    Form {
        instant: bool
    },
    Image {
        desc: String,
        #[serde(default)]
        thumb: bool,
        width: Option<String>,
        height: Option<String>,
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Container {
    #[allow(non_camel_case_types)]
    table(Table),
    #[allow(non_camel_case_types)]
    grid(Map<String, Value>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Table {
    pub column: usize,
    #[serde(default)]
    pub header: bool
}

#[derive(Debug, Clone, PartialEq, Props, Serialize, Deserialize, Default)]
pub struct Layout {
    #[serde(rename = "type")]
    pub kind: String,
    pub id: Option<String>,
    pub attrs: Option<Attrs>,
    pub data: Option<Bind>,
    pub value: Option<Value>,
    pub item: Option<Vec<Layout>>,
    pub children: Option<Vec<Layout>>,
}

impl Layout {
    #[allow(dead_code)]
    pub fn new(kind: impl AsRef<str>) -> Self {
        Layout {
            kind: kind.as_ref().to_string(),
            ..Layout::default()
        }
    }
    pub fn cmp_id(&self, other: &Self) -> bool {
        if let Some(id) = &self.id {
            if let Some(oid) = &other.id {
                if id == oid {
                    return true;
                }
            }
        }
        false
    }
    pub fn join(&mut self, rhs: Self) {
        let value = match &self.value {
            Some(x) => {
                if let Some(r) = rhs.value {
                    let y = match (x, &r) {
                        (Value::Number(x), Value::Number(r)) => {
                            json!(x.as_f64().unwrap() + r.as_f64().unwrap())
                        }
                        (Value::Bool(x), Value::Bool(r)) => {
                            json!(*x && *r)
                        }
                        (Value::String(x), Value::String(r)) => {
                            let mut x = x.clone();
                            x.push_str(r);
                            json!(x)
                        }
                        _ => r.clone(),
                    };
                    Some(y)
                } else {
                    Some(x.clone())
                }
            }
            None => rhs.value,
        };
        self.value = value;
        if let Some(children) = &mut self.children {
            if let Some(rchildren) = rhs.children {
                for x in children.iter_mut().zip_longest(rchildren.into_iter()) {
                    match x {
                        Both(l, r) => {
                            l.join(r);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Empty;
