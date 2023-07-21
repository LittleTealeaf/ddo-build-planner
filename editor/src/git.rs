use git2::Repository;

pub fn open_git_repository() -> Result<Repository, git2::Error> {
    Repository::open(include_str!(concat!(env!("OUT_DIR"), "/repo_path")))
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn git_repository_opens() {
        assert!(open_git_repository().is_ok());
    }
}
