use docopt::Docopt;
use serde::Deserialize;

use CliResult;

pub fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );

    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) =>
            format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}

pub fn get_args<'a, T>(usage: &str, argv: &[&str]) -> CliResult<T>
    where T: Deserialize<'a> {
        Docopt::new(usage)
        .and_then(|d| d.argv(argv.iter().map(|&x| x))
                        .version(Some(version()))
                        .parse())
        .map_err(From::from)
    }