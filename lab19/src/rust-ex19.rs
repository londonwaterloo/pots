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
    /// Операция выполняемая над двумя выражениями.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// значение 
    Value(i64),
}

#[derive(PartialEq, Eq, Debug)]
struct DivideByZeroError;

// Начальная реализация вычислителя выражений. Измените так чтобы
// возвращался Result и ошибка при делении на 0.
fn eval(e: Expression) -> i64 {
    match e {
        Expression::Op { op, left, right } => {
            let left = eval(*left);
            let right = eval(*right);
            match op {
                Operation::Add => left + right,
                Operation::Sub => left - right,
                Operation::Mul => left * right,
                Operation::Div => if right != 0 {
                    left / right
                } else {
                    panic!("Нельзя делить на ноль!");
                },
            }
        }
        Expression::Value(v) => v,
    }
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

fn main() {
    let expr = Expression::Op {
        op: Operation::Sub,
        left: Box::new(Expression::Value(20)),
        right: Box::new(Expression::Value(10)),
    };
    println!("выражение: {expr:?}");
    println!("результат: {:?}", eval(expr));
}