use anyhow::Result;
use async_trait::async_trait;
use clap::clap_app;
use clap::App;
use clap::ArgMatches;
use var_shopper::notifier::Notifier;

#[async_trait]
pub trait SubCmd {
    async fn run(&self) -> Result<()>;
}

struct CmdVersion {}

impl CmdVersion {
    fn match_arg<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
        matches.subcommand_matches("version")
    }
    fn create(_flags: Flags, _matches: &ArgMatches) -> Box<dyn SubCmd> {
        Box::new(Self {})
    }
}

#[async_trait]
impl SubCmd for CmdVersion {
    async fn run(&self) -> Result<()> {
        async {
            println!("{}", crate::version());
            Ok(())
        }.await
    }
}

struct CmdTest {
    messages: Vec<String>,
}

impl CmdTest {
    fn match_arg<'a>(matches: &'a ArgMatches) -> Option<&'a ArgMatches<'a>> {
        matches.subcommand_matches("test")
    }
    fn create(_flags: Flags, matches: &ArgMatches) -> Box<dyn SubCmd> {
        Box::new(Self {
            messages: matches.values_of("messages")
                             .map(|args| args.map(|s| s.to_owned()).collect::<Vec<_>>())
                             .unwrap_or_else(Vec::new),
        })
    }
}

#[async_trait]
impl SubCmd for CmdTest {
    async fn run(&self) -> Result<()> {
        let client = var_shopper::notifier::telegram::Bot::new(
            r#"1895858891:AAH4-zAP_hCOTIMY8KAu6U8-oJviSVM4Ccs"#,
            r#"-1001282079800"#);
        for msg in self.messages.iter() {
            client.notify(msg).await?;
        }
        Ok(())
    }
}

fn clap_root<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    clap_app!(var_shopper =>
        (version: version)
        (author: "Raphanus Lo <coldturnip@gmail.com>")
        (about: "Online shopping observer")
        (@arg debug: -d ... "Sets the level of debugging information")
        (@subcommand version => (about: "print version info") )
        (@subcommand test =>
            (about: "try something")
            (@arg messages: ... "argument list")
        )
    )
}

#[derive(Default)]
pub struct Flags {
    pub argv: Vec<String>,
}

pub fn flags_from_vec(
    args: Vec<String>,
) -> clap::Result<Box<dyn SubCmd>> {
    let version = crate::version();
    let flags = Flags::default();

    let matches = clap_root(&version)
        .get_matches_from_safe(args)
        .map_err(|e| clap::Error {
            message: e.message.trim_start_matches("error: ").to_string(),
            ..e
        })?;

    let cmd = if let Some(m) = CmdVersion::match_arg(&matches) {
        CmdVersion::create(flags, m)
    } else if let Some(m) = CmdTest::match_arg(&matches) {
        CmdTest::create(flags, m)
    } else {
        unreachable!()
    };
    Ok(cmd)
}
