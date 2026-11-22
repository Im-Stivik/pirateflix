mod bridge;
mod runtime;

use my_app_core::{App, Config};
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QUrl};

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build()?;
    let app = rt.block_on(App::new(Config::from_env()))?;
    runtime::init(rt, app);

    let mut qapp = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from("qrc:/qt/qml/com/myapp/qml/main.qml"));
    }
    if let Some(qapp) = qapp.as_mut() {
        qapp.exec();
    }
    Ok(())
}