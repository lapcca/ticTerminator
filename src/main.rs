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
        let open_ui_weak_ptr_clone = open_ui_weak_ptr.clone();
        let open_ui_weak_ptr_clone1 = open_ui_weak_ptr.clone();
        open_ui.on_select_file(move ||{
            let loc_path = FileDialog::new().pick_file().unwrap().display().to_string();
            let open_ui_ptr = open_ui_weak_ptr.upgrade().unwrap();
            // let msg = format!("got path:{loc_path}");
            // MessageDialog::new().set_description(msg).show();
            open_ui_ptr.set_path(loc_path.into());
        });
        let ui_weak_ptr_clone = ui_weak_ptr.clone();

        open_ui.on_ok_clicked(move ||{
            let open_ui_ptr = open_ui_weak_ptr_clone.upgrade().unwrap();
            let main_ui = ui_weak_ptr_clone.upgrade().unwrap();
            main_ui.set_path(open_ui_ptr.get_path().into());
            let _=open_ui_ptr.hide().unwrap();
        });
        open_ui.on_cancel_clicked(move ||{
            let open_ui_ptr = open_ui_weak_ptr_clone1.upgrade().unwrap();
            let _=open_ui_ptr.hide();
        });
       let _ = open_ui.run();

    });
    ui.run()
}
