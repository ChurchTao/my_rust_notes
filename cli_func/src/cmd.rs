use std::str::FromStr;
use anyhow::{anyhow,Result};
use clap::Parser;
use reqwest::Url;

// 子命令分别对应不同的 HTTP 方法，目前只支持 get / post
#[derive(Parser, Debug)]
pub enum SubCommand {
    Get(Get),
    Post(Post),
    // 暂且不支持其它 HTTP 方法
}
//default_value_t

/// Get请求
#[derive(Parser, Debug)]
pub struct Get {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    pub url: String,

    #[clap(short='H',parse(try_from_str = parse_kv_pair))]
    pub header: Vec<KvPair>,
}

/// Post请求
#[derive(Parser, Debug)]
pub struct Post {
    /// HTTP 请求的 URL
    #[clap(parse(try_from_str = parse_url))]
    pub url: String,
    /// HTTP 请求的 body
    #[clap(parse(try_from_str = parse_kv_pair))]
    pub body: Vec<KvPair>,
}

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq)]
pub struct KvPair {
    pub k: String,
    pub v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn parse_url(s: &str) -> Result<String> {
    // 这里我们仅仅检查一下 URL 是否合法
    let _url: Url = s.parse()?;

    Ok(s.into())
}
