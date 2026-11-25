import QtQuick
import QtQuick.Controls

Item {
    id: root

    signal playClicked()

    Button {
        anchors.centerIn: parent
        text: "Play"
        font.pixelSize: 22
        implicitWidth: 160
        implicitHeight: 56
        onClicked: root.playClicked()
    }
}