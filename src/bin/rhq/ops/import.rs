use crate::ops::ClapCommand;
use anyhow::Result;
use clap::{App, ArgMatches};
use rhq::Workspace;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ImportCommand {
    roots: Option<Vec<PathBuf>>,
    depth: Option<usize>,
    verbose: bool,
}

impl<'a> ClapCommand<'a> for ImportCommand {
    fn app<'b: 'a>(app: App<'a, 'b>) -> App<'a, 'b> {
        app.about("Import existed repositories into management")
            .arg_from_usage("[roots]...      'Root directories contains for scanning'")
            .arg_from_usage("--depth=[depth] 'Maximal depth of entries for each base directory'")
            .arg_from_usage("-v, --verbose   'Use verbose output'")
    }

    fn from_matches<'b: 'a>(m: &ArgMatches) -> ImportCommand {
        ImportCommand {
            roots: m.values_of("roots").map(|s| s.map(PathBuf::from).collect()),
            depth: m.value_of("depth").and_then(|s| s.parse().ok()),
            verbose: m.is_present("verbose"),
        }
    }

    fn run(self) -> Result<()> {
        let mut workspace = Workspace::new()?.verbose_output(self.verbose);

        let roots = self
            .roots
            .unwrap_or_else(|| workspace.config().include_dirs.clone());
        for root in roots {
            workspace.import_repositories(root, self.depth)?;
        }
        workspace.save_cache()?;

        Ok(())
    }
}
