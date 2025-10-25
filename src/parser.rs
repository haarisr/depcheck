use std::collections::HashSet;

use anyhow::Result;
use ruff_python_ast::visitor::Visitor;
use ruff_python_parser::{Mode, parse};

use crate::visitor::ImportVisitor;

pub fn parse_imports(contents: &str) -> Result<HashSet<String>> {
    let parsed = parse(contents, Mode::Module.into())?;
    let ast = parsed.into_syntax();
    let mut visitor = ImportVisitor::new();
    if let Some(module) = ast.module() {
        for stmt in module.body.iter() {
            visitor.visit_stmt(&stmt);
        }
    }
    Ok(visitor.into_imports())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import() -> Result<()> {
        let python = concat!(
            "import foo\n",
            "import numpy as np\n",
            "import torch\n",
            "import os\n",
        );

        let expected: HashSet<String> = ["foo", "numpy", "torch", "os"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = parse_imports(python)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_from_import() -> Result<()> {
        let python = concat!(
            "from pathlib import Path\n",
            "from flask import Flask\n",
            "from my_app import Cli\n",
            "from utils import greet as g\n"
        );

        let expected: HashSet<String> = ["pathlib", "flask", "my_app", "utils"]
            .into_iter()
            .map(String::from)
            .collect();
        let result = parse_imports(python)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_no_import() -> Result<()> {
        let python = "";

        let expected = HashSet::new();
        let result = parse_imports(python)?;
        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn test_invalid_syntax() -> Result<()> {
        let python = concat!("foo\n",);

        let expected = HashSet::new();
        let result = parse_imports(python)?;
        assert_eq!(result, expected);
        Ok(())
    }
}
