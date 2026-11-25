import QtQuick
import QtQuick.Controls

// Start screen: a single centered "Play" button.
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