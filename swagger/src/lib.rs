#![feature(try_from)]

extern crate regex;
use regex::Regex;
use std::path::PathBuf;

#[macro_use]
extern crate lazy_static;

extern crate mime;
use mime::Mime;

extern crate url;
use url::{Url, ParseError};

struct Swagger<'a> {    
    swagger: String,
    info: Info,
    host: Option<Hostname>,
    basePath: Option<PathBuf>,
    schemes: Option<Vec<Scheme>>,
    consumes: Option<Vec<Mime>>,
    produces: Option<Vec<Mime>>,
    paths: PathBuf,
    definitions: Option<Definitions>,
    parameters: Option<ParametersDefinitions>,
    responses: Option<ResponsesDefinitions>,
    securityDefinitions: Option<SecurityDefinitions>,
    security: Option<Vec<SecurityRequirement<'a>>>,
    tags: Option<Vec<Tag>>,
    externalDocs: Option<ExternalDocumentation>,
    extensions: Option<Vec<String>>
}

struct Info {
    title: String,
    description: String,
    termsOfService: String,
    contact: Contact,
    license: License,
    version: String,
    extensions: String
}

struct Hostname {
    hostname: String
}

enum Scheme {
    Http,
    Https,
    Ws,
    Wss
}

use std::collections::HashMap;
struct Definitions(HashMap<String, Schema>);
struct ParametersDefinitions(HashMap<String, Parameter>);
struct ResponsesDefinitions(HashMap<String, Response>);
struct SecurityDefinitions(HashMap<String, SecurityScheme>);

struct SecurityRequirement<'a> {
    scheme: &'a SecurityScheme,
    scope: &'a Scope
}

struct Tag {
    name: String,
    description: String,
    externalDocs: ExternalDocumentation
}

struct ExternalDocumentation {
    description: String,
    url: Url
}

struct Contact {
    name: String,
    url: Url,
    email: String // fixme
}

struct License {
    name: String,
    url: Url
}

struct Parameter {
    name: String,
    in_loc: ParamLocation,
    description: Option<String>,
    required: Option<bool>
}

enum ParamLocation {
    Query,
    Header,
    Path,
    FormData,
    Body(Schema)
}

struct Response {
    description: String,
    schema: Option<Schema>,
    headers: Option<Headers>,
    examples: Option<Example>
}

struct Headers(HashMap<String, Header>);

struct Header {
    description: Option<String>,
    swagger_type: SwaggerType,
    format: Option<String>, // TODO confirm
    items: Option<Items>,
    collectionFormat: CollectionFormat,
    // TODO schema stuff
}

enum SwaggerType {
    String,
    Number,
    Integer,
    Boolean,
    Array
}

enum CollectionFormat {
    Csv,
    Ssv,
    Tsv,
    Pipes
}

struct Items {
    swagger_type: SwaggerType,
    format: Option<String>, // TODO confirm
    items: Option<Box<Items>>,
    collectionFormat: CollectionFormat,
    // TODO schema stuff
}

use std::convert::TryFrom;

struct TryFromHostnameError(());

impl TryFrom<String> for Hostname {
    type Error = TryFromHostnameError;
    fn try_from(s: String) -> Result<Self, TryFromHostnameError> {
        lazy_static! {
            static ref hname: Regex = Regex::new(
                "^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])\
                (\\.([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])){0,3}\\.?$"
            ).unwrap();
        }
        if hname.is_match(&s) {
            Ok(Hostname { hostname: s })
        }
        else {
            Err(TryFromHostnameError(()))
        }
    }
}

use std::str::FromStr;

impl FromStr for Hostname {
    type Err = TryFromHostnameError;
    fn from_str(s: &str) -> Result<Hostname, TryFromHostnameError> {
        lazy_static! {
            static ref hname: Regex = Regex::new(
                "^([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])\
                (\\.([a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9-]{0,61}[a-zA-Z0-9])){0,3}\\.?$"
            ).unwrap();
        }
        if hname.is_match(s) {
            Ok(Hostname { hostname: s.to_owned() })
        }
        else {
            Err(TryFromHostnameError(()))
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


