#![feature(try_from)]

extern crate regex;
use regex::Regex;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate mime;
use mime::Mime;

extern crate url;
extern crate url_serde;
use url::{Url, ParseError};

use std::collections::HashMap;

// struct Swagger<'a> {    
//     swagger: String,
//     info: Info,
//     host: Option<Hostname>,
//     basePath: Option<PathBuf>,
//     schemes: Vec<Scheme>,
//     consumes: Vec<Mime>,
//     produces: Vec<Mime>,
//     paths: PathBuf,
//     definitions: Option<Definitions>,
//     parameters: Option<ParametersDefinitions>,
//     responses: Option<ResponsesDefinitions>,
//     securityDefinitions: Option<SecurityDefinitions>,
//     security: Vec<SecurityRequirement<'a>>,
//     tags: Vec<Tag>,
//     externalDocs: Option<ExternalDocumentation>,
//     extensions: Vec<String>
// }

// struct Hostname {
//     hostname: String
// }

// enum Scheme {
//     Http,
//     Https,
//     Ws,
//     Wss
// }

// use std::collections::HashMap;
// struct Definitions(HashMap<String, Schema>);
// struct ParametersDefinitions(HashMap<String, Parameter>);
// struct ResponsesDefinitions(HashMap<String, Response>);
// struct SecurityDefinitions(HashMap<String, SecurityScheme>);

// struct SecurityRequirement<'a> {
//     scheme: &'a SecurityScheme,
//     scope: &'a Scope
// }

// struct Tag {
//     name: String,
//     description: String,
//     externalDocs: ExternalDocumentation
// }

// struct Parameter {
//     name: String,
//     in_loc: ParamLocation,
//     description: Option<String>,
//     required: Option<bool>
// }

// enum ParamLocation {
//     Query,
//     Header,
//     Path,
//     FormData,
//     Body(Schema)
// }

// struct Response {
//     description: String,
//     schema: Option<Schema>,
//     headers: Option<Headers>,
//     examples: Option<Example>
// }

// struct Headers(HashMap<String, Header>);

// struct Header {
//     description: Option<String>,
//     swagger_type: SwaggerType,
//     format: Option<String>, // TODO confirm
//     items: Option<Items>,
//     collectionFormat: CollectionFormat,
//     // TODO schema stuff
// }

// enum SwaggerType {
//     String,
//     Number,
//     Integer,
//     Boolean,
//     Array
// }

// enum CollectionFormat {
//     Csv,
//     Ssv,
//     Tsv,
//     Pipes
// }

// struct Items {
//     swagger_type: SwaggerType,
//     format: Option<String>, // TODO confirm
//     items: Option<Box<Items>>,
//     collectionFormat: CollectionFormat,
//     // TODO schema stuff
// }

// use std::convert::TryFrom;

// struct TryFromHostnameError(());

// impl TryFrom<String> for Hostname {
//     type Error = TryFromHostnameError;
//     fn try_from(s: String) -> Result<Self, TryFromHostnameError> {
//         lazy_static! {
//             static ref hname: Regex = Regex::new(
//                 "^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])\
//                 (\\.([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])){0,3}\\.?$"
//             ).unwrap();
//         }
//         if hname.is_match(&s) {
//             Ok(Hostname { hostname: s })
//         }
//         else {
//             Err(TryFromHostnameError(()))
//         }
//     }
// }

// use std::str::FromStr;

// impl FromStr for Hostname {
//     type Err = TryFromHostnameError;
//     fn from_str(s: &str) -> Result<Hostname, TryFromHostnameError> {
//         lazy_static! {
//             static ref hname: Regex = Regex::new(
//                 "^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])\
//                 (\\.([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])){0,3}\\.?$"
//             ).unwrap();
//         }
//         if hname.is_match(s) {
//             Ok(Hostname { hostname: s.to_owned() })
//         }
//         else {
//             Err(TryFromHostnameError(()))
//         }
//     }
// }

use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

#[derive(Debug)]
struct ExternalDocumentation {
    description: Option<String>,
    url: Url,
    extensions: Option<serde_json::map::Map<String, serde_json::Value>> // TODO see if this works with YAML
}

impl<'de> Deserialize<'de> for ExternalDocumentation {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct ExternalDocumentationVisitor;

