use std::error::Error;
use std::convert::From;

use MixPanel;

use services::Service;

use serde::json;
use serde::json::{Value,from_str};

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
    pub fn send(mut self) -> Result<Vec<Value>, Box<Error>> {
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

// #[derive(RustcDecodable, Debug)]
// pub struct ExportResult {
//     event: String,
//     properties: ExportProperties
// }

// #[derive(Debug)]
// pub struct ExportProperties {
//     distinct_id: String,
//     time: i64,
//     origin: Option<String>,
//     origin_referrer: Option<String>,
//     origin_domain: Option<String>,
//     tab: Option<String>,
//     browser: String
// }

// impl Deserialize for ExportProperties {
//     fn deserialize<D>(deserializer: &mut D) -> Result<Self, D> where D: Deserializer {
        
//     }
// }
