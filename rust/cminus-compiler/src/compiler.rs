use ast::{Meta, Program, Parameter, Stmt, Expr, Operator};
use std::collections::HashMap;

struct SymbolEntry {
    param: Parameter,
}

type SymbolTables = Vec<HashMap<String, SymbolEntry>>;

pub fn process_symbol_table(program: &Program) {
    fn traverse_stmt(stmt: Meta<Stmt>, symbol_tables: &mut SymbolTables) {
        match stmt.inside {
            Stmt::Block(stmts) => {
                symbol_tables.push(HashMap::new());
                for stmt in stmts {
                    traverse_stmt(stmt, symbol_tables);
                }

                if let Some(table_for_scope) = symbol_tables.pop() {
                    for (var_name, _) in table_for_scope.iter() {
                        println!(
                            "variable {} out of scope for line {}",
                            var_name,
                            stmt.byte_offset
                        );
                    }
                }
            }
            Stmt::Declaration(ref parameters) => {
                for parameter in parameters {
                    println!(
                        "new variable {:?} in scope for line {}",
                        parameter.identifier,
                        stmt.get_line()
                    );
                    if let Some(table_for_scope) = symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry { param: parameter.clone() },
                        );
                    }
                }
            }
            Stmt::Function(_, parameters, statement) => {
                for parameter in parameters {
                    println!(
                        "new param {:?} in scope at line {}",
                        parameter.identifier,
                        stmt.byte_offset
                    );
                    if let Some(table_for_scope) = symbol_tables.last_mut() {
                        table_for_scope.insert(
                            parameter.identifier.clone(),
                            SymbolEntry { param: parameter.clone() },
                        );
                    }
                }
                traverse_stmt(*statement, symbol_tables);
            }
            Stmt::While(_, statement) => traverse_stmt(*statement, symbol_tables),
            Stmt::For(_, _, _, statement) => traverse_stmt(*statement, symbol_tables),
            _ => (),
        }
    }

    let mut symbol_tables: SymbolTables = vec![];
    for statement in program {
        traverse_stmt(statement.clone(), &mut symbol_tables);
    }
}
