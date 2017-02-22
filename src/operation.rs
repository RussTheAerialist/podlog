
use std::str::FromStr;

#[derive(Debug,PartialEq)]
pub enum OperationSource {
    UNKNOWN(String),
    SOAP,
    REST,
    WEBSITE,
    BATCH
}

impl OperationSource {
    fn from_str(data: &str) -> OperationSource {
        match data {
            "SOAP" => OperationSource::SOAP,
            "REST" => OperationSource::REST,
            "WEBSITE" => OperationSource::WEBSITE,
            "BATCH" => OperationSource::BATCH,
            _ => OperationSource::UNKNOWN(data.to_string())
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum OperationMethod {
    UNKNOWN(String),
    GET,
    PUT,
    DELETE
}

impl OperationMethod {
    fn from_str(data: &str) -> OperationMethod {
        match data {
            "GET" => OperationMethod::GET,
            "PUT" => OperationMethod::PUT,
            "DELETE" => OperationMethod::DELETE,
            _ => OperationMethod::UNKNOWN(data.to_string())
        }
    }
}

#[derive(Debug,PartialEq)]
pub enum OperationResourceType {
    UNKNOWN(String),
    OBJECT
}

impl OperationResourceType {
    fn from_str(data: &str) -> OperationResourceType {
        match data {
            "OBJECT" => OperationResourceType::OBJECT,
            _ => OperationResourceType::UNKNOWN(data.to_string())
        }
    }
}

#[derive(Debug)]
pub struct Operation {
    pub source: OperationSource,
    pub method: OperationMethod,
    pub resource_type: OperationResourceType
}

impl Default for Operation {
    fn default() -> Operation {
        Operation {
            source: OperationSource::BATCH,
            method: OperationMethod::GET,
            resource_type: OperationResourceType::OBJECT
        }
    }
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(data: &str) -> Result<Operation, &'static str> {
        let mut parts = data.split(".");
        let source_option = parts.next();
        let method_option = parts.next();
        let resource_type_option = parts.next();

        let source = match source_option {
            Some(s) => s,
            None => return Err("Unable to find source")
        };

        let method = match method_option {
            Some(s) => s,
            None => return Err("Unable to find match")
        };

        let resource_type = match resource_type_option {
            Some(s) => s,
            None => return Err("Unable to find resource type")
        };

        Ok(Operation {
            source: OperationSource::from_str(source),
            method: OperationMethod::from_str(method),
            resource_type: OperationResourceType::from_str(resource_type)
        })
    }
}

