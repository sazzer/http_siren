#![allow(clippy::needless_pass_by_value)]

use serde::Serialize;
use serde_json::Value;

/// Representation of a Siren document.
#[derive(Debug, Serialize)]
#[must_use]
pub struct Document<T>
where
    T: Serialize,
{
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class:      Vec<String>,
    pub properties: T,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entities:   Vec<Entity>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links:      Vec<Link>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub actions:    Vec<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title:      Option<String>,
}

/// Representation of an embedded entity. Either an embedded link or a full representation.
#[derive(Debug, Serialize)]
#[serde(untagged)]
#[must_use]
pub enum Entity {
    Link(Link),
    Representation(EmbeddedRepresentation),
}

/// Body of an embedded representation.
#[derive(Debug, Serialize)]
#[must_use]
pub struct EmbeddedRepresentation {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    rel:        Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    class:      Vec<String>,
    properties: Value,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    entities:   Vec<Entity>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    links:      Vec<Link>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    actions:    Vec<Action>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title:      Option<String>,
}

/// Representation of a link. Either as a standard link or an embedded entity.
#[derive(Debug, Serialize)]
#[must_use]
pub struct Link {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rel:        Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class:      Vec<String>,
    pub href:       String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title:      Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
}

/// Representation of an action.
#[derive(Debug, Serialize)]
#[must_use]
pub struct Action {
    pub name:       String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class:      Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method:     Option<String>,
    pub href:       String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title:      Option<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub fields:     Vec<Field>,
}

/// Representation of a field within an action.
#[derive(Debug, Serialize)]
#[must_use]
pub struct Field {
    pub name:       String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub class:      Vec<String>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value:      Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title:      Option<String>,
}

impl<T> Document<T>
where
    T: Serialize,
{
    /// Create a new Document.
    ///
    /// # Parameters
    /// - `payload` - The payload of the document.
    pub fn new(payload: T) -> Self {
        Self {
            class:      vec![],
            properties: payload,
            entities:   vec![],
            links:      vec![],
            actions:    vec![],
            title:      None,
        }
    }

    /// Apply a lambda to specify additional details on the document.
    ///
    /// # Parameters
    /// - `f` - The lambda function to execute on the document.
    pub fn with<F>(self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        f(self)
    }

    /// Specify a class for the document.
    ///
    /// # Parameters
    /// - `value` - The class to specify.
    pub fn with_class<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.class.push(value.to_string());

        self
    }

    /// Specify an embedded link for the document.
    ///
    /// # Parameters
    /// - `value` - The embedded link to specify.
    pub fn with_embedded_link<V>(mut self, value: V) -> Self
    where
        V: Into<Link>,
    {
        self.entities.push(Entity::Link(value.into()));

        self
    }

    /// Specify an embedded representation for the document.
    ///
    /// # Parameters
    /// - `value` - The embedded representation to specify.
    pub fn with_embedded_representation<V>(mut self, value: V) -> Self
    where
        V: Into<EmbeddedRepresentation>,
    {
        self.entities.push(Entity::Representation(value.into()));

        self
    }

    /// Specify a link for the document.
    ///
    /// # Parameters
    /// - `value` - The link to specify.
    pub fn with_link<L>(mut self, value: L) -> Self
    where
        L: Into<Link>,
    {
        self.links.push(value.into());

        self
    }

    /// Specify a action for the document.
    ///
    /// # Parameters
    /// - `value` - The action to specify.
    pub fn with_action<L>(mut self, value: L) -> Self
    where
        L: Into<Action>,
    {
        self.actions.push(value.into());

        self
    }

    /// Specify a title for the document.
    ///
    /// # Parameters
    /// - `value` - The title to specify.
    pub fn with_title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());

        self
    }
}

impl EmbeddedRepresentation {
    /// Create a new embedded representation.
    ///
    /// # Parameters
    /// - `payload` - The payload of the embedded representation.
    pub fn new<S>(payload: S) -> Self
    where
        S: Serialize,
    {
        let serialized = serde_json::to_value(payload).expect("Failed to serialize payload");

        Self {
            rel:        vec![],
            class:      vec![],
            properties: serialized,
            entities:   vec![],
            links:      vec![],
            actions:    vec![],
            title:      None,
        }
    }

    /// Apply a lambda to specify additional details on the representation.
    ///
    /// # Parameters
    /// - `f` - The lambda function to execute on the representation.
    pub fn with<F>(self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        f(self)
    }

    /// Specify a link relation for the representation.
    ///
    /// # Parameters
    /// - `value` - The link relation to specify.
    pub fn with_rel<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.rel.push(value.to_string());

