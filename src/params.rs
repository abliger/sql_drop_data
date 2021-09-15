use clap::{AppSettings,Clap};

#[derive(Clap)]
#[clap(version = "1.0", author = "fengsixue <3330181534@qq.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// 用户名
    #[clap(short = 'u', long)]
    pub username: String,
    /// 密码
    #[clap(short = 'p', long)]
    pub password: String,
    /// 数据库链接
    #[clap(short = 'd', long, default_value = "//118.31.18.193:1531/helowin")]
    pub database: String,
    /// 初始化sql文件
    #[clap(short = 'm', long)]
    pub modify: Vec<String>,
    /// 是否需要删除
    #[clap(short = 't', long="type")]
    pub type_app: String,
}

impl Opts {
    pub(crate) fn parse_app()->Opts{
        Opts::parse()
    }
}

