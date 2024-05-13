# hyprupdate
### A simple tool to manage yout hypr software

![image](https://github.com/LOSEARDES77/hyprupdate/assets/84414230/98036f58-68a4-487f-acd9-e022ed1d8fed)

---

This is a simple tool written in rust that makes more easy to manage hypr apps using a simple gui written in gtk.</br>
It does this by cloning the repos, compiling them and installing, for the build it uses the suggested build command on the official repos,
but this can be configured easily on a toml file.

> [!WARNING]
> You will need to install the required dependencies for each app. </br>
> This information is aviable on the official repos.

The config is located at `~/.local/share/hyprupdate/meta.toml` generated at first launch.
Also the repositoties are stored at this location.

If you notice any bugs you can report them at [https://github.com/LOSEARDES77/hyprupdate/issues](https://github.com/LOSEARDES77/hyprupdate/issues)

### Hypr apps supported
 - [Hyprland](https://github.com/hyprwm/Hyprland)
 - [hyprlock](https://github.com/hyprwm/hyprlock)
 - [hypridle](https://github.com/hyprwm/hypridle)
 - [hyprpaper](https://github.com/hyprwm/hyprpaper)
 - [hyprcursor](https://github.com/hyprwm/hyprcursor)
 - [hyprlang](https://github.com/hyprwm/hyprlang)
 - [hyprpicker](https://github.com/hyprwm/hyprpicker)
 - [hyprwayland-scanner](https://github.com/hyprwm/hyprwayland-scanner)
 - [xdg-desktop-portal-hyprland](https://github.com/hyprwm/xdg-desktop-portal-hyprland)

> [!NOTE]
> I am not the developer of this awesome apps. </br>
> The lead developer of all apps avobe is [vaxerski](https://github.com/vaxerski)


### Features
 - Can install uninstall with a simple click
 - Update all with a single click
 - Config in toml
 - output of build command at the stdout to check errors at compile time

> [!NOTE]
> Right now it only uses -git versions but adding the option to use tagged releases is planned
