use juniper::{Context, FieldResult, RootNode, ID};

#[derive(juniper::GraphQLObject)]
#[graphql(description = "A todo entry")]
struct Todo {
    #[graphql(description = "The todo's database ID")]
    _id: ID,
    #[graphql(description = "The todo's title")]
    title: String,
    #[graphql(description = "The todo's completion status")]
    completed: bool,
}

#[derive(Debug)]
pub struct Database {
    pub todos: String,
}

struct NestedTodo;

#[juniper::object(
    Context = Database,
)]
impl NestedTodo {
    #[graphql(description = "Get the nested todos")]
    fn get(context: &Database, show: String) -> FieldResult<Vec<Todo>> {
        Ok(vec![Todo {
            _id: ID::new("1"),
            title: context.todos.clone(),
            completed: false,
        }])
    }

    fn remaining() -> FieldResult<i32> {
        Ok(0)
    }
}

impl Context for Database {}

#[derive(Debug)]
pub struct Queries;

#[juniper::object(
    Context = Database,
)]
impl Queries {
    fn get(context: &Database, show: String) -> FieldResult<Vec<Todo>> {
        Ok(vec![Todo {
            _id: ID::new("1"),
            title: context.todos.clone(),
            completed: false,
        }])
    }

    fn remaining() -> FieldResult<i32> {
        Ok(0)
    }

    fn NestedTodo() -> FieldResult<NestedTodo> {
        Ok(NestedTodo {})
    }
}

#[derive(Debug)]
pub struct Mutations;

#[juniper::object(
    Context = Database,
)]
impl Mutations {
    fn add(title: String) -> FieldResult<Todo> {
        Ok(Todo {
            _id: ID::new("1"),
            title: title,
            completed: false,
        })
    }

    fn update(id: ID, title: String) -> FieldResult<bool> {
        Ok(true)
    }

    fn remove(id: ID) -> FieldResult<bool> {
        Ok(true)
    }
}

pub type Schema = RootNode<'static, Queries, Mutations>;

pub fn create_schema() -> Schema {
    Schema::new(Queries {}, Mutations {})
}
