use crate::{ast::AST, table::Table};
use std::collections::BTreeMap;

type Identifier = String;

#[derive(Debug)]
pub struct Database {
    table: BTreeMap<Identifier, Table>,
}

impl Database {
    pub fn new() -> Self {
        Self {
            table: BTreeMap::new(),
        }
    }

    pub fn execute(&mut self, tree: AST) {
        match tree {
            AST::TableDef { name, members } => {
                self.table.insert(name.clone(), Table::new(name, members));
            }
            AST::MethodCall { table, name, args } => {
                if let Some(table) = self.table.get_mut(&table) {
                    table.execute(name, args);
                }
            }
            _ => (),
        };
    }
}

#[test]
fn new() {
    let db = Database::new();
    assert_eq!(db.table.len(), 0);
}
