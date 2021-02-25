use interactive::Management;

pub struct LinkedListManager {

}

impl LinkedListManager {
    pub fn new() -> Self {
        Self {
        }
    }
}

impl Management for LinkedListManager {
    fn assign_job(&mut self, cmd: &str, args: &[&str]) -> bool {
        false
    }
    fn gen_graph(&self) -> String {
        String::from("")
    }
}