        self
    }

    /// Specify a class for the representation.
    ///
    /// # Parameters
    /// - `value` - The class to specify.
    pub fn with_class<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.class.push(value.to_string());

        self
    }

    /// Specify an embedded link for the representation.
    ///
    /// # Parameters
    /// - `value` - The embedded link to specify.
    pub fn with_embedded_link<V>(mut self, value: V) -> Self
    where
        V: Into<Link>,
    {
        self.entities.push(Entity::Link(value.into()));

        self
    }

    /// Specify an embedded representation for the representation.
    ///
    /// # Parameters
    /// - `value` - The embedded representation to specify.
    pub fn with_embedded_representation<V>(mut self, value: V) -> Self
    where
        V: Into<EmbeddedRepresentation>,
    {
        self.entities.push(Entity::Representation(value.into()));

        self
    }

    /// Specify a link for the representation.
    ///
    /// # Parameters
    /// - `value` - The link to specify.
    pub fn with_link<L>(mut self, value: L) -> Self
    where
        L: Into<Link>,
    {
        self.links.push(value.into());

        self
    }

    /// Specify an action for the representation.
    ///
    /// # Parameters
    /// - `value` - The action to specify.
    pub fn with_action<L>(mut self, value: L) -> Self
    where
        L: Into<Action>,
    {
        self.actions.push(value.into());

        self
    }

    /// Specify a title for the representation.
    ///
    /// # Parameters
    /// - `value` - The title to specify.
    pub fn with_title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());

        self
    }
}

impl Link {
    /// Create a new link.
    ///
    /// # Parameters
    /// - `href` - The href for the link.
    pub fn new<S>(href: S) -> Self
    where
        S: ToString,
    {
        Self {
            rel:        vec![],
            class:      vec![],
            href:       href.to_string(),
            title:      None,
            media_type: None,
        }
    }

    /// Apply a lambda to specify additional details on the link.
    ///
    /// # Parameters
    /// - `f` - The lambda function to execute on the link.
    pub fn with<F>(self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        f(self)
    }

    /// Specify a class for the link.
    ///
    /// # Parameters
    /// - `value` - The class to specify.
    pub fn with_class<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.class.push(value.to_string());

        self
    }

    /// Specify a link relation for the link.
    ///
    /// # Parameters
    /// - `value` - The link relation to specify.
    pub fn with_rel<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.rel.push(value.to_string());

        self
    }

    /// Specify a type for the link.
    ///
    /// # Parameters
    /// - `value` - The type to specify.
    pub fn with_type<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.media_type = Some(value.to_string());

        self
    }

    /// Specify a title for the link.
    ///
    /// # Parameters
    /// - `value` - The title to specify.
    pub fn with_title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());

        self
    }
}

impl Action {
    /// Create a new action.
    ///
    /// # Parameters
    /// - `name` - The name of the action.
    /// - `href` - The href for the action.
    pub fn new<N, H>(name: N, href: H) -> Self
    where
        N: ToString,
        H: ToString,
    {
        Self {
            name:       name.to_string(),
            class:      vec![],
            method:     None,
            href:       href.to_string(),
            title:      None,
            media_type: None,
            fields:     vec![],
        }
    }

    /// Apply a lambda to specify additional details on the action.
    ///
    /// # Parameters
    /// - `f` - The lambda function to execute on the action.
    pub fn with<F>(self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        f(self)
    }

    /// Specify a class for the action.
    ///
    /// # Parameters
    /// - `value` - The class to specify.
    pub fn with_class<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.class.push(value.to_string());

        self
    }

    /// Specify a type for the action.
    ///
    /// # Parameters
    /// - `value` - The type to specify.
    pub fn with_type<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.media_type = Some(value.to_string());

        self
    }

    /// Specify a title for the action.
    ///
    /// # Parameters
    /// - `value` - The title to specify.
    pub fn with_title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());

        self
    }

    /// Specify an HTTP method for the action.
    ///
    /// # Parameters
    /// - `value` - The HTTP method to specify.
    pub fn with_method<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.method = Some(value.to_string());

        self
    }

    /// Specify a field for the action.
    ///
    /// # Parameters
    /// - `value` - The field to specify.
    pub fn with_field<V>(mut self, value: V) -> Self
    where
        V: Into<Field>,
    {
        self.fields.push(value.into());

        self
    }
}

impl Field {
    /// Create a new field.
    ///
    /// # Parameters
    /// - `name` - The name of the field.
    pub fn new<S>(name: S) -> Self
    where
        S: ToString,
    {
        Self {
            name:       name.to_string(),
            class:      vec![],
            input_type: None,
            value:      None,
            title:      None,
        }
    }

    /// Apply a lambda to specify additional details on the field.
    ///
    /// # Parameters
    /// - `f` - The lambda function to execute on the field.
    pub fn with<F>(self, f: F) -> Self
    where
        F: Fn(Self) -> Self,
    {
        f(self)
    }

