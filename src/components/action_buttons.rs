use super::{browser::Browser, button};
use glib::clone;
use gtk::prelude::{BoxExt, ButtonExt};
use std::cell::Cell;
use std::rc::Rc;
use webkit2gtk::{LoadEvent, WebViewExt};

pub struct ActionButtons {
    container: Rc<gtk::Box>,
    previous_button: Rc<button::WebViewButton>,
    next_button: Rc<button::WebViewButton>,
    refresh_button: Rc<button::WebViewButton>,
    home_button: Rc<button::WebViewButton>,
    is_refreshing: Rc<Cell<bool>>,
}

impl ActionButtons {
    pub fn new(_css_provider: Rc<gtk::CssProvider>) -> Self {
        let previous_button = Rc::new(button::WebViewButton::new(Some("go-previous")));
        let next_button = Rc::new(button::WebViewButton::new(Some("go-next")));
        let refresh_button = Rc::new(button::WebViewButton::new(Some("view-refresh")));
        let home_button = Rc::new(button::WebViewButton::new(Some("go-home")));

        let hbox = Rc::new(gtk::Box::new(gtk::Orientation::Horizontal, 0));
        hbox.pack_start(&previous_button.button, false, false, 4);
        hbox.pack_start(&next_button.button, false, false, 4);
        hbox.pack_start(&refresh_button.button, false, false, 4);
        hbox.pack_start(&home_button.button, false, false, 4);

        ActionButtons {
            is_refreshing: Rc::new(Cell::new(false)),
            previous_button,
            next_button,
            refresh_button,
            home_button,
            container: hbox,
        }
    }

    pub fn get_widget(&self) -> Rc<gtk::Box> {
        Rc::clone(&self.container)
    }
    pub fn connect_action_with_browser(&self, browser: Rc<Browser>) {
        // Connect the button to the browser

        self.previous_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                browser.get_widget().go_back();
            }));
        // Connect the button to the browser
        self.next_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                browser.get_widget().go_forward();
            }));

        let is_refreshing = Rc::clone(&self.is_refreshing);
        self.refresh_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                if is_refreshing.get() {
                    browser.get_widget().stop_loading();
                } else {
                    browser.get_widget().reload();
                }
            }));

        self.home_button
            .button
            .connect_clicked(clone!(@strong browser => move |_| {
                browser.load_about_pages(&"about:home".to_string());
            }));

        let refresh_button = Rc::clone(&self.refresh_button);
        let is_refreshing = Rc::clone(&self.is_refreshing);
        browser.get_widget().connect_load_changed(
            move |_, load_event: LoadEvent| match load_event {
                LoadEvent::Started => {
                    refresh_button.set_icon_name("process-stop");
                    is_refreshing.set(true);
                }
                LoadEvent::Finished => {
                    refresh_button.set_icon_name("view-refresh");
                    is_refreshing.set(false);
                }
                _ => {}
            },
        );
    }
}
