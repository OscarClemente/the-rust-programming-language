pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing button {}", self.label);
    }
}

pub struct TextArea {
    pub width: u32,
    pub height: u32,
    pub text: String,
    pub name: String, 
}

impl Draw for TextArea {
    fn draw(&self) {
        println!("Drawing text area {}", self.name);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 3,
                label: String::from("login"),
            }),
            Box::new(TextArea {
                width: 30,
                height: 20,
                text: String::from("Introduce your comment"),
                name: String::from("Comment"),
            })
        ],
    };

    screen.run();
}
