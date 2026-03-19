// ======================================================
// ЗАДАНИЕ 6 — Вычисление арифметического выражения
//
// Выражение представлено как дерево.
//
// Нам нужно рекурсивно вычислить значение.
// ======================================================

/// Операция над выражениями
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// Узел дерева выражения
#[derive(Debug)]
enum Expression {
    /// Узел-операция с левым и правым поддеревом
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Лист дерева — просто число
    Value(i64),
}

/// Рекурсивное вычисление выражения.
fn eval(expr: &Expression) -> i64 {
    match expr {
        Expression::Value(v) => *v,
        Expression::Op { op, left, right } => {
            let l = eval(left);
            let r = eval(right);

            match op {
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mul => l * r,
                Operation::Div => l / r,
            }
        }
    }
}

/// Превращает дерево Expression в строку.
fn expr_to_string(expr: &Expression) -> String {
    match expr {
        Expression::Value(v) => v.to_string(),
        Expression::Op { op, left, right } => {
            let l = expr_to_string(left);
            let r = expr_to_string(right);

            let op_str = match op {
                Operation::Add => "+",
                Operation::Sub => "-",
                Operation::Mul => "*",
                Operation::Div => "/",
            };

            format!("({} {} {})", l, op_str, r)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        let expr = Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        };

        assert_eq!(eval(&expr), 30);
    }

    #[test]
    fn test_complex_expression() {
        let expr = Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(10)),
                right: Box::new(Expression::Value(9)),
            }),
            right: Box::new(Expression::Op {
                op: Operation::Mul,
                left: Box::new(Expression::Op {
                    op: Operation::Sub,
                    left: Box::new(Expression::Value(3)),
                    right: Box::new(Expression::Value(4)),
                }),
                right: Box::new(Expression::Value(5)),
            }),
        };

        assert_eq!(eval(&expr), 14);
    }

    #[test]
    fn test_division() {
        let expr = Expression::Op {
            op: Operation::Div,
            left: Box::new(Expression::Value(20)),
            right: Box::new(Expression::Value(5)),
        };

        assert_eq!(eval(&expr), 4);
    }
}

fn main() {
    let expr = Expression::Op {
        op: Operation::Add,
        left: Box::new(Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(9)),
        }),
        right: Box::new(Expression::Op {
            op: Operation::Mul,
            left: Box::new(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(3)),
                right: Box::new(Expression::Value(4)),
            }),
            right: Box::new(Expression::Value(5)),
        }),
    };

    let div_expr = Expression::Op {
        op: Operation::Div,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(5)),
    };

    println!("Было:  {}", expr_to_string(&expr));
    println!("Стало: {}", eval(&expr));

    println!("Было:  {}", expr_to_string(&div_expr));
    println!("Стало: {}", eval(&div_expr));
}