use biliup::uploader::bilibili::{Studio, Vid};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about)]
pub struct Cli {
    /// Turn debugging information on
    // #[clap(short, long, parse(from_occurrences))]
    // debug: usize,

    #[clap(subcommand)]
    pub command: Commands,

    /// 登录信息文件
    #[arg(short, long, default_value = "cookies.json")]
    pub user_cookie: PathBuf,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 登录B站并保存登录信息
    Login,
    /// 手动验证并刷新登录信息
    Renew,
    /// 上传视频
    Upload {
        // Optional name to operate on
        // name: Option<String>,
        /// 需要上传的视频路径,若指定配置文件投稿不需要此参数
        #[arg()]
        video_path: Vec<PathBuf>,

        /// Sets a custom config file
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// 选择上传线路
        #[arg(short, long, value_enum)]
        line: Option<UploadLine>,

        /// 单视频文件最大并发数
        #[arg(long, default_value = "3")]
        limit: usize,

        #[command(flatten)]
        studio: Studio,
    },
    /// 是否要对某稿件追加视频
    Append {
        // Optional name to operate on
        // name: Option<String>,
        /// vid为稿件 av 或 bv 号
        #[arg(short, long)]
        vid: Vid,
        /// 需要上传的视频路径,若指定配置文件投稿不需要此参数
        #[arg()]
        video_path: Vec<PathBuf>,

        /// 选择上传线路
        #[arg(short, long, value_enum)]
        line: Option<UploadLine>,

        /// 单视频文件最大并发数
        #[arg(long, default_value = "3")]
        limit: usize,

        #[command(flatten)]
        studio: Studio,
    },
    /// 打印视频详情
    Show {
        /// vid为稿件 av 或 bv 号
        // #[clap()]
        vid: Vid,
    },
    /// 输出flv元数据
    DumpFlv {
        #[arg()]
        file_name: PathBuf,
    },
    /// 下载视频
    Download {
        url: String,

        /// Output filename template. e.p. "./video/%Y-%m-%dT%H_%M_%S{title}"
        #[arg(short, long, default_value = "{title}")]
        output: String,

        /// 按照大小分割视频
        #[arg(long, value_parser = human_size)]
        split_size: Option<u64>,

        /// 按照时间分割视频
        #[arg(long)]
        split_time: Option<humantime::Duration>,
    },
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum UploadLine {
    Bda2,
    Ws,
    Qn,
    Kodo,
    Cos,
    CosInternal,
}

fn human_size(s: &str) -> Result<u64, String> {
    let ret = match s.as_bytes() {
        [init @ .., b'K'] => parse_u8(init)? * 1000.0,
        [init @ .., b'M'] => parse_u8(init)? * 1000.0 * 1000.0,
        [init @ .., b'G'] => parse_u8(init)? * 1000.0 * 1000.0 * 1000.0,
        init => parse_u8(init)?,
    };
    Ok(ret as u64)
}

fn parse_u8(string: &[u8]) -> Result<f64, String> {
    let string = String::from_utf8_lossy(string);
    string
        .parse()
        .map_err(|e| format!("{string} is not ascii digit. {:?}", e))
}
