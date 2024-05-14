#![allow(unused_imports)]
use std::env::{current_dir, set_current_dir};
use std::fs::{create_dir_all, metadata, remove_dir_all};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;

use crate::metadata::Metadata;
use git2::Repository;
use gtk::subclass::scrolled_window;
use gtk::{glib, prelude::*};
use gtk::{Application, ApplicationWindow};

mod metadata;

const HYPRLAND_REPO: &str = "https://github.com/hyprwm/Hyprland.git";
const HYPRLOCK_REPO: &str = "https://github.com/hyprwm/hyprlock.git";
const HYPRIDLE_REPO: &str = "https://github.com/hyprwm/hypridle.git";
const HYPRPAPER_REPO: &str = "https://github.com/hyprwm/hyprpaper.git";
const HYPRPICKER_REPO: &str = "https://github.com/hyprwm/hyprpicker.git";
const HYPRCURSOR_REPO: &str = "https://github.com/hyprwm/hyprcursor.git";
const HYPRLANG_REPO: &str = "https://github.com/hyprwm/hyprlang.git";
const HYPRWAYLAND_SCANNER_REPO: &str = "https://github.com/hyprwm/hyprwayland-scanner.git";
const XDG_DESKTOP_PORTAL_HYPRLAND_REPO: &str = "https://github.com/hyprwm/xdg-desktop-portal-hyprland.git";

