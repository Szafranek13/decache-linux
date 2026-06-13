use gtk::prelude::*;
use gtk::{Application, Builder, ApplicationWindow};

fn main() {
    let app = Application::builder()
        .application_id("com.example.app")
        .build();

    app.connect_activate(|app| {
        let builder = Builder::from_file("ui.glade");

        let window: ApplicationWindow = builder
            .object("main_window")
            .expect("Couldn't find main_window");
        
        let window_clone: ApplicationWindow = window.clone();

        let quit_button: gtk::Button = builder.object("quit_button").unwrap();
        

        quit_button.connect_clicked(move |_| {
            window_clone.close();
        });

        window.set_application(Some(app));
        window.show(); // or window.present()
    });

    app.run();
}
