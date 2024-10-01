fn main() {
    println!("Hello, world!");
}

use std::cell::RefCell;
use std::rc::Rc;

// データベースのトレイトを定義
trait Database {
    fn get_user(&self, id: u32) -> Option<String>;
}

// 実際のデータベース実装
struct RealDatabase;

impl Database for RealDatabase {
    fn get_user(&self, id: u32) -> Option<String> {
        // 実際のデータベース操作をここに実装
        Some(format!("User {}", id))
    }
}

// テスト対象の関数
fn process_user(db: &dyn Database, user_id: u32) -> String {
    match db.get_user(user_id) {
        Some(name) => format!("User found: {}", name),
        None => "User not found".to_string(),
    }
}

// テスト用のスタブ作成ヘルパー
struct DatabaseStub {
    get_user: Rc<RefCell<Box<dyn Fn(u32) -> Option<String>>>>,
}

impl Database for DatabaseStub {
    fn get_user(&self, id: u32) -> Option<String> {
        (self.get_user.borrow())(id)
    }
}

impl DatabaseStub {
    fn new() -> Self {
        DatabaseStub {
            get_user: Rc::new(RefCell::new(Box::new(|_| None))),
        }
    }

    fn stub_get_user<F>(&mut self, stub: F)
    where
        F: Fn(u32) -> Option<String> + 'static,
    {
        *self.get_user.borrow_mut() = Box::new(stub);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_user_found() {
        let mut db_stub = DatabaseStub::new();

        // RSpecの `allow(...).to receive(...).and_return(...)` に相当
        db_stub.stub_get_user(|id| {
            if id == 1 {
                Some("Alice".to_string())
            } else {
                None
            }
        });

        let result = process_user(&db_stub, 1);
        assert_eq!(result, "User found: Alice");
    }

    #[test]
    fn test_process_user_not_found() {
        let mut db_stub = DatabaseStub::new();

        // 全てのIDに対して None を返すスタブ
        db_stub.stub_get_user(|_| None);

        let result = process_user(&db_stub, 2);
        assert_eq!(result, "User not found");
    }
}
