use std::{fs::{metadata, File}, io::{Read, Write}};

#[derive(Debug, Clone)]
pub struct Metadata {
    file: String,
}

impl Metadata {
    pub fn new(appdir: &str) -> Metadata {
        let file = appdir.to_owned() + "meta.toml";

        let base_content = br#"
[hyprcursor]
compile_cmd = "cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX:PATH=/usr -S . -B ./build && cmake --build ./build --config Release --target all"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false

[hypridle]
compile_cmd = "cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -S . -B ./build && cmake --build ./build --config Release"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false

[hyprland]
compile_cmd = "make all"
install_cmd = "pkexec --keep-cwd make install"
installed = false
uncompiled = false

[hyprlang]
compile_cmd = "cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX:PATH=/usr -S . -B ./build && cmake --build ./build --config Release"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false

[hyprlock]
compile_cmd = "cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -S . -B ./build && cmake --build ./build --config Release"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false

[hyprpaper]
compile_cmd = "cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX:PATH=/usr -S . -B ./build && cmake --build ./build --config Release"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false

[hyprpicker]
compile_cmd = "cmake --no-warn-unused-cli -DCMAKE_BUILD_TYPE:STRING=Release -DCMAKE_INSTALL_PREFIX:PATH=/usr -S . -B ./build && cmake --build ./build --config Release"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false

[hyprwayland-scanner]
compile_cmd = "cmake -DCMAKE_INSTALL_PREFIX=/usr -B build && cmake --build build"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false

[xdg-desktop-portal-hyprland]
compile_cmd = "cmake -DCMAKE_INSTALL_LIBEXECDIR=/usr/lib -DCMAKE_INSTALL_PREFIX=/usr -B build && cmake --build build"
install_cmd = "pkexec --keep-cwd cmake --install ./build"
installed = false
uncompiled = false
"#;

        if metadata(&file).is_err() {
            let mut file = File::create(&file).expect("Failed to create metadata file");
            file.write_all(base_content).expect("Failed to write to metadata file");
        }

        Metadata {
            file, 
        }
    }

    pub fn set_to_compile(&self, name: &str, value: bool) {
        let name = name.to_lowercase();
        let config = self.read_config();

        let mut toml: toml::Table = toml::from_str(&config).expect("Failed to parse metadata file");

        toml[&name]["uncompiled"] = toml::Value::Boolean(value);

        let toml_string = toml::to_string(&toml).expect("Failed to convert to TOML string");

        let mut file = File::create(&self.file).expect("Failed to create metadata file");
        file.write_all(toml_string.as_bytes()).expect("Failed to write to metadata file");
    }

    pub fn get_to_compile(&self, software: &str) -> bool {
        let software = software.to_lowercase();
        let config = self.read_config();

        let toml: toml::Table  = toml::from_str(&config).expect("Failed to parse metadata file");

        toml[&software]["uncompiled"].as_bool().expect("Failed to get uncompiled value")
    }

    pub fn set_installed(&self, name: &str, value: bool) {
        let name = name.to_lowercase();
        let config = self.read_config();

        let mut toml: toml::Table  = toml::from_str(&config).expect("Failed to parse metadata file");

        toml[&name]["installed"] = toml::Value::Boolean(value);

        let mut file = File::create(&self.file).expect("Failed to create metadata file");
        file.write_all(toml.to_string().as_bytes()).expect("Failed to write to metadata file");

    }
    #[allow(dead_code)]
    pub fn get_installed(&self, software: &str) -> bool {
        let software = software.to_lowercase();
        let config = self.read_config();

        let toml: toml::Table  = toml::from_str(&config).expect("Failed to parse metadata file");

        toml[&software]["installed"].as_bool().expect("Failed to get installed value")
    }

    pub fn get_compile_cmd(&self, software: &str) -> String {
        let software = software.to_lowercase();
        let config = self.read_config();

        let toml: toml::Table  = toml::from_str(&config).expect("Failed to parse metadata file");

        toml[&software]["compile_cmd"].as_str().expect("Failed to get compile_cmd value").to_string()
    }

    pub fn get_install_cmd(&self, software: &str) -> String {
        let software = software.to_lowercase();
        let config = self.read_config();

        let toml: toml::Table  = toml::from_str(&config).expect("Failed to parse metadata file");

        toml[&software]["install_cmd"].as_str().expect("Failed to get install_cmd value").to_string()
    }

    fn read_config(&self) -> String {
        let mut file = File::open(&self.file).expect("Failed to open metadata file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read metadata file");

        contents
    }
}