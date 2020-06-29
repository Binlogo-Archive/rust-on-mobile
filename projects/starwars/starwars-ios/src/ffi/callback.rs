use starwars_core::swapi::{People, SwapiCallback};

pub struct Callback {
    pub on_success: Box<dyn FnMut(Vec<People>)>,
    pub on_error: Box<dyn FnMut(String)>,
}

#[allow(non_snake_case)]
impl SwapiCallback for Callback {
    fn onLoad(&mut self, res: Vec<People>) {
        (self.on_success)(res)
    }
    fn onError(&mut self, err: &str) {
        (self.on_error)(err.to_owned());
    }
}

unsafe impl Send for Callback {}
