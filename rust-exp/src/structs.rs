pub fn run() {
    let mut mark = Marks {
        language: 60,
        maths: 80,
        science: 90,
    };

    mark.science = 100;

    let stud = Student::new_student("Ranjith", "Sekar", "Chennai");

    println!(
        "Student name: {}, Address: {}, Language: {}, Maths:{}, Science: {}",
        stud.full_name(), stud.address, mark.language, mark.maths, mark.science
    );

    
}

struct Marks {
    language: i32,
    maths: i32,
    science: i32,
}

struct Student {
    first_name: String,
    last_name: String,
    address: String,
}

impl Student {
    fn new_student(fname: &str, lname: &str, in_addr: &str) -> Student {
        Student {
            first_name: fname.to_string(),
            last_name: lname.to_string(),
            address: in_addr.to_string(),
        }
    }

    fn full_name(&self) -> String {
        return format!("{} {}", self.first_name, self.last_name);
    }
}
