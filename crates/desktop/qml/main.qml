import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Dialogs

ApplicationWindow {
    id: win
    visible: true
    width: 960
    height: 600
    minimumWidth: 480
    minimumHeight: 320
    title: "Video Player"
    color: "#15171c"


    // File picker: video files only
    FileDialog {
        id: fileDialog
        title: "Select a video"
        nameFilters: [
            "Video files (*.mp4 *.m4v *.mkv *.avi *.mov *.webm *.wmv *.flv *.mpg *.mpeg *.ts *.3gp *.ogv)"
        ]
        onAccepted: {
            pages.currentIndex = 1
            playerPage.open(selectedFile)
        }
    }

    StackLayout {
        id: pages
        anchors.fill: parent
        currentIndex: 0

        StartPage {
            onPlayClicked: fileDialog.open()
        }

        PlayerPage {
            id: playerPage
            onStopped: pages.currentIndex = 0
        }
    }
}