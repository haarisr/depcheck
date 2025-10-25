use ruff_python_ast as ast;
use ruff_python_ast::Stmt;
use ruff_python_ast::visitor::{Visitor, walk_stmt as ruff_walk_stmt};
use std::collections::HashSet;

#[derive(Default)]
pub struct ImportVisitor {
    imports: HashSet<String>,
}

impl ImportVisitor {
    pub fn new() -> Self {
        Self {
            imports: HashSet::new(),
        }
    }

    pub fn into_imports(self) -> HashSet<String> {
        self.imports
    }
}

impl<'a> Visitor<'a> for ImportVisitor {
    fn visit_stmt(&mut self, stmt: &'a Stmt) {
        walk_stmt(self, stmt);
    }
}

fn walk_stmt(visitor: &mut ImportVisitor, stmt: &Stmt) {
    match stmt {
        Stmt::Import(ast::StmtImport {
            names,
            range: _,
            node_index: _,
        }) => {
            visitor
                .imports
                .extend(names.iter().map(|item| item.name.to_string()));
            ruff_walk_stmt(visitor, stmt);
        }
        _ => {
            ruff_walk_stmt(visitor, stmt);
        }
    }
}
