#[derive(Debug)]
struct Todo{
    id:u32,
    title:String,
    completed:bool
}
//显示
fn list(todos:&[Todo]){
       if todos.is_empty() {
        println!("当前没有任务");
        return;
    }
    println!("{:<5} {:<12} {}", "ID", "STATUS", "TITLE");
    println!("--------------------------------");
    for todo in todos{
        let status =if todo.completed{
            "[X]已完成"
        }else{
            "[]未完成"
        };
    println!("{:<5} {:<10} {}",
    todo.id,
    status,
    todo.title);
    }
    
}

//删除
fn delete(todos:&mut Vec<Todo>,
          id:u32)
    {
    let pos=todos.iter().position(|t|t.id==id);
    if let Some(idx)=pos{
        todos.remove(idx);
        println!("已删除{}的任务",id);
    }else{
        println!("未找到{}的任务",id);
    }
}
//完成
fn done(todos:&mut Vec<Todo>,
        id:u32)
    {
    let result=todos.iter_mut().find(|t|t.id==id);
    match result{
        Some(i)=>{
            println!("找到任务");
            i.completed=!i.completed;
            println!("更改后的内容{:?}",i)
        }
        None=>{
            println!("未找到任务");
        }
    }
}

//修改
fn edit(todos:&mut Vec<Todo>,
        id:u32,
        new_title:String)
    {
        let pos=todos.iter_mut().find(|t|t.id==id);
        if let Some(todo)=pos{
            todo.title=new_title;
             println!("修改成功，id{}标题已更新", id);
        }
        else{
             println!("未找到id={}的任务，修改失败", id);
        }
    }

//清空
fn clear(todos:&mut Vec<Todo>)
{
    todos.retain(|t|!t.completed);
    println!("=== 已清空完成任务 ===");
}

//查找
fn search(todos: &[Todo], keyword: &str) {
    // 过滤出标题包含关键词的所有Todo
    let result_list: Vec<&Todo> = todos.iter()
        .filter(|todo| todo.title.contains(keyword))
        .collect();

    if result_list.is_empty() {
        println!("未找到包含「{}」的任务", keyword);
    } else {
        println!("===== 关键词「{}」搜索结果 =====", keyword);
        for todo in result_list {
            let status = if todo.completed { "[X]已完成" } else { "[]未完成" };
            println!("{:<5} {:<10} {}", todo.id, status, todo.title);
        }
    }
} 

//添加任务
fn add(todos:&mut Vec<Todo>,id:u32,title:String)
{
    let new_todo=Todo{
        id,
        title,
        completed:false
    };
    todos.push(new_todo);
    println!("新增任务成功，id:{}", id);
}
   
fn main(){
    // let todo1=Todo{
    // id:1,
    // title:String::from("学习 rust"),
    // completed:false
    // };
    
    // println!("{:?}",todo1);
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
    //循环遍历打印出来
    // for i in &todos{
    //     //println!("{:?}",i)
    //     }
    // let result=todos.iter().find(|t|t.id==2);
    // match result {
    //         Some(i)=> {
    //         println!("找到任务: {:?}", i);
    //     }
    //         None=> {
    //             println!("未找到任务");
    //         }
    //     }
   

    list(&todos);
    delete(&mut todos,2);
    list(&todos);
    done(&mut todos, 1);
    edit(&mut todos, 3, String::from("test"));
    list(&todos);
}

