slint::include_modules!();
use std::rc::Rc;
use slint::{ModelRc, StandardListViewItem, VecModel, SharedString};
fn main() -> Result<(), slint::PlatformError> {

    let window = MainWindow::new()?;
    // Create a VecModel and put it in an Rc.
    let the_model : Rc<VecModel<StandardListViewItem>> =
        Rc::new(VecModel::from(vec!["Hello".into(), "World".into()]));
    // Convert it to a ModelRc.
    let the_model_rc = ModelRc::from(the_model.clone());
    // Pass the model to the ui: The generated set_the_model setter from the
    // the_model property takes a ModelRc.
    window.set_types(the_model_rc);
    window.run()
}
