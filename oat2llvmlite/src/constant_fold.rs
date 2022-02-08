use oat_ast as oat;
use oat_ast::{BinaryOp, Expression, Statement, UnaryOp};

/// Constant fold an expression
pub fn constant_fold_expression<'a>(e: &'a Expression) -> &'a Expression {
    use BinaryOp::*;
    use Expression::*;
    match e {
        Binary { op, left, right } => {
            let left = constant_fold_expression(left);
            let right = constant_fold_expression(right);
            match (op, left, right) {
                (Add, CInt(i), CInt(j)) => &CInt(i + j),
                (Mul, CInt(i), CInt(j)) => &CInt(i * j),
                (Sub, CInt(i), CInt(j)) => &CInt(i - j),
                (IAnd, CInt(i), CInt(j)) => &CInt(i & j),
                (IOr, CInt(i), CInt(j)) => &CInt(i | j),
                (Eq, _, _) => &CBool(left == right),
                (Neq, _, _) => &CBool(left != right),
                (Lt, CInt(i), CInt(j)) => &CBool(i < j),
                (Lte, CInt(i), CInt(j)) => &CBool(i <= j),
                (Gt, CInt(i), CInt(j)) => &CBool(i > j),
                (Gte, CInt(i), CInt(j)) => &CBool(i >= j),

                (And, CBool(true), b) | (And, b, CBool(true)) => b,
                (And, CBool(false), b) | (And, b, CBool(false)) => &CBool(false),
                (Or, CBool(true), _) | (Or, _, CBool(true)) => &CBool(true),
                (Or, CBool(false), b) | (Or, b, CBool(false)) => b,
                (_, _, _) => {
                    return &Binary {
                        op: *op,
                        left: Box::new(left),
                        right: Box::new(right),
                    }
                }
            }
        }
        // Unary(op, e) => {
        // }
        CArr(t, elements) => &CArr(t, elements.iter().map(constant_fold_expression).collect()),
        _ => e,
    }
}

pub fn constant_fold_statement(statement: &Statement) -> Vec<Statement> {
    use Expression::*;
    use Statement::*;

    match statement {
        Assignment(target, value) => {
            let target = constant_fold_expression(target);
            let value = constant_fold_expression(value);
            vec![Assignment(target, value)]
        }
        Declaration(name, value) => {
            let value = constant_fold_expression(value);
            vec![Declaration(name, value)]
        }
        If {
            condition,
            then,
            else_,
        } => {
            let condition = constant_fold_expression(condition);
            let then = then.iter().map(constant_fold_statement).flatten().collect();
            let else_ = else_
                .iter()
                .map(constant_fold_statement)
                .flatten()
                .collect();
            match condition {
                CBool(true) => then,
                CBool(false) => else_,
                _ => vec![If {
                    condition,
                    then,
                    else_,
                }],
            }
        }
        While { condition, body } => {
            let condition = constant_fold_expression(condition);
            let body = compile_body(body);
            match condition {
                CBool(false) => vec![],
                _ => vec![While { condition, body }],
            }
        }
        _ => vec![statement],
    }
}

fn compile_body(body: &Vec<Statement>) -> Vec<Statement> {
    body.iter().map(constant_fold_statement).flatten().collect()
}
