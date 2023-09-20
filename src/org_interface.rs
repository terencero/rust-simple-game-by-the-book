use std::{collections::HashMap, io, vec};
// text based organization interface to CRUD org directory

// entry - reads text based input to perform one of CRUD
pub fn init_org_app() {
    let mut dept_map: HashMap<String, Vec<String>> = HashMap::new();
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

        let all_members = get_all_members(dept_name, dept_map);

        println!("all members -> {:?}", all_members);

        if all_members.is_empty() {
            println!("There are no members in this department! \n
                Would you like to add member(s)? [y/n]
            ");

            let mut answer = String::new();

            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read your answer...");

            let answer = answer.trim().to_lowercase();

            if answer == "y" {
                // take in names
            } else if answer == "n" {
                // return empty vec
            } else {
                // prompt user to answer with y/n
            }
        }
    }
}
// create a department
// fn create_department(dept_name: &str, mut dept: HashMap<&str, Vec<&str>>) {
//     dept.entry(dept_name).;
// }

// read all members of a department
// don't understand lifetime params yet... just following the compiler recommendations  
fn get_all_members(
    dept_name: String, 
    mut dept: HashMap<String, Vec<String>>,
) -> HashMap<String, Vec<String>> {
    dept.entry(dept_name).or_insert(vec!["aa".to_string()]);
    dept
}


// read all members of all departments sorted by department

// add a member to a department
fn add_members_to_department(
    dept_name: String,
    mut dept: HashMap<String, Vec<String>>,
    names: Vec<String>,
) -> HashMap<String, Vec<String>> {
    // name of dept
    // reference to department hash map
    // name or names of people to add, keep it simple first name only for now

    // add name(s) to hash map
    dept.entry(dept_name).or_insert(names);

    dept
    // return hash map
}

// read a member of a department

// read a member and return details of department

// remove member from department

// move member from one department to another
