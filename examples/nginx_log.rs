// parse:
// 93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)"
// ^(?<ip>\S+)\s+\S+\s+\S+\s+\[(?<date>[^\]]+)\]\s+"(?<mothd>\S+)\s+(?<url>\S+)\s+(?<proto>[^"]+)"\s+(?<status>\d+)\s+(?<bytes>\d+)\s+"(?<referer>[^"]+)"\s+"(?<ua>[^"]+)"$
#[allow(unused)]
#[derive(Debug)]
struct NginxLog {
    addr: String,
    datetime: String,
    mothd: String,
    url: String,
    proto: String,
    status: u32,
    body_bytes: u64,
    referer: String,
    ua: String,
}
use regex::Regex;

use anyhow::Result;
fn main() -> Result<()> {
    let s = r#"93.180.71.3 - - [17/May/2015:08:05:32 +0000] "GET /downloads/product_1 HTTP/1.1" 304 0 "-" "Debian APT-HTTP/1.3 (0.8.16~exp12ubuntu10.21)""#;
    let log = parse_nginx_log(s)?;
    println!("{:?}", log);
    Ok(())
}

fn parse_nginx_log(s: &str) -> Result<NginxLog> {
    let re = Regex::new(
        r#"^(?P<ip>\S+)\s+\S+\s+\S+\s+\[(?P<date>[^\]]+)\]\s+"(?P<mothd>\S+)\s+(?P<url>\S+)\s+(?P<proto>[^"]+)"\s+(?P<status>\d+)\s+(?P<bytes>\d+)\s+"(?P<referer>[^"]+)"\s+"(?P<ua>[^"]+)"$"#,
    )?;
    let caps = re.captures(s).ok_or(anyhow::anyhow!("no match"))?;
    Ok(NginxLog {
        addr: caps["ip"].to_string(),
        datetime: caps["date"].to_string(),
        mothd: caps["mothd"].to_string(),
        url: caps["url"].to_string(),
        proto: caps["proto"].to_string(),
        status: caps["status"].parse()?,
        body_bytes: caps["bytes"].parse()?,
        referer: caps["referer"].to_string(),
        ua: caps["ua"].to_string(),
    })
}
