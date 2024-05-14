#![allow(unused_imports)]
use std::env::{current_dir, set_current_dir};
use std::fs::{create_dir_all, metadata, remove_dir_all};
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};

use crate::metadata::Metadata;
use git2::Repository;
use gtk::prelude::*;
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
            .default_width(320)
            .default_height(200)
            .title("Hyprupdate")
            .build();

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

        hypr_software_box.add(&hyprland.clone());
        hypr_software_box.add(&hyprlock);
        hypr_software_box.add(&hypridle);
        hypr_software_box.add(&hyprlang);
        hypr_software_box.add(&hyprcursor);
        hypr_software_box.add(&hyprpaper);
        hypr_software_box.add(&hyprpicker);
        hypr_software_box.add(&hyprwayland_scanner);
        hypr_software_box.add(&xdg_desktop_portal_hyprland);

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
                        println!("{} up to date.", name);
                    } else {
                        println!("Updated {}", name);
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
                        Repository::clone(
                            which_repo(&i.label().unwrap().as_str()),
                            data_dir.clone() + i.label().unwrap().as_str(),
                        )
                        .expect("Failed to clone");
                    }
                    meta1.set_installed(&i.label().unwrap().as_str(), true);
                    meta1.set_to_compile(&i.label().unwrap().as_str(), true);
                    compile(&i.label().unwrap().as_str(), meta1.clone());
                }
            }
        });

        let uninstall_selected_btn = gtk::Button::with_label("Uninstall Selected");
        uninstall_selected_btn.connect_clicked(move |_| {
            for i in &software3 {
                if i.is_active() {
                    remove_dir_all(get_data_path() + i.label().unwrap().as_str())
                        .expect("Failed to remove directory");
                    meta2.set_installed(&i.label().unwrap().as_str(), false);
                }
            }
        });

        let box_up = gtk::Box::builder()
            .orientation(gtk::Orientation::Horizontal)
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

        box_up.add(&hypr_software_box);

        box_down.add(&get_installed_btn);
        box_down.add(&update_btn);
        box_down.add(&install_selected_btn);
        box_down.add(&uninstall_selected_btn);

        main_box.add(&box_up);
        main_box.add(&box_down);

        win.set_child(Some(&main_box));

        // Don't forget to make all widgets visible.
        win.show_all();
    });

    app.run();
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

fn compile(software: &str, meta: Metadata) {
    if !meta.get_to_compile(software) {
        return;
    }
    println!("\x1b[32mCompiling {}\x1b[0m", software);

    let path = get_data_path() + software;
    set_current_dir(path.as_str()).expect("Failed to set current directory");
    let build_cmd = meta.get_compile_cmd(software);
    run_command(build_cmd.as_str());
    meta.set_to_compile(software, false);
    let install_cmd = meta.get_install_cmd(software);
    run_command(install_cmd.as_str());
    meta.set_installed(software, true);
    println!("{} installed", software);
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

fn run_command(inpt: &str) {
    let commands = inpt.trim().split("&&").map(|i| i.trim());
    for command in commands {
        println!("\x1b[34mRunning: {}\x1b[0m", command);
        let iterator = command.split_whitespace();
        let mut command = Command::new(iterator.clone().next().unwrap());
        for arg in iterator.skip(1) {
            command.arg(arg);
        }
        let mut process = command
            .stderr(Stdio::piped())
            .spawn()
            .expect("failed to build repository");

        let reader = BufReader::new(process.stderr.take().expect("failed to get stdout"));
        for line in reader.lines() {
            match line {
                Ok(line) => {
                    println!("{}", line);
                },
                Err(e) => {
                    println!("\x1b[31mError: {}\x1b[0m", e);
                },
            }
        }
        process.wait().expect("failed to wait on child");
    }
}
