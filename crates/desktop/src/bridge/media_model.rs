use core::pin::Pin;
use cxx_qt_lib::QString;

use crate::runtime;
use cxx_qt::Threading;

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, name)]
        #[qproperty(QString, status)]
        type UserModel = super::UserModelRust;

        // #[qinvokable]
        // #[cxx_name = "loadUser"]
        // fn list_all(self: Pin<&mut UserModel>, id: u32);
    }

    // lets us get back to the Qt thread from async code
    impl cxx_qt::Threading for UserModel {}
}

#[derive(Default)]
pub struct UserModelRust {
    name: QString,
    status: QString,
}

// impl qobject::UserModel {
//     pub fn load_user(mut self: Pin<&mut Self>, id: u32) {
//         self.as_mut().set_status(QString::from("Loading..."));
//
//         let qt_thread = self.qt_thread();
//         let app = runtime::app();
//
//         // run the service call on the tokio runtime, never block the UI thread
//         runtime::rt().spawn(async move {
//             let result = app.users.get_user(id).await;
//
//             // hop back to the Qt thread to update properties
//             let _ = qt_thread.queue(move |mut model| match result {
//                 Ok(user) => {
//                     model.as_mut().set_name(QString::from(user.name.as_str()));
//                     model.as_mut().set_status(QString::from("Done"));
//                 }
//                 Err(e) => {
//                     model.as_mut().set_status(QString::from(e.to_string().as_str()));
//                 }
//             });
//         });
//     }
// }