use std::collections::HashMap;

struct Employee {
    name: String,
    department: String,
}

fn main() {
    let mut departments: HashMap<String, Vec<String>> = HashMap::new();
    add_employee(
        Employee {
            name: String::from("Mey"),
            department: String::from("研发部"),
        },
        &mut departments,
    );
    add_employee(
        Employee {
            name: String::from("Jack"),
            department: String::from("研发部"),
        },
        &mut departments,
    );
    add_employee(
        Employee {
            name: String::from("Kevin"),
            department: String::from("市场部"),
        },
        &mut departments,
    );
    add_employee(
        Employee {
            name: String::from("Mike"),
            department: String::from("人事部"),
        },
        &mut departments,
    );
    get_employees(String::from("研发部"), &departments);
    get_employees(String::from("市场部"), &departments);
    get_employees(String::from("人事部"), &departments);
    get_employees(String::from("投资部"), &departments);
}

fn add_employee(employee: Employee, company: &mut HashMap<String, Vec<String>>) {
    let department = company.entry(employee.department).or_insert(Vec::new());
    if !department.contains(&employee.name) {
        department.push(employee.name);
    }
}

fn get_employees(department: String, company: &HashMap<String, Vec<String>>) {
    match company.get(&department) {
        Some(employees) => {
            print!("{}: ", department);
            for e in employees {
                print!("{}, ", e);
            }
            println!();
        }
        None => {
            println!("This department has no employee");
        }
    }
}