        impl<'de> Visitor<'de> for ExternalDocumentationVisitor {
            type Value = ExternalDocumentation;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("OpenAPI 2.0 External Documentation Object")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<ExternalDocumentation, V::Error>
                where V: MapAccess<'de>
            {
                let mut description = None;
                let mut url = None;
                let mut extensions = None;
                while let Some(key) = visitor.next_key::<String>()? {
                    match key.as_str() {
                        "description" => {
                            if description.is_some() {
                                return Err(de::Error::duplicate_field("description"));
                            }
                            description = Some(visitor.next_value()?);
                        }
                        "url" => {
                            if url.is_some() {
                                return Err(de::Error::duplicate_field("url"));
                            }
                            let tmp_url :url_serde::De<Url> = visitor.next_value()?; // TODO can we avoid this?
                            url = Some(tmp_url.into_inner());
                        }
                        _ => {
                            if !key.as_str().starts_with("x-") {
                                return Err(de::Error::custom(format!("invalid field name {}, extensions must start with x-", key.as_str())));
                            }
                            if !extensions.is_some() {
                                extensions = Some(serde_json::map::Map::new());
                            }
                            extensions.as_mut().unwrap().insert(key.as_str().to_owned(), visitor.next_value()?);
                        }
                    }
                }
                let url = url.ok_or_else(|| de::Error::missing_field("url"))?;

                Ok(ExternalDocumentation {description: description, url: url, extensions: extensions})
            }
        }

        deserializer.deserialize_map(ExternalDocumentationVisitor)
    }
}

#[derive(Debug)]
struct Info {
    title: String,
    description: Option<String>,
    terms_of_service: Option<String>,
    contact: Option<Contact>,
    license: Option<License>,
    version: String,
    extensions: Option<serde_json::map::Map<String, serde_json::Value>> // TODO see if this works with YAML
}

impl<'de> Deserialize<'de> for Info {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct InfoVisitor;

        impl<'de> Visitor<'de> for InfoVisitor {
            type Value = Info;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("OpenAPI 2.0 Info Object")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Info, V::Error>
                where V: MapAccess<'de>
            {
                let mut title = None;
                let mut description = None;
                let mut terms_of_service = None;
                let mut contact = None;
                let mut license = None;
                let mut version = None;
                let mut extensions = None;
                while let Some(key) = visitor.next_key::<String>()? {
                    match key.as_str() {
                        "title" => {
                            if title.is_some() {
                                return Err(de::Error::duplicate_field("title"));
                            }
                            title = Some(visitor.next_value()?);
                        }
                        "description" => {
                            if description.is_some() {
                                return Err(de::Error::duplicate_field("description"));
                            }
                            description = Some(visitor.next_value()?);
                        }
                        "termsOfService" => {
                            if terms_of_service.is_some() {
                                return Err(de::Error::duplicate_field("termsOfService"));
                            }
                            terms_of_service = Some(visitor.next_value()?);
                        }
                        "contact" => {
                            if contact.is_some() {
                                return Err(de::Error::duplicate_field("contact"));
                            }
                            contact = Some(visitor.next_value()?);
                        }
                        "license" => {
                            if license.is_some() {
                                return Err(de::Error::duplicate_field("license"));
                            }
                            license = Some(visitor.next_value()?);
                        }
                        "version" => {
                            if version.is_some() {
                                return Err(de::Error::duplicate_field("version"));
                            }
                            version = Some(visitor.next_value()?);
                        }
                        _ => {
                            if !key.as_str().starts_with("x-") {
                                return Err(de::Error::custom(format!("invalid field name {}, extensions must start with x-", key.as_str())));
                            }
                            if !extensions.is_some() {
                                extensions = Some(serde_json::map::Map::new());
                            }
                            extensions.as_mut().unwrap().insert(key.as_str().to_owned(), visitor.next_value()?);
                        }
                    }
                }
                let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
                let version = version.ok_or_else(|| de::Error::missing_field("version"))?;
                Ok(Info {
                    title: title, 
                    description: description, 
                    terms_of_service: terms_of_service, 
                    contact: contact,
                    license: license,
                    version: version,
                    extensions: extensions})
            }
        }

        deserializer.deserialize_map(InfoVisitor)
    }
}

#[derive(Deserialize, Debug)]
struct License {
    name: String,
    #[serde(with = "url_serde")]
    url: Option<Url>
}

#[derive(Debug)]
struct Contact {
    name: Option<String>,
    url: Option<Url>,
    email: Option<String>, // FIXME
    extensions: Option<serde_json::map::Map<String, serde_json::Value>> // TODO see if this works with YAML
}

impl<'de> Deserialize<'de> for Contact {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct ContactVisitor;

        impl<'de> Visitor<'de> for ContactVisitor {
            type Value = Contact;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("OpenAPI 2.0 Contact Object")
            }

            fn visit_map<V>(self, mut visitor: V) -> Result<Contact, V::Error>
                where V: MapAccess<'de>
            {
                let mut name = None;
                let mut url = None;
                let mut email = None;
                let mut extensions = None;
                while let Some(key) = visitor.next_key::<String>()? {
                    match key.as_str() {
                        "name" => {
                            if name.is_some() {
                                return Err(de::Error::duplicate_field("name"));
                            }
                            name = Some(visitor.next_value()?);
                        }
                        "url" => {
                            if url.is_some() {
                                return Err(de::Error::duplicate_field("url"));
                            }
                            let tmp_url :url_serde::De<Url> = visitor.next_value()?; // TODO can we avoid this?
                            url = Some(tmp_url.into_inner());
                        }
                        "email" => {
                            if email.is_some() {
                                return Err(de::Error::duplicate_field("email"));
                            }
                            email = Some(visitor.next_value()?);
                        }
                        _ => {
                            if !key.as_str().starts_with("x-") {
                                return Err(de::Error::custom(format!("invalid field name {}, extensions must start with x-", key.as_str())));
                            }
                            if !extensions.is_some() {
                                extensions = Some(serde_json::map::Map::new());
                            }
                            extensions.as_mut().unwrap().insert(key.as_str().to_owned(), visitor.next_value()?);
                        }
                    }
                }
                Ok(Contact {name: name, url: url, email: email, extensions: extensions})
            }
        }

        deserializer.deserialize_map(ContactVisitor)
    }
}

