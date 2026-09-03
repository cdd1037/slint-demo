#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use slint::ComponentHandle;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = AppWindow::new()?;

    let weak = app.as_weak();
    app.on_increment(move || {
        if let Some(app) = weak.upgrade() {
            app.set_counter(app.get_counter() + 1);
        }
    });

    let weak = app.as_weak();
    app.on_reset(move || {
        if let Some(app) = weak.upgrade() {
            app.set_counter(0);
        }
    });

    app.run()
}
