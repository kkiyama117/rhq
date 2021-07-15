use crate::util::process;
use anyhow::{anyhow, Result};
// use git2::{build::RepoBuilder, Repository};
use git2::Repository;
use std::{ffi::OsStr, path::Path};

pub fn init<P: AsRef<Path>>(path: P) -> Result<()> {
    Repository::init(path)
        .map_err(|e| anyhow!("command 'git' is exited with return code {:?}.", e.code()))?;
    Ok(())
}

pub fn clone<P, U, I, S>(url: U, path: P, args: I) -> Result<()>
where
    P: AsRef<Path>,
    U: AsRef<str>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    // let mut builder = RepoBuilder::new();
    // args.into_iter()
    //     .try_for_each(|arg: S| match (arg.as_ref() as &OsStr).to_str() {
    //         None => Err(anyhow!("invalid option {:?}.", arg.as_ref().to_str())),
    //         Some(arg) => match arg {
    //             _ => {
    //                 println!("{}", arg);
    //                 Ok(())
    //             }
    //         },
    //     })?;
    // builder
    //     .clone(url.as_ref(), path.as_ref())
    //     .map_err(|e| anyhow!("command 'git' is exited with {:?}.\n{:?}", e.code(),e.message()))?;
    // Ok(())
    let path = format!("{}", path.as_ref().display());
    process::inherit("git")
        .arg("clone")
        .args(&[url.as_ref(), &path])
        .args(args)
        .status()
        .map_err(Into::into)
        .and_then(|st| match st.code() {
            Some(0) => Ok(()),
            st => Err(anyhow!(
                "command 'git' is exited with return code {:?}.",
                st
            )),
        })
}

pub fn get_remote_url<P: AsRef<Path>>(repo_path: P) -> Result<Option<String>> {
    // 1. get current branch name.
    let output = process::piped("git")
        .current_dir(&repo_path)
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()?;
    if !output.status.success() {
        return Err(anyhow!("failed to get branch name"));
    }
    let branch = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    // 2. get remote name of upstream ref
    let arg = format!("{}@{{upstream}}", branch);
    let output = process::piped("git")
        .current_dir(&repo_path)
        .args(&["rev-parse", "--abbrev-ref", &arg])
        .output()?;
    if !output.status.success() {
        return Ok(None);
    }
    let upstream = String::from_utf8_lossy(&output.stdout)
        .trim()
        .trim_end_matches(&format!("/{}", branch))
        .to_owned();

    // 3. get remote URL of upstream ref
    Ok(Repository::open(repo_path)?
        .find_remote(&upstream)?
        .url()
        .map(Into::into))
}

pub fn set_remote<P: AsRef<Path>>(path: P, url: &str) -> Result<()> {
    Repository::open(path)?.remote("origin", url)?;
    Ok(())
}
