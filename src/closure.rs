#[derive(Default)]
pub struct Todo {
    pub id: i16,
    pub deleted: bool,
    pub completed: bool,
}

pub fn with_todo_id<P>(todos: &mut Vec<Todo>, todo_id: i16, f: P) where P: Fn(&mut Todo) {
    if let Some(todo) = todos.iter_mut().find(|todo| todo.id == todo_id) {
        f(todo);
    }
}

pub fn remove_todo(todos: &mut Vec<Todo>, todo_id: i16) {
    with_todo_id(todos, todo_id, |todo| todo.deleted = true);
}

pub fn mark_done(todos: &mut Vec<Todo>, todo_id: i16) {
    with_todo_id(todos, todo_id, |todo| todo.completed = true);
}
