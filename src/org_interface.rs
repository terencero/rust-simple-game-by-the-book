use std::{collections::HashMap, io, vec};
// text based organization interface to CRUD org directory

// entry - reads text based input to perform one of CRUD
pub fn init_org_app() {
    let mut dept_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut answer = String::new();

    println!("[1] To list all members of a department.");

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read your answer...");

    let answer = answer.trim();

    if answer == "1" {
        let mut dept_name = String::new();

        println!("What is the name of the department?");
        io::stdin()
            .read_line(&mut dept_name)
            .expect("Failed to read message.");

        let all_members = get_all_members(&dept_name, &mut dept_map);

        println!("all members -> {:?}", all_members);
    }
}
// create a department
// fn create_department(dept_name: &str, mut dept: HashMap<&str, Vec<&str>>) {
//     dept.entry(dept_name).;
// }

// read all members of a department
// don't understand lifetime params yet... just following the compiler recommendations  
fn get_all_members<'a>(
    dept_name: &'a str, 
    dept: &'a mut HashMap<&'a str, Vec<&'a str>>,
) -> &'a mut Vec<&'a str> {
    let default_param = vec!["no members"];

    let res = dept.entry(dept_name).or_insert(default_param);
    res
}


// read all members of all departments sorted by department

// add a member to a department

// read a member of a department

// read a member and return details of department

// remove member from department

// move member from one department to another
