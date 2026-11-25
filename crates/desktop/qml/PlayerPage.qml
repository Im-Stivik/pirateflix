import QtQuick
import QtQuick.Layouts
import QtMultimedia

// Video playback screen: owns the media player, the video surface,
// the control bar and the keyboard shortcuts.
Item {
    id: root

    readonly property int seekStepMs: 5000

    // Emitted after playback has been stopped and cleaned up.
    signal stopped()

    function open(url) {
        player.source = url
        player.play()
    }

    function seekBy(deltaMs) {
        player.position = Math.max(0, Math.min(player.duration, player.position + deltaMs))
    }

    function togglePlay() {
        if (player.playbackState === MediaPlayer.PlayingState)
            player.pause()
        else
            player.play()
    }

    function stop() {
        player.stop()
        player.source = ""
        root.stopped()
    }

    // Media backend (FFmpeg in Qt >= 6.5)
    MediaPlayer {
        id: player
        videoOutput: video
        audioOutput: AudioOutput {}
    }

    // Keyboard: arrow keys seek (active only while this page is shown)
    Shortcut {
        sequence: "Left"
        enabled: root.visible
        onActivated: root.seekBy(-root.seekStepMs)
    }
    Shortcut {
        sequence: "Right"
        enabled: root.visible
        onActivated: root.seekBy(root.seekStepMs)
    }

    ColumnLayout {
        anchors.fill: parent
        spacing: 0

        VideoOutput {
            id: video
            Layout.fillWidth: true
            Layout.fillHeight: true
            fillMode: VideoOutput.PreserveAspectFit
        }

        ControlBar {
            Layout.fillWidth: true
            Layout.preferredHeight: 96

            position: player.position
            duration: player.duration
            seekable: player.seekable
            playing: player.playbackState === MediaPlayer.PlayingState

            onSeekTo: (ms) => player.position = ms
            onBackwardClicked: root.seekBy(-root.seekStepMs)
            onForwardClicked: root.seekBy(root.seekStepMs)
            onPlayPauseClicked: root.togglePlay()
            onStopClicked: root.stop()
        }
    }
}