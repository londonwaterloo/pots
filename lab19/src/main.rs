#[allow(dead_code)]

/// Операция выполняемая над двумя выражениями.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// Операция в форме узла дерева.
#[derive(Debug)]
enum Expression {
    Op {
        op: Operation,
        left: Box<Expression>,
        right: Box<Expression>,
    },
    Value(i64),
}

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

fn eval(e: Expression) -> Result<i64, DivideByZeroError> {
    match e {
        Expression::Op { op, left, right } => {
            let left = eval(*left)?;
            let right = eval(*right)?;

            match op {
                Operation::Add => Ok(left + right),
                Operation::Sub => Ok(left - right),
                Operation::Mul => Ok(left * right),
                Operation::Div => {
                    if right != 0 {
                        Ok(left / right)
                    } else {
                        Err(DivideByZeroError)
                    }
                }
            }
        }
        Expression::Value(v) => Ok(v),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Add,
                left: Box::new(Expression::Value(2)),
                right: Box::new(Expression::Value(3)),
            }),
            Ok(5)
        );
    }

    #[test]
    fn test_sub() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Sub,
                left: Box::new(Expression::Value(20)),
                right: Box::new(Expression::Value(10)),
            }),
            Ok(10)
        );
    }

    #[test]
    fn test_mul() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Mul,
                left: Box::new(Expression::Value(6)),
                right: Box::new(Expression::Value(7)),
            }),
            Ok(42)
        );
    }

    #[test]
    fn test_div() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(100)),
                right: Box::new(Expression::Value(4)),
            }),
            Ok(25)
        );
    }

    #[test]
    fn test_error() {
        assert_eq!(
            eval(Expression::Op {
                op: Operation::Div,
                left: Box::new(Expression::Value(99)),
                right: Box::new(Expression::Value(0)),
            }),
            Err(DivideByZeroError)
        );
    }
}

fn main() {
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    };

    println!("выражение: {expr:?}");
    println!("результат: {:?}", eval(expr));
}