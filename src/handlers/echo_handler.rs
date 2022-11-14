pub struct EchoHandler {
    message: String,
}

impl EchoHandler {
    pub fn new(message: String) -> Self {
        Self { message }
    }

    pub fn run(&self) {
        println!("Echo -> {}", &self.message);
    }
}
