#[derive(Debug)]
struct Todo{
    id:u32,
    title:String,
    completed:bool
}
//显示
fn list(todos:&[Todo]){
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
fn delete(todo:&mut Vec<Todo>,
          id:u32)
    {
    let new_id=id;
    let pos=todo.iter().position(|t|t.id==new_id);
    if let Some(idx)=pos{
        todo.remove(idx);
        println!("已删除id=2的任务");
    }else{
        println!("未找到id=2的任务");
    }
}
//完成
fn done(todo:&mut Vec<Todo>,
        id:u32)
    {
    let new_id=id;
    let result=todo.iter_mut().find(|t|t.id==new_id);
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
fn edit(todo:&mut Vec<Todo>,
        id:u32,
        new_title:String)
    {
        let idp=id;
        let pos=todo.iter_mut().find(|t|t.id==idp);
        if let Some(todo)=pos{
            todo.title=new_title;
             println!("修改成功，id{}标题已更新", id);
        }
        else{
             println!("未找到id={}的任务，修改失败", id);
        }
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
    done(&mut todos,2);

}

