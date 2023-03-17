use std::collections::BTreeMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
#[derive(Clone)]
pub struct School {
    grades: BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        Self {
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.grades
            .entry(grade)
            .and_modify(|grade| grade.push(student.to_string()))
            .or_insert(vec![student.to_string()]);
        self.grades.entry(grade).and_modify(|grade| grade.sort());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.grades.keys().copied().collect()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let g = self.grades.get(&grade);
        if !g.is_none() {
            g.unwrap().to_owned()
        } else {
            Vec::new()
        }
    }
}
