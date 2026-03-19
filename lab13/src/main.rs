/// Узел двоичного дерева.
#[derive(Debug)]
struct Node<T: Ord> {
    value: T,
    left: Subtree<T>,
    right: Subtree<T>,
}

/// Возможно пустое поддерево.
#[derive(Debug)]
struct Subtree<T: Ord>(Option<Box<Node<T>>>);

/// Контейнер, сохраняющий множество значений с помощью двоичного дерева.
///
/// Если одно значение добавляется несколько раз, сохраняется только одно.
#[derive(Debug)]
pub struct BinaryTree<T: Ord> {
    root: Subtree<T>,
}

impl<T: Ord> BinaryTree<T> {
    fn new() -> Self {
        Self { root: Subtree::new() }
    }

    fn insert(&mut self, value: T) {
        self.root.insert(value);
    }

    fn has(&self, value: &T) -> bool {
        self.root.has(value)
    }

    fn len(&self) -> usize {
        self.root.len()
    }
}

impl<T: Ord> Subtree<T> {
    /// Создает пустое поддерево.
    fn new() -> Self {
        Subtree(None)
    }

    /// Вставляет значение в поддерево.
    ///
    /// Если поддерево пустое, создается новый узел.
    /// Если значение уже существует, оно не добавляется повторно.
    fn insert(&mut self, value: T) {
        match &mut self.0 {
            // Если поддерево пустое — создаем новый узел
            None => {
                self.0 = Some(Box::new(Node {
                    value,
                    left: Subtree::new(),
                    right: Subtree::new(),
                }));
            }

            // Если узел уже есть — идем влево, вправо или ничего не делаем
            Some(node) => {
                if value < node.value {
                    node.left.insert(value);
                } else if value > node.value {
                    node.right.insert(value);
                }
                // Если value == node.value, ничего не делаем:
                // множество хранит только уникальные значения
            }
        }
    }

    /// Возвращает количество узлов в поддереве.
    fn len(&self) -> usize {
        match &self.0 {
            None => 0,
            Some(node) => 1 + node.left.len() + node.right.len(),
        }
    }

    /// Проверяет, содержится ли значение в поддереве.
    fn has(&self, value: &T) -> bool {
        match &self.0 {
            None => false,
            Some(node) => {
                if value < &node.value {
                    node.left.has(value)
                } else if value > &node.value {
                    node.right.has(value)
                } else {
                    true
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn len() {
        let mut tree = BinaryTree::new();
        assert_eq!(tree.len(), 0);
        tree.insert(2);
        assert_eq!(tree.len(), 1);
        tree.insert(1);
        assert_eq!(tree.len(), 2);
        tree.insert(2); // not a unique item
        assert_eq!(tree.len(), 2);
    }

    #[test]
    fn has() {
        let mut tree = BinaryTree::new();

        fn check_has(tree: &BinaryTree<i32>, exp: &[bool]) {
            let got: Vec<bool> =
                (0..exp.len()).map(|i| tree.has(&(i as i32))).collect();
            assert_eq!(&got, exp);
        }

        check_has(&tree, &[false, false, false, false, false]);
        tree.insert(0);
        check_has(&tree, &[true, false, false, false, false]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(4);
        check_has(&tree, &[true, false, false, false, true]);
        tree.insert(3);
        check_has(&tree, &[true, false, false, true, true]);
    }

    #[test]
    fn unbalanced() {
        let mut tree = BinaryTree::new();
        for i in 0..100 {
            tree.insert(i);
        }
        assert_eq!(tree.len(), 100);
        assert!(tree.has(&50));
    }
}

fn main() {
    let mut tree = BinaryTree::new();

    tree.insert(2);
    tree.insert(1);
    tree.insert(3);
    tree.insert(2); // повторно не добавится

    println!("Дерево: {:?}", tree);
    println!("Количество элементов: {}", tree.len());
    println!("Есть ли 1? {}", tree.has(&1));
    println!("Есть ли 4? {}", tree.has(&4));
}