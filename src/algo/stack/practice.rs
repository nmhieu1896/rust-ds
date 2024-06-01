use std::collections::VecDeque;

pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut students = VecDeque::from(students);
    let mut sandwiches = VecDeque::from(sandwiches);

    let mut served_students = 0;
    while !students.is_empty() && served_students < students.len() {
        if students[0] == sandwiches[0] {
            students.pop_front();
            sandwiches.pop_front();
            served_students = 0;
        } else {
            let head_student = students.pop_front().unwrap();
            students.push_back(head_student);
            served_students += 1;
        }
    }
    return students.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test1() {
        let students = vec![1, 1, 0, 0];
        let sandwiches = vec![0, 1, 0, 1];
        assert_eq!(count_students(students, sandwiches), 0);
    }

    #[test]
    pub fn test2() {
        let students = vec![1, 1, 1, 0, 0, 1];
        let sandwiches = vec![1, 0, 0, 0, 1, 1];
        assert_eq!(count_students(students, sandwiches), 3);
    }
}
