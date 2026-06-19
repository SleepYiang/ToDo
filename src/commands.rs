use crate::models::Todo;
//公共打印函数
fn print_todo(todo:&Todo)
{
    let status =
        if todo.completed
        {
            "[X]已完成"
        }
        else
        {
            "[ ]未完成"
        };

    println!(
        "{:<5} {:<10} {}",
        todo.id,
        status,
        todo.title
    );
}

//完成
pub fn done(todos:&mut Vec<Todo>,id:u32)
{
    let pos=todos.iter_mut().find(|t|t.id==id);
    match pos{
        Some(todo)=>{
            todo.completed=!todo.completed;
           
        }
        None=>{
            println!("未找到相关内容");
        }
    }
}

//显示
pub fn list(todos:&[Todo])
{
    if todos.is_empty(){
        println!("当前没有任务");
        return;
    }
    println!("{:<5} {:<12} {}", "ID", "STATUS", "TITLE");
    println!("--------------------------------");
    for todo in todos{
      print_todo(todo);
    }
}

//添加
pub fn add(todos:&mut Vec<Todo>,id:u32,title:String)
{
    let new_todo=Todo{
        id,
        title,
        completed:false
    };
    todos.push(new_todo);
    println!("新增任务成功，id:{}", id);
}


//删除
pub fn delete(todos:&mut Vec<Todo>,id:u32)
{
    let pos=todos.iter().position(|t|t.id==id);
    if let Some(indx)=pos{
        todos.remove(indx);
        println!("已删除{}的任务",id);
    }else{
        println!("未找到{}的任务",id);
    }
}
//清除
pub fn clear(todos:&mut Vec<Todo>)
{
    todos.retain(|todo|!todo.completed);
    println!("=== 已清空完成任务 ===");
}


//查找
pub fn search(todos:&[Todo],keyword:&str)
{
    let result_list:Vec<&Todo>=todos.iter()
        .filter(|todo|todo.title.contains(keyword))
        .collect();
    if result_list.is_empty(){
        println!("未找到相关内容")
    }else{
        println!("===== 关键词「{}」搜索结果 =====", keyword);
        for todo in result_list {
            print_todo(todo);
        }
    }
}

//编辑
pub fn edit(todos:&mut Vec<Todo>,
        id:u32,
        new_title:&str)
{
      
    let pos=todos.iter_mut().find(|t|t.id==id);
    if let Some(todo)=pos{
        todo.title=new_title.to_string();
         println!("修改成功，id{}标题已更新", id);
    }
    else{
        println!("未找到id={}的任务，修改失败", id);
    }
    
}

