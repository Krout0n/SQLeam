#[derive(Debug)]
pub struct Table {
    col: Vec<User>,
}

#[derive(Debug)]
pub struct User {
    pub id: usize,
    pub name: &'static str,
}

impl User {
    pub fn new(id: usize, name: &'static str) -> Self {
        Self { id, name }
    }
}

impl Table {
    pub fn new() -> Self {
        Self {
            col: vec![User::new(1, "kuru"), User::new(2, "ton")],
        }
    }

    pub fn command(&mut self, name: &str) {
        match name {
            "select" => {
                dbg!(&self);
                ()
            }
            "insert" => {
                self.col.push(User::new(self.col.len() + 1, "inserted"));
                dbg!(&self);
            }
            _ => unimplemented!(),
        };
    }
}
