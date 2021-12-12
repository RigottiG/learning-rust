// Topic: Result & the question mark operator

enum Position {
    Maintenance,
    Marketing,
    Manager,
    LineSupervisors,
    KitchenStaff,
    AssemblyTechnicians,
}

enum Status {
    Active,
    Terminated,
}

struct Employee {
    position: Position,
    status: Status,
}

fn try_access(employee: &Employee) -> Result<(), String> {
    match employee.status {
        Status::Terminated => return Err("terminated".to_owned()),
        _ => (),
    }

    match employee.position {
        Position::Maintenance => Ok(()),
        Position::Marketing => Ok(()),
        Position::Manager => Ok(()),
        _ => Err("invalid position".to_owned()),
    }
}

fn print_access(employee: &Employee) -> Result<(), String> {
    try_access(employee)?;
    println!("access ok");
    Ok(())
}

fn main() {
    let manager = Employee {
        position: Position::Manager,
        status: Status::Active,
    };

    let line_supervisor = Employee {
        position: Position::LineSupervisors,
        status: Status::Terminated,
    };

    match print_access(&manager) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }

    match print_access(&line_supervisor) {
        Err(e) => println!("access denied: {:?}", e),
        _ => (),
    }
}
