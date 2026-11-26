use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    CxxQtBuilder::new()
        .qt_module("Quick")
        .qt_module("QuickControls2")
        .qml_module(QmlModule::<&str, &str> {
            uri: "com.myapp",
            rust_files: &["src/bridge/media_model.rs"],
            qml_files: &[
                "qml/main.qml",
                "qml/StartPage.qml",
                "qml/PlayerPage.qml",
                "qml/ControlBar.qml",
            ],
            ..Default::default()

        })
        .build();
}