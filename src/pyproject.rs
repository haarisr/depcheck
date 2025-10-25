use itertools::Itertools;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Result, anyhow};
use serde::Deserialize;

use uv_pep508::Requirement;
use uv_pypi_types::DependencyGroups;
use uv_pypi_types::LenientRequirement;
use uv_pypi_types::VerbatimParsedUrl;
use uv_requirements::RequirementsSource;
use uv_workspace::pyproject::Project;
use uv_workspace::pyproject::Tool as ToolUV;

/// TODO: Add link to uv file
/// A `pyproject.toml` as specified in PEP 517.
#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PyProjectToml {
    /// PEP 621-compliant project metadata.
    pub project: Option<Project>,
    /// Tool-specific metadata.
    pub tool: Option<ToolUV>,
    /// Non-project dependency groups, as defined in PEP 735.
    pub dependency_groups: Option<DependencyGroups>,
    /// The raw unserialized document.
    #[serde(skip)]
    pub raw: String,

    /// Used to determine whether a `build-system` section is present.
    #[serde(default, skip_serializing)]
    pub build_system: Option<serde::de::IgnoredAny>,
}

pub fn parse_requirements_from_file(path: PathBuf) -> Result<Vec<Requirement<VerbatimParsedUrl>>> {
    let source: RequirementsSource = RequirementsSource::from_requirements_file(path)?;
    let requirements = match source {
        RequirementsSource::PyprojectToml(path) => {
            let contents = match fs_err::read_to_string(&path) {
                Ok(contents) => contents,
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                    return Err(anyhow::anyhow!("File not found: `{}`", path.display()));
                }
                Err(err) => {
                    return Err(anyhow::anyhow!(
                        "Failed to read `{}`: {}",
                        path.display(),
                        err
                    ));
                }
            };
            let pyproject_toml = toml::from_str::<PyProjectToml>(&contents)?;
            let requirements = pyproject_toml
                .project
                .and_then(|p| p.dependencies)
                .unwrap_or_default();
            Ok(requirements)
        }
        _ => Err(anyhow!("We currently do not support other formats")),
    }?;

    let requirements: Vec<Requirement<VerbatimParsedUrl>> = requirements
        .into_iter()
        .map(|requires_dist| LenientRequirement::<VerbatimParsedUrl>::from_str(&requires_dist))
        .map_ok(Requirement::from)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(requirements)
}

#[cfg(test)]
mod tests {
    use super::*;
    use fs_err::File;
    use std::io::Write;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_import() -> Result<()> {
        let pyproject = r#"
[project]
name = "depcheck"
version = "1.2.3"
dependencies = [
  "requests[security,tests]==2.8.*,>=2.8.1 ; python_full_version < '2.7'",
  "numpy ( >=1.19 )",
  "awesome-project[dev]",
  "github-lib @ git+https://github.com/user/my-lib.git@v1.0.0",
  "whl-lib @ file:///${PROJECT_ROOT}/wheels/my_lib-0.2.1-py3-none-any.whl",
  "local-lib @ file:///path/to/local-lib",
]
"#;
        let tmp_dir = TempDir::new()?;
        let file_path = tmp_dir.path().join("pyproject.toml");

        let mut file = File::create(&file_path)?;
        file.write_all(pyproject.as_bytes())?;
        let result = parse_requirements_from_file(file.into_path())?;

        let expected_names: Vec<&str> = vec![
            "requests",
            "numpy",
            "awesome-project",
            "github-lib",
            "whl-lib",
            "local-lib",
        ];
        let names: Vec<&str> = result.iter().map(|req| req.name.as_str()).collect();
        assert_eq!(names, expected_names);
        Ok(())
    }
}
