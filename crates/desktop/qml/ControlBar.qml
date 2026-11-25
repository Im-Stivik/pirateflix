import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

// Progress bar + transport buttons. Purely presentational: it receives
// state through properties and reports user actions through signals.
Rectangle {
    id: root

    // State (all in milliseconds where applicable)
    property int position: 0
    property int duration: 0
    property bool seekable: false
    property bool playing: false

    // User actions
    signal seekTo(int ms)
    signal backwardClicked()
    signal forwardClicked()
    signal playPauseClicked()
    signal stopClicked()

    color: "#1f232b"

    function fmt(ms) {
        var s = Math.max(0, Math.floor(ms / 1000))
        var h = Math.floor(s / 3600)
        var m = Math.floor((s % 3600) / 60)
        var sec = s % 60
        var mm = (m < 10 ? "0" : "") + m
        var ss = (sec < 10 ? "0" : "") + sec
        return h > 0 ? h + ":" + mm + ":" + ss : m + ":" + ss
    }

    ColumnLayout {
        anchors.fill: parent
        anchors.margins: 10
        spacing: 6

        // Progress
        RowLayout {
            Layout.fillWidth: true
            spacing: 10

            Label {
                text: root.fmt(root.position)
                color: "white"
            }
            Slider {
                Layout.fillWidth: true
                from: 0
                to: Math.max(1, root.duration)
                value: root.position
                enabled: root.seekable
                onMoved: root.seekTo(value)
            }
            Label {
                text: root.fmt(root.duration)
                color: "white"
            }
        }

        // Buttons
        RowLayout {
            Layout.alignment: Qt.AlignHCenter
            spacing: 10

            Button {
                text: "⏪ 5s"
                onClicked: root.backwardClicked()
            }
            Button {
                text: root.playing ? "Pause" : "Continue"
                onClicked: root.playPauseClicked()
            }
            Button {
                text: "5s ⏩"
                onClicked: root.forwardClicked()
            }
            Button {
                text: "Stop"
                onClicked: root.stopClicked()
            }
        }
    }
}