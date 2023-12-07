use git2::{Error, Repository};

pub fn get_git_commit() -> Result<String, Error> {
    Ok(Repository::open(".")?
        .head()?
        .peel_to_commit()?
        .id()
        .to_string())
}