#[cfg(test)]
mod tests {
    use Contact;
    use License;
    use Info;
    use ExternalDocumentation;
    use serde_json;
    use url::Url;

    #[test]
    fn contact_to_rust_valid() {
        let contact_example_json = r#"{
            "name": "API Support",
            "url": "http://www.swagger.io/support",
            "email": "support@swagger.io"
        }"#;
        let contact: Contact = serde_json::from_str(contact_example_json).unwrap();

        assert!(contact.name == Some("API Support".to_string()));
        assert!(contact.url == Some(Url::parse("http://www.swagger.io/support").unwrap()));
        assert!(contact.email == Some("support@swagger.io".to_string()));
        println!("Contact Example: {:?}", contact);

        let contact_with_extensions_json = r#"{
            "name": "API Support",
            "url": "http://www.swagger.io/support",
            "email": "support@swagger.io",
            "x-hambuger": null,
            "x-sandwich": 99
        }"#;
        let contact: Contact = serde_json::from_str(contact_with_extensions_json).unwrap();

        assert!(contact.name == Some("API Support".to_string()));
        assert!(contact.url == Some(Url::parse("http://www.swagger.io/support").unwrap()));
        assert!(contact.email == Some("support@swagger.io".to_string()));
        // assert!(contact.extensions)
        println!("Contact With Extensions: {:?}", contact);
    }

    #[test]
    #[should_panic]
    fn contact_to_rust_invalid_url() { 
        let contact_invalid_url = r#"{
            "name": "API Support",
            "url": "-invalid-",
            "email": "support@swagger.io"
        }"#;
        let _contact: Contact = serde_json::from_str(contact_invalid_url).unwrap();
    }

    #[test]
    #[should_panic]
    fn contact_to_rust_invalid_field() {
        let contact_invalid_field = r#"{
            "name": "API Support",
            "url": "http://www.swagger.io/support",
            "email": "support@swagger.io",
            "moustache": "handlebar"
        }"#;
        let _contact: Contact = serde_json::from_str(contact_invalid_field).unwrap();        
    }

    #[test]
    fn license_to_rust_valid() {
        let license_example_json = r#"{
            "name": "Apache 2.0",
            "url": "http://www.apache.org/licenses/LICENSE-2.0.html"
        }"#;
        let license: License = serde_json::from_str(license_example_json).unwrap();

        assert!(license.name == "Apache 2.0".to_string());
        assert!(license.url == Some(Url::parse("http://www.apache.org/licenses/LICENSE-2.0.html").unwrap()));
        println!("license Example: {:?}", license);
    }

    #[test]
    #[should_panic]
    fn license_to_rust_invalid_url() { 
        let license_invalid_url = r#"{
            "name": "Apache 2.0",
            "url": "-invalid-"
        }"#;
        let _license: License = serde_json::from_str(license_invalid_url).unwrap();
    }

    #[test]
    fn info_to_rust_valid() {
        let info_example_json = r#"{
            "title": "Swagger Sample App",
            "description": "This is a sample server Petstore server.",
            "termsOfService": "http://swagger.io/terms/",
            "contact": {
                "name": "API Support",
                "url": "http://www.swagger.io/support",
                "email": "support@swagger.io"
            },
            "license": {
                "name": "Apache 2.0",
                "url": "http://www.apache.org/licenses/LICENSE-2.0.html"
            },
            "version": "1.0.1"
        }"#;
        let info: Info = serde_json::from_str(info_example_json).unwrap();

        assert!(info.title == "Swagger Sample App".to_string());
        assert!(info.description == Some("This is a sample server Petstore server.".to_string()));
        assert!(info.terms_of_service == Some("http://swagger.io/terms/".to_string()));
        assert!(info.version == "1.0.1".to_string());
        println!("info Example: {:?}", info);
    }

    #[test]
    fn extdoc_to_rust_valid() {
        let extdoc_example_json = r#"{
          "description": "Find more info here",
          "url": "https://swagger.io"
        }"#;
        let extdoc: ExternalDocumentation = serde_json::from_str(extdoc_example_json).unwrap();

        assert!(extdoc.description == Some("Find more info here".to_string()));
        assert!(extdoc.url == Url::parse("https://swagger.io").unwrap());
        println!("extdoc Example: {:?}", extdoc);
    }
}
