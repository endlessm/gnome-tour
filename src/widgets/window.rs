use gtk::prelude::*;

use super::headerbar::HeaderBar;
use super::pages::{ImagePageWidget, WelcomePageWidget};
use super::paginator::PaginatorWidget;
use crate::config::PROFILE;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
    container: gtk::Stack,
    headerbar: HeaderBar,
    paginator: PaginatorWidget,
}

impl Window {
    pub fn new(app: &gtk::Application) -> Self {
        let widget = gtk::ApplicationWindow::new(app);
        let container = gtk::Stack::new();
        let headerbar = HeaderBar::new();
        let paginator = PaginatorWidget::new();

        let mut window_widget = Window {
            widget,
            container,
            headerbar,
            paginator,
        };

        window_widget.init();
        window_widget
    }

    pub fn start_tour(&self) {
        if let Some(page) = self.paginator.get_current_page() {
            self.headerbar.set_page_title(&page.get_title());
        }
        self.container.set_visible_child_name("pages");
        self.headerbar.start_tour();
    }

    fn end_tour(&self) {
        self.container.set_visible_child_name("welcome");
        self.headerbar.end_tour();
    }

    pub fn next_page(&self) {
        let total_pages = self.paginator.get_total_pages();
        let current_page = self.paginator.get_current_page_nr();
        self.headerbar.set_page_nr(current_page + 1, total_pages);

        if current_page == total_pages {
            self.widget.destroy();
        } else {
            self.paginator.next();
        }
        if let Some(page) = self.paginator.get_current_page() {
            self.headerbar.set_page_title(&page.get_title());
        }
    }

    pub fn previous_page(&self) {
        let total_pages = self.paginator.get_total_pages();
        let current_page = self.paginator.get_current_page_nr();
        self.headerbar.set_page_nr(current_page - 1, total_pages);

        match current_page {
            1 => self.end_tour(),
            _ => self.paginator.previous(),
        }

        if let Some(page) = self.paginator.get_current_page() {
            self.headerbar.set_page_title(&page.get_title());
        }
    }

    fn init(&mut self) {
        self.widget.set_default_size(920, 640);
        self.container.set_transition_type(gtk::StackTransitionType::SlideLeftRight);
        self.container.set_transition_duration(300);

        // Devel Profile
        if PROFILE == "Devel" {
            self.widget.get_style_context().add_class("devel");
        }

        self.widget.set_titlebar(Some(&self.headerbar.widget));

        let welcome_page = WelcomePageWidget::new();
        self.container.add_named(&welcome_page.widget, "welcome");

        self.paginator.add_page(Box::new(ImagePageWidget::new(
            "/org/gnome/Tour/activities.svg",
            "Activities Overview",
            "Open Activities to start apps",
            "You can also view open windows, search and use workspaces.",
        )));

        self.paginator.add_page(Box::new(ImagePageWidget::new(
            "/org/gnome/Tour/search.svg",
            "Search",
            "In the Activities Overview, just start typing to search",
            "Search can be used to launch apps, find settings, do calculations and much more.",
        )));

        self.paginator.add_page(Box::new(ImagePageWidget::new(
            "/org/gnome/Tour/calendar.svg",
            "Date & Time",
            "Click the time to see your now and next",
            "This includes notifications, media controls, calendar events, the weather and world clocks.",
        )));

        self.paginator.add_page(Box::new(ImagePageWidget::new(
            "/org/gnome/Tour/status-menu.svg",
            "System Menu",
            "View system information and settings",
            "Get an overview of the system status and quickly change settings.",
        )));
        self.paginator.add_page(Box::new(ImagePageWidget::new(
            "/org/gnome/Tour/software.svg",
            "Software",
            "Find and install apps",
            "The Software app makese it easy to find and install all the apps you need.",
        )));

        self.paginator.add_page(Box::new(ImagePageWidget::new(
            "/org/gnome/Tour/help.svg",
            "Learn More",
            "That's it! To learn more, see the Help",
            "The help app contains information, tips and tricks.",
        )));

        self.container.add_named(&self.paginator.widget, "pages");
        self.widget.add(&self.container);
    }
}
