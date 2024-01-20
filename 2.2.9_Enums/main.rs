enum Role {
    student(String),
    teacher(String),
    admin(String),
}

fn main() {
    let student = Role::student;
    let teacher = Role::teacher;
    let admin = Role::admin;
}
