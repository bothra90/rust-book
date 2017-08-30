// Question:
// Using a hash map and vectors, create a text interface to allow a user to add
// employee names to a department in the company. For example, “Add Sally to
// Engineering” or “Add Amir to Sales”. Then let the user retrieve a list of all
// people in a department or all people in the company by department, sorted
// alphabetically.

use std::collections::HashMap;

pub struct Company {
    // Map from dept. to employees in the dept.
    pub departments: HashMap<String, Vec<String>>,
}

impl Company {
    pub fn add_to_dept(&mut self, dept: &str, name: &str) {
        let entry = self.departments.entry(String::from(dept)).or_insert(vec![]);
        entry.push(String::from(name));
    }

    pub fn get_all_from_dept(&self, dept: &str) -> Option<&Vec<String>> {
        if let Some(ref employees) = self.departments.get(dept) {
            Some(employees)
        } else {
            None
        }
    }

    pub fn get_all_sorted(&self) -> Vec<String> {
        let mut all = self.departments.values().fold(Vec::new(), |mut acc, x| {
            acc.extend_from_slice(&x);
            acc
        });
        all.sort();
        all
    }
}
