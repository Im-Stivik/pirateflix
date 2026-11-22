use std::sync::{Arc, OnceLock};
use my_app_core::App;
use tokio::runtime::Runtime;

static RT: OnceLock<Runtime> = OnceLock::new();
static APP: OnceLock<Arc<App>> = OnceLock::new();

pub fn init(rt: Runtime, app: Arc<App>) {
    let _ = RT.set(rt);
    let _ = APP.set(app);
}
pub fn rt() -> &'static Runtime { RT.get().expect("runtime not initialised") }
pub fn app() -> Arc<App> { APP.get().expect("app not initialised").clone() }