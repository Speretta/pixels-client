use std::path::PathBuf;


#[derive(Default)]
pub struct FileDialog {
    #[cfg(target_os = "macos")]
    opened_file: Option<PathBuf>,
    #[cfg(target_os = "macos")]
    open_file_dialog: Option<egui_file::FileDialog>,
}

impl FileDialog {
    

    #[cfg(target_os = "macos")]
    pub fn update(&mut self, ctx: &Context){
        if let Some(dialog) = &mut self.open_file_dialog {
            if dialog.show(ctx).selected() {
              if let Some(file) = dialog.path() {
                  self.opened_file = Some(file);
              }
            }
          }
    }

    #[cfg(target_os = "macos")]
    pub fn show(&mut self) -> Option<PathBuf> {
        let mut dialog = egui_file::FileDialog::open_file(self.opened_file.clone()).filter(Box::new(|path| {
            path.extension().map_or(false, |ext| {
                let ext = ext.to_ascii_lowercase();
                ext == OsString::from("PNG") || ext == OsString::from("jpg") || ext == OsString::from("jpeg")
            } )
        }));
        dialog.open();
        self.open_file_dialog = Some(dialog);
        self.opened_file.clone()
    }

    #[cfg(not(target_os = "macos"))]
    pub fn show() -> Option<PathBuf>{
        rfd::FileDialog::new()
        .add_filter("PNG Image", &["png"])
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .set_directory("~")
        .pick_file()
    }
}
