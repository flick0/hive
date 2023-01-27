pub struct HiveWindow {
    pub name: String,
    pub resizable: bool,
    pub stacking: gtk_layer_shell::Layer,
    pub focusable: bool,
    pub reserve_space: bool,
    pub position: crate::util::Position
}

impl HiveWindow {

}