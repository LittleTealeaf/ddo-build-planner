use git2::Repository;

pub struct GitRepo {
    repo: Repository,
}

impl GitRepo {
    pub fn open() -> Result<Self, git2::Error> {
        Ok(Self {
            repo: Repository::open(include_str!(concat!(env!("OUT_DIR"), "/repo_path")))?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_in_git_repository() {
        assert!(GitRepo::open().is_ok());
    }
}
