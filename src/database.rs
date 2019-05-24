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

    pub fn execute(&mut self, tree: AST) -> Result<(), &'static str> {
        match tree {
            AST::TableDef { name, members } => {
                self.table.insert(name.clone(), Table::new(name, members));
                Ok(())
            }
            AST::MethodCall { table, name, args } => {
                if let Some(table) = self.table.get_mut(&table) {
                    table.execute(name, args)
                } else {
                    Err("Table not found!!")
                }
            }
            _ => Err("Unimplemented AST!!"),
        }
    }
}

#[test]
fn new() {
    let db = Database::new();
    assert_eq!(db.table.len(), 0);
}

#[test]
fn execute() {
    let mut db = Database::new();
    // Create new table
    assert_eq!(
        db.execute(AST::TableDef {
            name: "Hoge".to_string(),
            members: vec![]
        }),
        Ok(())
    );

    assert_eq!(
        db.execute(AST::MethodCall {
            table: "Fuga".to_string(),
            name: "insert".to_string(),
            args: vec![]
        }),
        Err("Table not found!!")
    );
}
