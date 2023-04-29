use std::path::{Path, PathBuf};

use anyhow::Context;

use crate::config::{
    orchestrator_config::{self, OrchestratorConfig},
    tests::TestSpecificationID,
};
#[derive(Debug)]
pub struct HtpFolder(pub PathBuf, pub FolderType);

#[derive(Debug)]
pub enum FolderType {
    Dependency(DependencyFolderType),
    Test(TestFolderType),
}
#[derive(Debug)]
pub enum DependencyFolderType {
    BuildInput,
    BuildOutput,
}

#[derive(Debug)]
pub enum TestFolderType {
    Config,
    Persist,
}

impl HtpFolder {
    pub fn new_dependency(
        orchestrator_config: &OrchestratorConfig,
        folder_type: DependencyFolderType,
        dep_name: &str,
        uuid: &str,
    ) -> anyhow::Result<Self> {
        let mut path = orchestrator_config.htp_folder_root.clone();
        path.push("dependencies");
        path.push(format!("{}-{}", dep_name, uuid));
        path.push(match folder_type {
            DependencyFolderType::BuildInput => "build_input",
            DependencyFolderType::BuildOutput => "build_output",
        });
        log::info!("Creating {:?}", &path);
        std::fs::create_dir_all(&path).context("Cannot create {path}")?;
        Ok(Self(path, FolderType::Dependency(folder_type)))
    }
    pub fn new_test(
        orchestrator_config: &OrchestratorConfig,
        folder_type: TestFolderType,
        testid: &TestSpecificationID,
        version: &str,
    ) -> anyhow::Result<Self> {
        let mut path = orchestrator_config.htp_folder_root.clone();
        path.push("tests");
        path.push(testid.0.clone());
        path.push(format!("{}-{}", testid.1, version));
        path.push(match folder_type {
            TestFolderType::Config => "config",
            TestFolderType::Persist => "persist",
        });
        log::info!("Creating {:?}", &path);
        std::fs::create_dir_all(&path).context("Cannot create {path}")?;
        Ok(Self(path, FolderType::Test(folder_type)))
    }
    pub fn copy_from(&self, src: impl AsRef<Path>) -> anyhow::Result<()> {
        copy_dir_recurse(src, &self.0).context("Failed to copy from {src} to {self.0}")
    }
}

impl From<HtpFolder> for PathBuf {
    fn from(item: HtpFolder) -> Self {
        item.0
    }
}
impl<'a> From<&'a HtpFolder> for PathBuf {
    fn from(item: &'a HtpFolder) -> Self {
        item.0.clone()
    }
}

fn copy_dir_recurse(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_recurse(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