fn main() {
    let app = Application::builder()
        .application_id("org.hyprland.hyprupdate")
        .build();

    app.connect_activate(|app| {

        let data_path: String = get_data_path();

        if metadata(&data_path).is_err() {
            create_dir_all(&data_path).expect("Failed to create directory");
        }



        let meta = Metadata::new(&data_path);
        let meta1 = meta.clone();
        let meta2 = meta.clone();
        let meta3 = meta.clone();

                // We create the main window.
        let win = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(600)
            .title("Hyprupdate")
            .build();


        thread::sleep(std::time::Duration::from_millis(20));
        let terminal_output = gtk::TextView::new();
        terminal_output.set_editable(false);
        terminal_output.set_cursor_visible(false);
        terminal_output.set_wrap_mode(gtk::WrapMode::Word);
        terminal_output.set_vexpand(true);
        terminal_output.set_hexpand(true);
    
        let terminal_output1 = terminal_output.clone();
        let terminal_output2 = terminal_output.clone();
        let terminal_output3 = terminal_output.clone();
    
        let scrolled_window = gtk::ScrolledWindow::new(gtk::Adjustment::NONE, gtk::Adjustment::NONE);
        scrolled_window.set_vexpand(true);
        scrolled_window.set_hexpand(true);
        scrolled_window.set_min_content_height(400);
        scrolled_window.set_min_content_width(win.allocated_width() - 20);
        scrolled_window.set_policy(gtk::PolicyType::Never, gtk::PolicyType::Always);
        scrolled_window.set_child(Some(&terminal_output));
    
        let scrolled_window1 = scrolled_window.clone();
        let scrolled_window2 = scrolled_window.clone();
        let scrolled_window3 = scrolled_window.clone();
        let scrolled_window4 = scrolled_window.clone();
        
        update_terminal_output(scrolled_window, terminal_output, "Welcome to Hyprupdate\n".to_string());

        let mut software = Vec::new();

        let hyprland = gtk::CheckButton::with_label("Hyprland");

        let hyprlock = gtk::CheckButton::with_label("hyprlock");

        let hypridle = gtk::CheckButton::with_label("hypridle");

        let hyprlang = gtk::CheckButton::with_label("hyprlang");

        let hyprcursor = gtk::CheckButton::with_label("hyprcursor");

        let hyprpaper = gtk::CheckButton::with_label("hyprpaper");

        let hyprpicker = gtk::CheckButton::with_label("hyprpicker");

        let hyprwayland_scanner = gtk::CheckButton::with_label("hyprwayland-scanner");

        let xdg_desktop_portal_hyprland =
            gtk::CheckButton::with_label("xdg-desktop-portal-hyprland");

        software.push(hyprland.clone());
        software.push(hyprlock.clone());
        software.push(hypridle.clone());
        software.push(hyprlang.clone());
        software.push(hyprcursor.clone());
        software.push(hyprpaper.clone());
        software.push(hyprpicker.clone());
        software.push(hyprwayland_scanner.clone());
        software.push(xdg_desktop_portal_hyprland.clone());

        let software2 = software.clone();
        let software3 = software.clone();
        let software4 = software.clone();

        let hypr_software_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(20)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();
        
        let hypr_software_box1 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(20)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        let hypr_software_box2 = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(20)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        hypr_software_box.add(&hyprland);
        hypr_software_box.add(&hyprlock);
        hypr_software_box.add(&hypridle);

        hypr_software_box1.add(&hyprlang);
        hypr_software_box1.add(&hyprcursor);
        hypr_software_box1.add(&hyprpaper);

        hypr_software_box2.add(&hyprpicker);
        hypr_software_box2.add(&hyprwayland_scanner);
        hypr_software_box2.add(&xdg_desktop_portal_hyprland);

        let get_installed_btn = gtk::Button::with_label("Get Installed");
        get_installed_btn.connect_clicked(move |_| {
            for i in &software {
                let name = i.label().unwrap();
                if metadata(get_data_path() + name.as_str()).is_ok() {
                    meta3.set_installed(&name, true);
                    i.set_active(true);
                }else{
                    meta3.set_installed(&name, false);
                    i.set_active(false);
                }
            }
        });

        let update_btn = gtk::Button::with_label("Update Repositories");
        update_btn.connect_clicked(move |_| {
            for i in &software4 {
                if i.is_active() {
                    let name = i.label().unwrap();
                    set_current_dir(get_data_path() + name.as_str())
                        .expect("Failed to set current directory");

                    let command = std::process::Command::new("git")
                        .arg("pull")
                        .output()
                        .expect("failed to execute process");

                    let command = std::str::from_utf8(&command.stdout).unwrap();
                    if command.contains("Already up to date.") {
                        #[cfg(debug_assertions)]
                        println!("\x1b[34m{} up to date.\x1b[0m", name);

                        
                        update_terminal_output(scrolled_window1.clone(), terminal_output1.clone(), format!("{} up to date.", name));
                    } else {
                        #[cfg(debug_assertions)]
                        println!("\x1b[34mUpdated {}\x1b[0m", name);
                        update_terminal_output(scrolled_window1.clone(), terminal_output1.clone(), format!("Updated {}\n", name));
                        meta.set_to_compile(&name, true);
                    }
                }
            }
        });

        let install_selected_btn = gtk::Button::with_label("Install Selected");
        install_selected_btn.connect_clicked(move |_| {
            let data_dir = get_data_path();
            for i in &software2 {
                if i.is_active() {
                    if metadata(get_data_path() + i.label().unwrap().as_str()).is_err() {
                        #[cfg(debug_assertions)]
                        println!("\x1b[32mCloning: {}\x1b[0m", i.label().unwrap().as_str());

                        update_terminal_output(scrolled_window3.clone(), terminal_output3.clone(), format!("Cloning: {}\n", i.label().unwrap().as_str()));
                        let repo = Repository::clone(
                            which_repo(&i.label().unwrap().as_str()),
                            data_dir.clone() + i.label().unwrap().as_str(),
                        ).expect("Failed to clone");
                        for mut j in repo.submodules().unwrap(){
                            j.update(true, None).expect("Failed to update submodule");
                        }
                    }
                    meta1.set_installed(&i.label().unwrap().as_str(), true);
                    meta1.set_to_compile(&i.label().unwrap().as_str(), true);
                    compile(&i.label().unwrap().as_str(), meta1.clone(), scrolled_window3.clone(), terminal_output3.clone());
                }
            }
        });

        let uninstall_selected_btn = gtk::Button::with_label("Uninstall Selected");
        uninstall_selected_btn.connect_clicked(move |_| {
            for i in &software3 {
                if i.is_active() {
                    #[cfg(debug_assertions)]
                    println!("\x1b[31mUninstalling: {}\x1b[0m", i.label().unwrap().as_str());
                    update_terminal_output(scrolled_window2.clone(), terminal_output2.clone(), format!("::Uninstalling {}\n", i.label().unwrap().as_str()));
                    remove_dir_all(get_data_path() + i.label().unwrap().as_str())
                        .expect("Failed to remove directory");
                    meta2.set_installed(&i.label().unwrap().as_str(), false);
                }
            }
        });
        

        let box_up = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(20)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .build();

        let box_down = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(20)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .build();

        let main_box = gtk::Box::builder()
            .orientation(gtk::Orientation::Vertical)
            .spacing(20)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        let bar = gtk::HeaderBar::new();
        bar.set_title(Some("Hyprupdate"));
        bar.set_show_close_button(true);

        win.set_titlebar(Some(&bar));

        box_up.add(&scrolled_window4);
        let hyprbox = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(20)
            .margin_top(20)
            .margin_bottom(20)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .build();

        hyprbox.add(&hypr_software_box);
        hyprbox.add(&hypr_software_box1);
        hyprbox.add(&hypr_software_box2);

        box_up.add(&hyprbox);

        box_down.add(&get_installed_btn);
        box_down.add(&update_btn);
        box_down.add(&install_selected_btn);
        box_down.add(&uninstall_selected_btn);

        main_box.add(&box_up);
        main_box.add(&box_down);

        win.set_child(Some(&main_box));

        win.show_all();
        
        scrolled_window4.set_width_request(win.allocated_width() - 20);
        win.connect_resize_mode_notify(move |win| {
            scrolled_window4.set_width_request(win.allocated_width() - 20);
        });

    });

    app.run();

}

