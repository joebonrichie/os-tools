// SPDX-FileCopyrightText: Copyright © 2020-2025 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Operations that happen post-blit (primarily, triggers within container)
//! Note that we support transaction scope and system scope triggers, invoked
//! before `/usr` is activated and after, respectively.
//!
//! Note that currently we only load from `/usr/share/moss/triggers/{tx,sys.d}/*.yaml`
//! and do not yet support local triggers
use std::{
    collections::HashMap,
    os::fd::AsFd,
    path::{Path, PathBuf},
    process,
};
use thiserror::Error;

use composefs::{self, fs::FilesystemReader, fsverity::FsVerityHashValue, repository::Repository, tree::FileSystem};

pub fn read_filesystem<ObjectID: FsVerityHashValue>(
    dirfd: impl AsFd,
    path: &Path,
    repo: Option<&Repository<ObjectID>>,
    stat_root: bool,
) -> Result<FileSystem<ObjectID>, Error> {
    let mut reader = FilesystemReader {
        repo,
        inodes: HashMap::new(),
    };

    let root = reader.read_directory(dirfd, path.as_os_str(), stat_root)?;

    Ok(FileSystem {
        root,
        have_root_stat: stat_root,
    })
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("root must have an active state")]
    FsVerityHashValue,
    #[error("composefs")]
    Anyhow(#[from] anyhow::Error),
}
