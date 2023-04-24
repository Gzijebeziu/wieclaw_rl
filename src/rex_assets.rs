use rltk::{rex::XpFile};

rltk::embedded_resource!(MENU, "../resources/menu.xp");
rltk::embedded_resource!(WFC_DEMO_IMAGE1, "../resources/wfc-demo1.xp");

pub struct RexAssets {
    pub menu : XpFile
}

impl RexAssets {
    #[allow(clippy::new_without_default)]
    pub fn new() -> RexAssets {
        rltk::link_resource!(MENU, "../resources/menu.xp");
        rltk::link_resource!(WFC_DEMO_IMAGE1, "../resources/wfc-demo1.xp");

        RexAssets{
            menu : XpFile::from_resource("../resources/menu.xp").unwrap()
        }
    }
}