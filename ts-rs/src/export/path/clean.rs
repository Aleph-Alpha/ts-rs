use std::path::{PathBuf, Path, Component};

pub trait PathClean: AsRef<Path> {
    fn clean(&self) -> PathBuf {
        let mut out = Vec::new();

        for comp in self.as_ref().components() {
            match comp {
                Component::CurDir => (),
                Component::ParentDir => match out.last() {
                    Some(Component::RootDir) => (),
                    Some(Component::Normal(_)) => {
                        out.pop();
                    }
                    None
                    | Some(Component::CurDir)
                    | Some(Component::ParentDir)
                    | Some(Component::Prefix(_)) => out.push(comp),
                },
                comp => out.push(comp),
            }
        }

        if !out.is_empty() {
            out.iter().collect()
        } else {
            PathBuf::from(".")
        }
    }
}

impl<T: AsRef<Path>> PathClean for T {}

