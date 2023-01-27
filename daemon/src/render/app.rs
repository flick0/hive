use gdk;
use gtk::prelude::*;

use crate::render::window;

pub fn init_window(hivewindow:&window::HiveWindow, monitor: &gdk::Rectangle) -> Option<gtk::Window> {
    let window = gtk::Window::new(gtk::WindowType::Toplevel);
    gtk_layer_shell::init_for_window(&window);

    window.set_resizable(hivewindow.resizable);
    gtk_layer_shell::set_layer(&window, hivewindow.stacking);

    gtk_layer_shell::set_keyboard_interactivity(&window, hivewindow.focusable);

    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Left, hivewindow.position.anchor.left);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Right, hivewindow.position.anchor.right);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Top, hivewindow.position.anchor.top);
    gtk_layer_shell::set_anchor(&window, gtk_layer_shell::Edge::Bottom, hivewindow.position.anchor.bottom);

    let xoffset = hivewindow.position.offset_x.pixels_relative_to(monitor.width());
    let yoffset = hivewindow.position.offset_y.pixels_relative_to(monitor.height());

    if hivewindow.position.anchor.left {
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Left, xoffset);
    } else {
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Right, xoffset);
    }
    if hivewindow.position.anchor.bottom {
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Bottom, yoffset);
    } else {
        gtk_layer_shell::set_margin(&window, gtk_layer_shell::Edge::Top, yoffset);
    }

    if hivewindow.reserve_space {
        gtk_layer_shell::auto_exclusive_zone_enable(&window);
    }
    Some(window)
}