    /// Specify a class for the field.
    ///
    /// # Parameters
    /// - `value` - The class to specify.
    pub fn with_class<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.class.push(value.to_string());

        self
    }

    /// Specify a type for the field.
    ///
    /// # Parameters
    /// - `value` - The type to specify.
    pub fn with_type<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.input_type = Some(value.to_string());

        self
    }

    /// Specify a value for the field.
    ///
    /// # Parameters
    /// - `value` - The value to specify.
    pub fn with_value<S>(mut self, value: S) -> Self
    where
        S: Serialize,
    {
        let serialized = serde_json::to_value(value).expect("Failed to serialize value");

        self.value = Some(serialized);

        self
    }

    /// Specify a title for the field.
    ///
    /// # Parameters
    /// - `value` - The title to specify.
    pub fn with_title<S>(mut self, value: S) -> Self
    where
        S: ToString,
    {
        self.title = Some(value.to_string());

        self
    }
}

#[cfg(test)]
mod tests {
    use assert2::check;
    use serde_json::json;

    use super::{
        super::values::{FieldTypes, HttpMethods, LinkRelation},
        *,
    };

    #[test]
    fn test_example() {
        let document = Document::new(json!({
            "orderNumber": 42,
            "itemCount": 3,
            "status": "pending"
        }))
        .with_class("order")
        .with_embedded_link(
            Link::new("http://api.x.io/orders/42/items")
                .with_class("items")
                .with_class("collection")
                .with_rel("http://x.io/rels/order-items"),
        )
        .with_embedded_representation(
            EmbeddedRepresentation::new(json!({
                "customerId": "pj123",
                "name": "Peter Joseph"
            }))
            .with_class("info")
            .with_class("customer")
            .with_rel("http://x.io/rels/customer")
            .with_link(
                Link::new("http://api.x.io/customers/pj123").with_rel(LinkRelation::SelfLink),
            ),
        )
        .with_action(
            Action::new("add-item", "http://api.x.io/orders/42/items")
                .with_title("Add Item")
                .with_method(HttpMethods::POST)
                .with_type("application/x-www-form-urlencoded")
                .with_field(
                    Field::new("orderNumber")
                        .with_type(FieldTypes::Hidden)
                        .with_value("42"),
                )
                .with_field(Field::new("productCode").with_type(FieldTypes::Text))
                .with_field(Field::new("quantity").with_type(FieldTypes::Number)),
        )
        .with_link(Link::new("http://api.x.io/orders/42").with_rel(LinkRelation::SelfLink))
        .with_link(Link::new("http://api.x.io/orders/41").with_rel(LinkRelation::Previous))
        .with_link(Link::new("http://api.x.io/orders/43").with_rel(LinkRelation::Next));

        let serialized = serde_json::to_value(document).unwrap();

        check!(
            serialized
                == json!(
                {
                  "class": [
                    "order"
                  ],
                  "properties": {
                    "orderNumber": 42,
                    "itemCount": 3,
                    "status": "pending"
                  },
                  "entities": [
                    {
                      "rel": [
                        "http://x.io/rels/order-items"
                      ],
                      "class": [
                        "items",
                        "collection"
                      ],
                      "href": "http://api.x.io/orders/42/items"
                    },
                    {
                      "rel": [
                        "http://x.io/rels/customer"
                      ],
                      "class": [
                        "info",
                        "customer"
                      ],
                      "properties": {
                        "customerId": "pj123",
                        "name": "Peter Joseph"
                      },
                      "links": [
                        {
                          "rel": [
                            "self"
                          ],
                          "href": "http://api.x.io/customers/pj123"
                        }
                      ]
                    }
                  ],
                  "links": [
                    {
                      "rel": [
                        "self"
                      ],
                      "href": "http://api.x.io/orders/42"
                    },
                    {
                      "rel": [
                        "previous"
                      ],
                      "href": "http://api.x.io/orders/41"
                    },
                    {
                      "rel": [
                        "next"
                      ],
                      "href": "http://api.x.io/orders/43"
                    }
                  ],
                  "actions": [
                    {
                      "name": "add-item",
                      "method": "POST",
                      "href": "http://api.x.io/orders/42/items",
                      "title": "Add Item",
                      "type": "application/x-www-form-urlencoded",
                      "fields": [
                        {
                          "name": "orderNumber",
                          "type": "hidden",
                          "value": "42"
                        },
                        {
                          "name": "productCode",
                          "type": "text"
                        },
                        {
                          "name": "quantity",
                          "type": "number"
                        }
                      ]
                    }
                  ]
                })
        );
    }
}
