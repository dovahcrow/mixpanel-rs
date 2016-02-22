use std::error::Error;
use std::convert::From;

use MixPanel;

use services::Service;

use serde_json as json;
use serde::de::{Deserialize,Deserializer,Visitor,self};
use serde::de::MapVisitor;

pub struct ExportBuilder<'a> {
    client: &'a MixPanel,
    from_date: &'a str,
    to_date: &'a str,
    event: Vec<&'a str>,
    filter: Option<&'a str>,
    bucket: Option<&'a str>,
}

impl<'a> ExportBuilder<'a> {
    pub fn new(client: &'a MixPanel, from_date: &'a str, to_date: &'a str) -> ExportBuilder<'a> {
        ExportBuilder {
            from_date: from_date,
            to_date: to_date,
            client: client,
            event: vec![],
            filter: None,
            bucket: None
        }
    }

    pub fn add_event(mut self, event: &'a str) -> ExportBuilder<'a> {
        self.event.push(event);
        self
    }
    pub fn set_filter(mut self, filter: &'a str) -> ExportBuilder<'a> {
        self.filter = Some(filter);
        self
    }
    pub fn set_bucket(mut self, bucket: &'a str) -> ExportBuilder<'a> {
        self.bucket = Some(bucket);
        self
    }
    pub fn send(mut self) -> Result<Vec<ExportResult>, Box<Error>> {
        let event_string = format!("{:?}", self.event);
        let mut querys = vec![
            ("to_date", self.to_date),
            ("from_date", self.from_date),
            ];
        if self.event.len() != 0 {
            querys.push(("event", &event_string));
        }
        if let Some(filter) = self.filter.take() {
            querys.push(("where", filter));
        }
        if let Some(bucket) = self.bucket.take() {
            querys.push(("bucket", bucket));
        }

        let u8_seq = try!(self.client.send(Service::Export, &querys));
        let mut ret = vec![];
        for jsonl in u8_seq.split(|&ch| ch == '\n' as u8) {
            if jsonl.len() == 0 {
                continue
            }
            let jsonl: Vec<u8> = From::from(jsonl);
            let jsonl = try!(String::from_utf8(jsonl));
            ret.push(try!(json::from_str(&jsonl)));
        }
        Ok(ret)
    }
}

#[derive(Debug, Default)]
pub struct ExportResult {
    event: String,
    properties: ExportProperties
}

impl ExportResult {
    fn visitor() -> ExportResultVisitor {
        ExportResultVisitor
    }
}

pub struct ExportResultVisitor;

impl Visitor for ExportResultVisitor {
    type Value = ExportResult;
    fn visit_unit<E>(&mut self) -> Result<ExportResult, E> where E: de::Error {
        Err(de::Error::syntax("error"))
    }
    fn visit_map<V>(&mut self, mut visitor: V) -> Result<ExportResult, V::Error> where V: MapVisitor {
        let mut output = ExportResult {..Default::default()};
        while let Ok(Some(key)) = visitor.visit_key() {
            let key: String = key;
            match &key[..] {
                "event" => output.event = visitor.visit_value().ok().unwrap_or_default(),
                "properties" => output.properties = visitor.visit_value().ok().unwrap_or_default(),
                _ => {
                    try!(visitor.end())
                }
            }
        }
        try!(visitor.end());
        Ok(output)
    }
}

impl Deserialize for ExportResult {
    fn deserialize<D>(deserializer: &mut D) -> Result<ExportResult, D::Error> where D: Deserializer {
        deserializer.visit_map(Self::visitor())
    }
}

#[derive(Debug, Default)]
pub struct ExportProperties {
    time: i64,
    distinct_id: String,
    browser: Option<String>,
    browser_version: Option<String>,
    city: Option<String>,
    initial_referrer: Option<String>,
    initial_referring_domain: Option<String>,
    lib_version: Option<String>,
    os: Option<String>,
    referrer: Option<String>,
    referring_domain: Option<String>,
    region: Option<String>,
    screen_height: Option<String>,
    screen_width: Option<String>,
    mp_country_code: Option<String>,
    mp_lib: Option<String>,
    search_engine: Option<String>,
    device: Option<String>,
    origin: Option<String>,
    origin_referrer: Option<String>,
    origin_domain: Option<String>,
    tab: Option<String>,
}

impl ExportProperties {
    fn visitor() -> ExportPropertiesVisitor {
        ExportPropertiesVisitor
    }
}

struct ExportPropertiesVisitor;

macro_rules! set {
    ($output: ident, $visitor: ident, $key: ident, $($pattr: expr => $iattr: ident),*) => {{
        match &$key[..] {
            "distinct_id" => $output.distinct_id = $visitor.visit_value().ok().unwrap_or_default(),
            "time" => $output.time = $visitor.visit_value().ok().unwrap_or_default(),
            $($pattr => {
                $output.$iattr = Some($visitor.visit_value().ok().unwrap_or_default());
            })*
                e => {
                    let a: String = $visitor.visit_value().ok().unwrap_or_default();
                    println!("unexpected {}: {}", e, a)
            }
        }
    }}
}

impl Visitor for ExportPropertiesVisitor {
    type Value = ExportProperties;
    fn visit_unit<E>(&mut self) -> Result<ExportProperties, E> where E: de::Error {
        Err(de::Error::syntax("error"))
    }
    fn visit_map<V>(&mut self, mut visitor: V) -> Result<ExportProperties, V::Error> where V: MapVisitor {
        let mut output = ExportProperties {..Default::default()};
        while let Ok(Some(key)) = visitor.visit_key() {
            let key: String = key;
            set!(
                output, visitor, key,
                "$browser" => browser,
                "$browser_version" => browser_version,
                "$city" => city,
                "$initial_referrer" => initial_referrer,
                "$initial_referring_domain" => initial_referring_domain,
                "$lib_version" => lib_version,
                "$os" => os,
                "$referrer" => referrer,
                "$referring_domain" => referring_domain,
                "$region" => region,
                "$screen_height" => screen_height,
                "$screen_width" => screen_width,
                "mp_country_code" => mp_country_code,
                "mp_lib" => mp_lib,
                "$search_engine" => search_engine,
                "$device" => device
                )
        }
        try!(visitor.end());
        Ok(output)
    }
}

impl Deserialize for ExportProperties {
    fn deserialize<D>(deserializer: &mut D) -> Result<ExportProperties, D::Error> where D: Deserializer {
        deserializer.visit_map(Self::visitor())
    }
}
