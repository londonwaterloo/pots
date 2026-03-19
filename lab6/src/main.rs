// ======================================================
// ЗАДАНИЕ 6 — Вычисление арифметического выражения
//
// Выражение представлено как дерево:
//
//      +
//     / \
//    10  20
//
// или более сложное:
//
//        +
//       / \
//      +   *
//     / \  / \
//    10  9  -  5
//           / \
//          3   4
//
// Нам нужно рекурсивно вычислить значение.
//
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
///
/// Алгоритм:
/// 1. Если это Value — просто вернуть число.
/// 2. Если это Op:
///    - рекурсивно вычислить левое поддерево
///    - рекурсивно вычислить правое поддерево
///    - применить операцию
fn eval(expr: &Expression) -> i64 {
    match expr {
        // БАЗА РЕКУРСИИ
        Expression::Value(v) => *v,

        // РЕКУРСИВНЫЙ СЛУЧАЙ
        Expression::Op { op, left, right } => {
            // Сначала вычисляем левую и правую часть
            let l = eval(left);
            let r = eval(right);

            // Затем применяем операцию
            match op {
                Operation::Add => l + r,
                Operation::Sub => l - r,
                Operation::Mul => l * r,
                Operation::Div => l / r, // деление целочисленное
            }
        }
    }
}

// ------------------------------------------------------
// Тесты
// ------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_add() {
        // 10 + 20
        let expr = Expression::Op {
            op: Operation::Add,
            left: Box::new(Expression::Value(10)),
            right: Box::new(Expression::Value(20)),
        };

        assert_eq!(eval(&expr), 30);
    }

    #[test]
    fn test_complex_expression() {
        // (10 + 9) + ((3 - 4) * 5)
        //
        // Ручной расчёт:
        // 10 + 9 = 19
        // 3 - 4 = -1
        // -1 * 5 = -5
        // 19 + (-5) = 14

        let expr =
            Expression::Op {
                op: Operation::Add,
                left: Box::new(
                    Expression::Op {
                        op: Operation::Add,
                        left: Box::new(Expression::Value(10)),
                        right: Box::new(Expression::Value(9)),
                    }
                ),
                right: Box::new(
                    Expression::Op {
                        op: Operation::Mul,
                        left: Box::new(
                            Expression::Op {
                                op: Operation::Sub,
                                left: Box::new(Expression::Value(3)),
                                right: Box::new(Expression::Value(4)),
                            }
                        ),
                        right: Box::new(Expression::Value(5)),
                    }
                ),
            };

        assert_eq!(eval(&expr), 14);
    }
}

/// Превращает дерево Expression в строку (инфиксная запись с скобками),
/// чтобы в выводе было "по-человечески": (10 + 9) + ((3 - 4) * 5)
fn expr_to_string(expr: &Expression) -> String {
    match expr {
        Expression::Value(v) => v.to_string(),

        Expression::Op { op, left, right } => {
            // Рекурсивно печатаем левую и правую части
            let l = expr_to_string(left);
            let r = expr_to_string(right);

            // Символ операции
            let op_str = match op {
                Operation::Add => "+",
                Operation::Sub => "-",
                Operation::Mul => "*",
                Operation::Div => "/",
            };

            // Скобки специально ставим всегда, чтобы было однозначно
            format!("({} {} {})", l, op_str, r)
        }
    }
}

fn main() {
    // (10 + 9) + ((3 - 4) * 5)
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

    let before = expr_to_string(&expr);
    let after = eval(&expr);

    println!("Было:  {}", before);
    println!("Стало: {}", after);
}