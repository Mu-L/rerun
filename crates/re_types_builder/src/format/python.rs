use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};

use crate::CodeFormatter;

// ---

pub struct PythonCodeFormatter {
    pub pkg_path: Utf8PathBuf,
    pub testing_pkg_path: Utf8PathBuf,
}

impl PythonCodeFormatter {
    pub fn new(pkg_path: impl Into<Utf8PathBuf>, testing_pkg_path: impl Into<Utf8PathBuf>) -> Self {
        Self {
            pkg_path: pkg_path.into(),
            testing_pkg_path: testing_pkg_path.into(),
        }
    }
}

impl CodeFormatter for PythonCodeFormatter {
    fn format(&mut self, _reporter: &crate::Reporter, files: &mut crate::GeneratedFiles) {
        use rayon::prelude::*;

        re_tracing::profile_function!();

        // Running `black` once for each file is very slow, so we write all
        // files to a temporary folder, format it, and copy back the results.

        let tempdir = tempfile::tempdir().unwrap();
        let tempdir_path = Utf8PathBuf::try_from(tempdir.path().to_owned()).unwrap();

        files.par_iter().for_each(|(filepath, contents)| {
            let formatted_source_path = format_path_for_tmp_dir(
                &self.pkg_path,
                &self.testing_pkg_path,
                filepath,
                &tempdir_path,
            );
            crate::codegen::common::write_file(&formatted_source_path, contents);
        });

        format_python_dir(&tempdir_path).unwrap();

        // Read back and copy to the final destination:
        files.par_iter_mut().for_each(|(filepath, contents)| {
            let formatted_source_path = format_path_for_tmp_dir(
                &self.pkg_path,
                &self.testing_pkg_path,
                filepath,
                &tempdir_path,
            );
            *contents = std::fs::read_to_string(formatted_source_path).unwrap();
        });
    }
}

fn format_path_for_tmp_dir(
    pkg_path: &Utf8Path,
    testing_pkg_path: &Utf8Path,
    filepath: &Utf8Path,
    tempdir_path: &Utf8Path,
) -> Utf8PathBuf {
    // If the prefix is pkg_path, strip it, and then append to tempdir
    // However, if the prefix is testing_pkg_path, strip it and insert an extra
    // "testing" to avoid name collisions.
    filepath.strip_prefix(pkg_path).map_or_else(
        |_| {
            tempdir_path
                .join("testing")
                .join(filepath.strip_prefix(testing_pkg_path).unwrap())
        },
        |f| tempdir_path.join(f),
    )
}

fn format_python_dir(dir: &Utf8PathBuf) -> anyhow::Result<()> {
    re_tracing::profile_function!();

    // The order below is important and sadly we need to call black twice. Ruff does not yet
    // fix line-length (See: https://github.com/astral-sh/ruff/issues/1904).
    //
    // 1) Call black, which among others things fixes line-length
    // 2) Call ruff, which requires line-lengths to be correct
    // 3) Call black again to cleanup some whitespace issues ruff might introduce

    run_black_on_dir(dir).context("black")?;
    run_ruff_on_dir(dir).context("ruff")?;
    run_black_on_dir(dir).context("black")?;
    Ok(())
}

fn python_project_path() -> Utf8PathBuf {
    let path = crate::rerun_workspace_path()
        .join("rerun_py")
        .join("pyproject.toml");
    assert!(path.exists(), "Failed to find {path:?}");
    path
}

fn run_black_on_dir(dir: &Utf8PathBuf) -> anyhow::Result<()> {
    re_tracing::profile_function!();
    use std::process::{Command, Stdio};

    let proc = Command::new("black")
        .arg(format!("--config={}", python_project_path()))
        .arg(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = proc.wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        anyhow::bail!("{stdout}\n{stderr}")
    }
}

fn run_ruff_on_dir(dir: &Utf8PathBuf) -> anyhow::Result<()> {
    re_tracing::profile_function!();
    use std::process::{Command, Stdio};

    let proc = Command::new("ruff")
        .arg(format!("--config={}", python_project_path()))
        .arg("--fix")
        .arg(dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let output = proc.wait_with_output()?;

    if output.status.success() {
        Ok(())
    } else {
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        anyhow::bail!("{stdout}\n{stderr}")
    }
}
