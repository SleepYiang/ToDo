mod models;
mod commands;
use models::Todo;
use commands::*;   
fn main(){

    let mut todos: Vec<Todo> = Vec::new();
    todos.push(Todo { 
        id: 1, 
        title: "study".to_string(), 
        completed:false 
    });
    todos.push(Todo { 
        id: 2, 
        title: "锻炼".to_string(), 
        completed: false 
    });
     todos.push(Todo { 
        id: 3, 
        title: "RushB".to_string(), 
        completed: false 
    });

    list(&todos);
    //delete(&mut todos,2);
    //list(&todos);
    done(&mut todos, 1);
    edit(&mut todos, 3, "test");
    list(&todos);
    add(&mut todos,4,"学习".to_string());
    list(&todos);
    delete(&mut todos, 2);
    list(&todos);
    search(&todos, "学习");

}

