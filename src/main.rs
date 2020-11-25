use std::env;
// structure

struct TodoItem {
    name: String,
    sname: String,
    completed: char
}

// implement structure of TodoItem
impl TodoItem{
    fn new(name:String,sname:String)->TodoItem{
        return TodoItem{
            name: name,
            sname: sname,
            completed:' '
        }
    }
}

struct TodoList{
    list:Vec<TodoItem>
}

// initialised new empty list
impl TodoList{
    fn new() -> TodoList{
        return TodoList{
            list: Vec::new()
        };
    }

    fn add_in_list(&mut self, name:String, sname:String){
        //create new todo item and add into new variable
        let todo_item = TodoItem::new(name,sname);
        self.list.push(todo_item);
    }

    fn display(&self){
        for item in &self.list {
            println!("[{}] - First Name- {} \nLast Name- {}", item.completed, item.name, item.sname);
        }
    }

    fn mark_done(&mut self,index: usize){
        self.list[index].completed = 'x'
    }

    fn delete_task(&mut self, index:usize){
        self.list.remove(index);
    }
}

fn main() {
    let arguments: Vec<String> = env::args().collect();
    let command = arguments[1].clone();


    // let todo_item = TodoItem{
    //     name: "Rohit Dhanke".to_string(),
    //     completed: ' '
    // };

    // passing the value to the function
    let mut todo_list = TodoList::new();
    // let todo_item_1 = TodoItem::new("1st Item inserted".to_string());
    // let todo_item_2 = TodoItem::new("2nd item inserted".to_string());
    // let todo_item_3 = TodoItem::new("3nd item inserted in vector".to_string());
    //let todo_list = vec![todo_item_1,todo_item_2,todo_item_3];

    // calling and passing value to function
    todo_list.add_in_list("Rohit".to_string(),"Dhanke".to_string());
    todo_list.add_in_list("2st Item inserted".to_string(),"Last name Item inserted".to_string());
    todo_list.add_in_list("3st Item inserted".to_string(),"Last name Item inserted".to_string());

    /* Get command is for getting the data from list*/
    if command == "get"{
        // for item in &todo_list.list {
        //     println!("[{}] - {}", item.completed, item.name);
        // }
        todo_list.display();

    }/* Add command is for adding the data in list*/
    else if command == "add"{
        // add first name as a task1
        let task1 = arguments[2].clone();
        // add first name as a task2
        let task2 = arguments[3].clone();
        // calling and passing value to function
        todo_list.add_in_list(task1,task2);
        // calling the display function
        todo_list.display();
    } else if command == "done"{
        let index = arguments[2].parse().expect("Error converting to integer");
        todo_list.mark_done(index);
        todo_list.display();
    }else if command == "remove"{
        let remove = arguments[2].parse().expect("Error parsing for remove");
        todo_list.delete_task(remove);
        todo_list.display();
    }
}
