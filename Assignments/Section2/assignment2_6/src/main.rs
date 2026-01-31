// Assignment 2.6: Plugin Architecture

trait Plugin {
    fn run(&self);
}

struct AudioPlugin {
    name: String,
}

impl Plugin for AudioPlugin {
    fn run(&self) {
        println!("Playing audio: {}", self.name);
    }
}

struct VideoPlugin {
    brightness: u8,
}

impl Plugin for VideoPlugin {
    fn run(&self) {
        println!("Rendering video with brightness: {}", self.brightness);
    }
}

fn execute_plugin<T: Plugin>(plugin: T) {
    plugin.run();
}

fn main() {
    let ap = AudioPlugin {
        name: String::from("Modern Wisdom"),
    };
    let vp = VideoPlugin { brightness: 100 };

    execute_plugin(ap);
    execute_plugin(vp);

    // Compiler error: error[E0277]: the trait bound `String: Plugin` is not satisfied
    // let test = String::from("Hello");
    // execute_plugin(test);
}