fn update_terminal_output(scrolled_window: gtk::ScrolledWindow, terminal_output: gtk::TextView, message: String) {
    let buff = terminal_output.buffer().unwrap();
    buff.insert_at_cursor(&message);
    let vadj = scrolled_window.vadjustment();
    vadj.set_value(vadj.upper());
    scrolled_window.set_vadjustment(Some(&vadj));
}

fn get_data_path() -> String {
    #[cfg(not(debug_assertions))]
    {
        let data_path = dirs::data_local_dir().unwrap();
        let data_path_str = data_path.to_str().unwrap();
        let data_path_str = format!("{}/hyprupdate/", data_path_str);

        return data_path_str;
    }
    #[cfg(debug_assertions)]
    {
        let data_path_str = format!("{}/hyprupdate/", current_dir().unwrap().to_str().unwrap());
        return data_path_str;
    }
}

fn compile(software: &str, meta: Metadata, scrolled_window: gtk::ScrolledWindow, terminal_output: gtk::TextView) {
    if !meta.get_to_compile(software) {
        return;
    }
    #[cfg(debug_assertions)]
    println!("\x1b[32mCompiling {}\x1b[0m", software);

    update_terminal_output(scrolled_window.clone(), terminal_output.clone(), format!("::Compiling {}\n", software));

    let path = get_data_path() + software;
    set_current_dir(path.as_str()).expect("Failed to set current directory");
    let build_cmd = meta.get_compile_cmd(software);
    run_command(build_cmd.as_str(), scrolled_window.clone(), terminal_output.clone());
    meta.set_to_compile(software, false);
    let install_cmd = meta.get_install_cmd(software);
    run_command(install_cmd.as_str(), scrolled_window.clone(), terminal_output.clone());
    meta.set_installed(software, true);
    #[cfg(debug_assertions)]
    println!("{} installed", software);
    update_terminal_output(scrolled_window.clone(), terminal_output.clone(), format!("::Installed {}\n", software));
}

fn which_repo(name: &str) -> &str {
    match name {
        "Hyprland" => HYPRLAND_REPO,
        "hyprlock" => HYPRLOCK_REPO,
        "hypridle" => HYPRIDLE_REPO,
        "hyprlang" => HYPRLANG_REPO,
        "hyprcursor" => HYPRCURSOR_REPO,
        "hyprpaper" => HYPRPAPER_REPO,
        "hyprpicker" => HYPRPICKER_REPO,
        "hyprwayland-scanner" => HYPRWAYLAND_SCANNER_REPO,
        "xdg-desktop-portal-hyprland" => XDG_DESKTOP_PORTAL_HYPRLAND_REPO,
        _ => "",
    }
}

fn run_command(inpt: &str, scroll_window: gtk::ScrolledWindow, terminal_output: gtk::TextView) {
    let commands = inpt.trim().split("&&").map(|i| i.trim());
    for command in commands {
        #[cfg(debug_assertions)]
        println!("\x1b[34mRunning: {}\x1b[0m", command);

        update_terminal_output(scroll_window.clone(), terminal_output.clone(), format!("::Running {}\n", command));

        let iterator = command.split_whitespace();
        let mut command = Command::new(iterator.clone().next().unwrap());
        for arg in iterator.skip(1) {
            command.arg(arg);
        }
        let mut process = command
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to build repository");
        
        let reader = BufReader::new(process.stdout.take().expect("failed to get stdout"));
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    #[cfg(debug_assertions)]
                    println!("{}", line);
                    update_terminal_output(scroll_window.clone(), terminal_output.clone(), format!("{}\n", line.trim()));
                },
                Err(e) => {
                    #[cfg(debug_assertions)]
                    println!("\x1b[31mError: {}\x1b[0m", e);
                    
                    update_terminal_output(scroll_window.clone(), terminal_output.clone(), format!("Error: {}\n", e));
                },
            }
        }
        process.wait().expect("failed to wait on child");
    }
}
