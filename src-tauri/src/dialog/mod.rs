

use tauri::{plugin::Plugin, Runtime, Invoke};



#[tauri::command]
fn open_file_dialog() -> Result<(), String> {
    
    tauri::api::dialog::FileDialogBuilder::new().pick_file(move |path| {
    
    });
    Ok(())
}

pub struct DialogPlugin<R: Runtime> {
    invoke_handler: Box<dyn Fn(Invoke<R>) + Send + Sync>,
}

impl<R: Runtime> DialogPlugin<R> {

    pub fn new()-> Self {
        Self {
            invoke_handler: Box::new(tauri::generate_handler![ open_file_dialog]),
        }
    }

}

impl<R: Runtime> Plugin<R> for DialogPlugin<R> {

    fn name(&self) -> &'static str {"dialog"}

    fn initialize(&mut self, app: &tauri::AppHandle<R>, _: serde_json::Value) -> tauri::plugin::Result<()> {
        Ok(())
    }

    fn extend_api(&mut self, message: Invoke<R>) {
        (self.invoke_handler)(message)
    }

    fn initialization_script(&self) -> Option<String> {
    None
    }

    fn created(&mut self, window: tauri::Window<R>) {}

    fn on_page_load(&mut self, window: tauri::Window<R>, payload: tauri::PageLoadPayload) {}

}