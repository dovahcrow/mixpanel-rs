extern crate crypto;
extern crate hyper;
extern crate chrono;
extern crate serde;

use std::error::Error;
use std::io::Read;

use hyper::Url;
use hyper::Client;

use crypto::md5::Md5;
use crypto::digest::Digest;

mod export;
pub use export::ExportBuilder;

mod services;
use services::Service;

const BASE_URL: &'static str = "http://data.mixpanel.com/api/2.0";

pub struct MixPanel {
    api_key: String,
    api_secret: String,
}

impl MixPanel {
    pub fn new(key: &str, secret: &str) -> MixPanel {
        MixPanel {
            api_key: key.into(),
            api_secret: secret.into()
        }
    }

    pub fn export<'a>(&'a self, from_date: &'a str, to_date: &'a str) -> ExportBuilder<'a> {
        ExportBuilder::new(self, from_date, to_date)
    }
    fn send<'a>(&self, api: Service, foreign_querys: &[(&'a str, &'a str)]) -> Result<Vec<u8>, Box<Error>> {
        let client = Client::new();
        let mut url = try!(Url::parse(&api.construct_url()));

        let now = chrono::Local::now();
        let expire = format!("{}", now.timestamp() + 600);

        let mut sig_string;

        let mut querys = vec![
            ("api_key", &self.api_key[..]),
            ("expire", &expire)];

        // waiting for push_all available
        for elem in foreign_querys.iter() { querys.push(*elem); }
        // querys.push_all(foreign_querys);

        querys.sort();

        sig_string = querys.iter().fold(String::new(), |mut acc, qp| {
            let to_push = format!("{}={}", qp.0, qp.1);
            acc.push_str(&to_push);
            acc
        });
        sig_string.push_str(&self.api_secret);

        sig_string = md5(&sig_string);

        querys.push(("sig", &sig_string[..]));
        url.set_query_from_pairs(querys.into_iter());

        let mut res = try!(client.get(url)
                       .send());
        let mut body = vec![];
        try!(res.read_to_end(&mut body));
        Ok(body)
    }
}

fn md5(s: &str) -> String {
    let mut md5 = Md5::new();
    md5.input(s.as_bytes());
    let mut output: [u8; 16] = [0 ;16];
    md5.result(&mut output);
    let sig_string = output.iter()
        .flat_map(|&ch_u8| {
            vec![ch_u8>>4, ch_u8&0xf].into_iter()
        }).fold(String::new(), |mut acc, ch| {
            acc.push(std::char::from_digit(ch as u32, 16).unwrap());
            acc
        });
    sig_string
}
