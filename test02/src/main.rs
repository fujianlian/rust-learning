mod manage_system;

fn main() {
    let mut system = manage_system::manage::StudentManagementSystem::new();

    system.create_student(1, "Alice".to_string());
    system.create_student(2, "Bob".to_string());
    system.create_student(3, "Tom".to_string());

    system.create_class(101, "Class One".to_string());
    system.create_class(102, "Class Two".to_string());
    system.create_class(103, "Class Three".to_string());

    system.create_course(401, "Math".to_string());
    system.create_course(402, "History".to_string());
    system.create_course(403, "English".to_string());

    system.create_club(201, "Basketbacll".to_string());
    system.create_club(202, "Volleyball".to_string());
    system.create_club(203, "Pingpong".to_string());

    system.add_student_to_class(1, 102);
    system.add_student_to_class(2, 103);
    system.add_student_to_class(3, 101);

    system.add_student_to_club(2, 203);
    system.add_student_to_club(3, 202);
    system.add_student_to_club(1, 201);

    system.list_students();
    println!("--------------");

    system.list_courses();
    println!("--------------");

    system.list_classes();
    println!("--------------");

    system.list_clubs();

    println!("--------------");
    system.remove_student_from_club(1, 201);
    system.list_clubs();


} 