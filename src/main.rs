slint::include_modules!();
use std::rc::Rc;
use slint::{ModelRc, StandardListViewItem, VecModel};
use rfd::{FileDialog, MessageDialog};
fn main() -> Result<(), slint::PlatformError> {
    let mut path:String = String::new();
    let ui = MainWindow::new()?;
    // Create a VecModel and put it in an Rc.
    let the_model : Rc<VecModel<StandardListViewItem>> =
        Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
    // Convert it to a ModelRc.
    let the_model_rc = ModelRc::from(the_model.clone());
    // Pass the model to the ui: The generated set_the_model setter from the
    // the_model property takes a ModelRc.
    ui.set_types(the_model_rc);

    let ui_weak_ptr = ui.as_weak();
    ui.on_open_file(move || {
        let open_ui = OpenDialog::new().unwrap();
        let open_ui_weak_ptr = open_ui.as_weak();
        open_ui.on_ok_clicked(||{
            let loc_path = FileDialog::new().pick_file().unwrap().display().to_string();
            let open_ui_ptr = open_ui_weak_ptr.upgrade().unwrap();
            MessageDialog::new().set_description(format!("Get file path:{}", loc_path).as_str()).show();
            open_ui_ptr.set_path(loc_path.into());

        });

    });
    ui.run()
}
