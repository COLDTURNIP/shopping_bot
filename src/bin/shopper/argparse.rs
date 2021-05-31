use std::future::Future;
use std::pin::Pin;

use anyhow::bail;
use anyhow::Result;
use clap::clap_app;
use clap::App;
use clap::ArgMatches;

pub trait SubCmd {
    type Output: Future<Output = Result<()>>;
    fn run(&self) -> Self::Output;
}

type AsyncSubCmdResult = Pin<Box<dyn Future<Output = Result<()>>>>;

struct CmdVersion {}

impl CmdVersion {
    fn matchArg<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
        matches.subcommand_matches("version")
    }
    fn create(_flags: Flags, _matches: &ArgMatches) -> Box<dyn SubCmd<Output = AsyncSubCmdResult>> {
        Box::new(Self {})
    }
}

impl SubCmd for CmdVersion {
    type Output = AsyncSubCmdResult;
    fn run(&self) -> Self::Output {
        Box::pin(async {
            println!("{}", crate::version());
            Ok(())
        })
    }
}

struct CmdTest {}

impl CmdTest {
    fn matchArg<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
        matches.subcommand_matches("test")
    }
    fn create(_flags: Flags, _matches: &ArgMatches) -> Box<dyn SubCmd<Output = AsyncSubCmdResult>> {
        Box::new(Self {})
    }
}

impl SubCmd for CmdTest {
    type Output = Pin<Box<dyn Future<Output = Result<()>>>>;
    fn run(&self) -> Self::Output {
        let fut = async {
            costco_observer::costco_observer::telegram::hello().await;
            Ok(())
        };
        Box::pin(fut)
    }
}

fn clap_root<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    clap_app!(costco_observer =>
        (version: "1.0")
        (author: "Raphanus Lo <coldturnip@gmail.com>")
        (about: "Costco observer")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand version => (about: "print version info") )
        (@subcommand test => (about: "try something") )
    )
}

#[derive(Default)]
pub struct Flags {
    pub argv: Vec<String>,
}

pub fn flags_from_vec(
    args: Vec<String>,
) -> clap::Result<Box<dyn SubCmd<Output = AsyncSubCmdResult>>> {
    let version = crate::version();
    let flags = Flags::default();

    let matches = clap_root(&version)
        .get_matches_from_safe(args)
        .map_err(|e| clap::Error {
            message: e.message.trim_start_matches("error: ").to_string(),
            ..e
        })?;

    let cmd = if let Some(m) = CmdVersion::matchArg(&matches) {
        CmdVersion::create(flags, m)
    } else if let Some(m) = CmdTest::matchArg(&matches) {
        CmdTest::create(flags, m)
    } else {
        unreachable!()
    };
    Ok(cmd)
}
