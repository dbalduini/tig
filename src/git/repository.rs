use crate::git::objects;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;

#[derive(Debug)]
pub struct Repository {
    worktree: String,
    pub gitdir: PathBuf,
}

impl Repository {
    pub fn new(path: String) -> Self {
        Self {
            worktree: path.clone(),
            gitdir: PathBuf::from(&path).join(".git"),
        }
    }

    pub fn init(&self, force: bool) -> std::io::Result<()> {
        // check if .git already exists
        // otherwise create
        //fs::create_dir_all(&self.gitdir)?;

        // read toml configuration
        // create configuration if not exists

        if !force {
            let version = 0;
            if version != 0 {
                // throws
            }
        }

        Ok(())
    }

    pub fn create(path: String) -> Result<Self, std::io::Error> {
        let repo = Self::new(path);

        repo.init(true)?;

        // create directories
        repo.repo_dir(PathBuf::from("branches"), true);
        repo.repo_dir(PathBuf::from("objects"), true);
        repo.repo_dir(PathBuf::from("refs/tags"), true);
        repo.repo_dir(PathBuf::from("refs/heads"), true);

        // create files
        let mut description = repo.repo_file(PathBuf::from("description"), true).unwrap();
        description.write_all(
            b"Unnamed repository; edit this file 'description' to name the repository.\n",
        )?;

        let mut head = repo.repo_file(PathBuf::from("HEAD"), true).unwrap();
        head.write_all(b"ref: refs/heads/master\n")?;

        let mut config = repo.repo_file(PathBuf::from("config"), true).unwrap();
        config.write_all(b"[core]\nrepositoryformatversion=0\nfilemode=false\nbare=false\n")?;

        Ok(repo)
    }

    // Compute path under gitdir
    fn repo_path(&self, path: PathBuf) -> PathBuf {
        PathBuf::from(&self.gitdir).join(path)
    }

    fn repo_dir(&self, p: PathBuf, mkdir: bool) -> PathBuf {
        let path = self.repo_path(p);

        if path.exists() {
            if path.is_dir() {
                return path;
            }
            panic!("path {} is not a directory", path.display());
        }

        if mkdir {
            std::fs::create_dir_all(&path).unwrap();
        }
        path
    }

    fn repo_file(&self, p: PathBuf, create: bool) -> std::io::Result<File> {
        if p.parent().is_some() {
            self.repo_dir(p.parent().unwrap().to_path_buf(), true);
        }

        let path = self.repo_path(p);

        if create {
            File::create(path)
        } else {
            File::open(path)
        }
    }

    pub fn object_write(&self, obj: objects::Blob, write: bool) -> std::io::Result<String> {
        let (hash, contents) = objects::hash_object(obj);

        if write {
            let path = get_path_from_hash(&hash);

            let mut file = self.repo_file(path, true)?;
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&contents.as_bytes())?;

            let compressed_bytes = encoder.finish()?;
            file.write_all(&compressed_bytes)?;
            file.flush()?;
        }

        Ok(hash)
    }

    pub fn object_read(&self, hash: String) -> objects::Blob {
        let path = get_path_from_hash(&hash);

        let file = self.repo_file(path, false).unwrap();

        let mut decoder = ZlibDecoder::new(file);

        let mut buff = String::new();
        decoder.read_to_string(&mut buff).unwrap();

        objects::parse_object(buff)
    }

    pub fn update_ref(&self, hash: String) {
        let mut file = self.repo_file(PathBuf::from("refs/heads/master"), true).unwrap();
        file.write_all((hash + "\n").as_bytes());
        file.flush();
    }
}

fn get_path_from_hash(hash: &str) -> PathBuf {
    PathBuf::from("objects").join(&hash[..2]).join(&hash[2..])
}
