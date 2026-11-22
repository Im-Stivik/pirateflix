import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import com.myapp

ApplicationWindow {
    visible: true
    width: 400
    height: 300
    title: "My App"

    UserModel { id: userModel }

    ColumnLayout {
        anchors.centerIn: parent
        spacing: 12

        Button {
            text: "Load user 1"
            onClicked: userModel.loadUser(1)
        }
        Label { text: "Name: " + userModel.name }
        Label { text: "Status: " + userModel.status }
    }
}