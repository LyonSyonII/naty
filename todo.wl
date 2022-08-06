#######################
#        TODO         #
#######################
[] Naty_nativefy
    [] Abort and clean installation if download fails
    [] Add icon to executable (currently it only shows on window/taskbar)
        [] See [`rcedit`](https://github.com/electron/rcedit/)
    [x] [Fixed - 0.2.2 ] Add warning when icon is not found in website
    [] Use Naty's logo if no icon is availabe
    [] Tray support
    [] If logo is .svg transform it to .png
    [] [Windows] Add .exe to final executable
    [] [Windows] No output to stdout
        [] Caused by #![windows_subsystem = "windows"], search form to activate it only on App executable
    [] [Windows] Window closes instantly
[] Naty_app
    [] web.whatsapp.com
        [x] Window freezes when opening an image
            [x] [Fixed - 0.2.2] EventLoop::Poll instead of EventLoop::Wait. 
            [x] [Comments] Seems like the issue does not happen anymore (even with EventLoop::Wait). It's probably related to a drivers issue.
        [] Window freezes when a video is present
        [] [Linux] Window stutters if dragged when whatsapp's loading
    [] Tray support
    [] Support for notifications
        [] Linux
        [] Windows
        [] MacOS


##########################
#   CHANGELOG - v0.2.2   #
##########################
- Fixed binary downloads when the file already exists
- Fixed window freezes in certain occasions
- Added hide_taskbar_icon option
- Added warning when icon is not found on website