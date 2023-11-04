use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Student {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Class {
    id: u32,
    name: String,
    students: Vec<Student>,
}

#[derive(Debug)]
struct Course {
    id: u32,
    name: String,
}

#[derive(Debug)]
struct Club {
    id: u32,
    name: String,
    members: Vec<Student>,
}

pub struct StudentManagementSystem {
    students: HashMap<u32, Student>,
    classes: HashMap<u32, Class>,
    courses: HashMap<u32, Course>,
    clubs: HashMap<u32, Club>,
}

impl StudentManagementSystem {
    pub fn new() -> StudentManagementSystem {
        StudentManagementSystem {
            students: HashMap::new(),
            classes: HashMap::new(),
            courses: HashMap::new(),
            clubs: HashMap::new(),
        }
    }

    pub fn create_student(&mut self, id: u32, name: String) {
        let student = Student { id, name };
        self.students.insert(id, student);
    }

    pub fn create_class(&mut self, id: u32, name: String) {
        let class = Class { id, name, students: vec![] };
        self.classes.insert(id, class);
    }

    pub fn create_course(&mut self, id: u32, name: String) {
        let course = Course { id, name };
        self.courses.insert(id, course);
    }

    pub fn create_club(&mut self, id: u32, name: String) {
        let club = Club { id, name, members: vec![] };
        self.clubs.insert(id, club);
    }

    pub fn add_student_to_class(&mut self, student_id: u32, class_id: u32) {
        if let Some(student) = self.students.get(&student_id) {
            if let Some(class) = self.classes.get_mut(&class_id) {
                class.students.push(student.clone());
            }
        }
    }

    pub fn add_student_to_club(&mut self, student_id: u32, club_id: u32) {
        if let Some(student) = self.students.get(&student_id) {
            if let Some(club) = self.clubs.get_mut(&club_id) {
                club.members.push(student.clone());
            }
        }
    }

    pub fn remove_student_from_club(&mut self, student_id: u32, club_id: u32) {
        if let Some(club) = self.clubs.get_mut(&club_id) {
            club.members.retain(|s| s.id != student_id);
        }
    }

    pub fn list_students(&self) {
        for student in self.students.values() {
            println!("{:?}", student);
        }
    }

    pub fn list_classes(&self) {
        for class in self.classes.values() {
            println!("{:?}", class);
        }
    }

    pub fn list_clubs(&self) {
        for club in self.clubs.values() {
            println!("{:?}", club);
        }
    }

    pub fn list_courses(&self) {
        for courses in self.courses.values() {
            println!("{:?}", courses);
        }
    }
}
