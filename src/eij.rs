use std::collections::BTreeMap;

use iced::{
    Element,
    widget::{Column, row, text, text_input},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Attribute {
    attributes: BTreeMap<String, AttributeType>,
}

impl Attribute {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn clear_attributes(&mut self) {
        self.attributes.clear();
    }

    fn add_attribute(&mut self, name: impl Into<String>, attr_type: AttributeType) {
        self.attributes.insert(name.into(), attr_type);
    }

    pub fn get_attribute_type(&self, name: &str) -> Option<&AttributeType> {
        self.attributes.get(name)
    }

    pub fn remove_attribute(&mut self, name: &str) {
        self.attributes.remove(name);
    }

    pub fn add_bool(&mut self, name: impl Into<String>) {
        self.add_attribute(name, AttributeType::Bool);
    }

    pub fn add_int(&mut self, name: impl Into<String>) {
        self.add_attribute(name, AttributeType::Int);
    }

    pub fn add_float(&mut self, name: impl Into<String>) {
        self.add_attribute(name, AttributeType::Float);
    }

    pub fn add_text(&mut self, name: impl Into<String>) {
        self.add_attribute(name, AttributeType::Text);
    }

    pub fn add_list(&mut self, name: impl Into<String>, item_type: AttributeType) {
        self.add_attribute(name, AttributeType::List(Box::new(item_type)));
    }

    pub fn add_record(&mut self, name: impl Into<String>, record: Attribute) {
        self.attributes
            .insert(name.into(), AttributeType::Record(record.attributes));
    }

    pub fn add_record_field(
        &mut self,
        record_name: &str,
        field_name: impl Into<String>,
        field_type: AttributeType,
    ) {
        if let Some(AttributeType::Record(fields)) = self.attributes.get_mut(record_name) {
            fields.insert(field_name.into(), field_type);
        }
    }

    pub fn add_optional(&mut self, name: impl Into<String>, inner_type: AttributeType) {
        self.add_attribute(name, AttributeType::Optional(Box::new(inner_type)));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    attributes: BTreeMap<String, AttributeValue>,
}

impl Player {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: BTreeMap::new(),
        }
    }

    pub fn set_attribute(&mut self, name: impl Into<String>, value: AttributeValue) {
        self.attributes.insert(name.into(), value);
    }

    pub fn get_attribute(&self, name: &str) -> Option<&AttributeValue> {
        self.attributes.get(name)
    }

    pub fn remove_attribute(&mut self, name: &str) {
        self.attributes.remove(name);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttributeType {
    Bool,
    Int,
    Float,
    Text,
    List(Box<AttributeType>),
    Optional(Box<AttributeType>),
    Record(BTreeMap<String, AttributeType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value")]
pub enum AttributeValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    List(Vec<AttributeValue>),
    Optional(Option<Box<AttributeValue>>),
    Record(BTreeMap<String, AttributeValue>),
}

impl AttributeValue {
    pub fn value_as_string(&self) -> String {
        match self {
            AttributeValue::Bool(b) => b.to_string(),
            AttributeValue::Int(i) => i.to_string(),
            AttributeValue::Float(f) => f.to_string(),
            AttributeValue::Text(s) => s.clone(),
            AttributeValue::List(items) => format!(
                "[{}]",
                items
                    .iter()
                    .map(|item| item.value_as_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            AttributeValue::Optional(opt) => match opt {
                Some(inner) => format!("Some({})", inner.value_as_string()),
                None => "None".to_string(),
            },
            AttributeValue::Record(fields) => format!(
                "{{{}}}",
                fields
                    .iter()
                    .map(|(k, v)| format!("{}: {}", k, v.value_as_string()))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AttributeEditMessage {
    UpdateAttribute(String, AttributeValue), // (attribute name, new value as string)
}

#[derive(Debug, Clone)]
pub struct AttributeEditor {
    attribute: Attribute,
    values: BTreeMap<String, AttributeValue>,
}

impl AttributeEditor {
    pub fn new(attribute: Attribute) -> Self {
        Self {
            attribute,
            values: BTreeMap::new(),
        }
    }

    pub fn set_value(&mut self, name: impl Into<String>, value: AttributeValue) {
        self.values.insert(name.into(), value);
    }
}

pub fn attr_view(state: &AttributeEditor) -> Element<'_, AttributeEditMessage> {
    let rows = state.values.iter().map(|(name, value)| {
        row![
            text(name),
            text_input(name, &value.value_as_string()).on_input(move |new_value| {
                let attr_type = state
                    .attribute
                    .get_attribute_type(name)
                    .cloned()
                    .expect("Attribute type should exist");

                let value = match attr_type {
                    AttributeType::Bool => {
                        let parsed = new_value.parse::<bool>();
                        if let Ok(b) = parsed {
                            AttributeValue::Bool(b)
                        } else {
                            return AttributeEditMessage::UpdateAttribute(
                                name.clone(),
                                AttributeValue::Text(new_value),
                            );
                        }
                    }
                    AttributeType::Int => {
                        let parsed = new_value.parse::<i64>();
                        if let Ok(i) = parsed {
                            AttributeValue::Int(i)
                        } else {
                            return AttributeEditMessage::UpdateAttribute(
                                name.clone(),
                                AttributeValue::Text(new_value),
                            );
                        }
                    }
                    AttributeType::Float => {
                        let parsed = new_value.parse::<f64>();
                        if let Ok(f) = parsed {
                            AttributeValue::Float(f)
                        } else {
                            return AttributeEditMessage::UpdateAttribute(
                                name.clone(),
                                AttributeValue::Text(new_value),
                            );
                        }
                    }
                    AttributeType::Text => AttributeValue::Text(new_value),
                    AttributeType::List(_) => {
                        // For simplicity, we won't handle list editing in this example
                        return AttributeEditMessage::UpdateAttribute(
                            name.clone(),
                            AttributeValue::Text(new_value),
                        );
                    }
                    AttributeType::Optional(_inner_type) => {
                        // For simplicity, we won't handle optional editing in this example
                        return AttributeEditMessage::UpdateAttribute(
                            name.clone(),
                            AttributeValue::Text(new_value),
                        );
                    }
                    AttributeType::Record(_) => {
                        // For simplicity, we won't handle record editing in this example
                        return AttributeEditMessage::UpdateAttribute(
                            name.clone(),
                            AttributeValue::Text(new_value),
                        );
                    }
                };

                AttributeEditMessage::UpdateAttribute(name.clone(), value)
            })
        ]
        .into()
    });

    Column::with_children(rows).into()
}
