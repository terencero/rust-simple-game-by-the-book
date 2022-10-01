use rand::Rng;

pub fn randomize_names() {
    let mut names = vec!["Val", "Ben", "JVE", "Anil", "Sonika", "tRo", "Brandon", "Nadiya", "Andre"];

    print_name(&mut names);
}

fn print_name(names: &mut Vec<&str>) {
    let name = pull_name_from_hat(names);

    println!("{name}");

    if names.len() > 0 {
        print_name(names);
    }
}

fn pull_name_from_hat(names: &mut Vec<&str>) -> String {
    let random_index = get_random_index(names.len());
    let name = names.swap_remove(random_index);

    String::from(name)
}

fn get_random_index(range: usize) -> usize {
    let random_num = rand::thread_rng().gen_range(0..=range - 1);
    random_num
}