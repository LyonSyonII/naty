#######################
#        TODO         #
#######################
[] Naty_nativefy
    [] Abort and clean installation if download fails
    [] Add icon to executable (currently it only shows on window/taskbar)
        [] See [`rcedit`](https://github.com/electron/rcedit/)
    [] Add warning when icon is not found
    [] Tray support
[] Naty_app
    [] web.whatsapp.com
        [x] Window freezes when opening an image
            [x] [Fixed - 0.2.2] EventLoop::Poll instead of EventLoop::Wait. 
            [x] [Comments] Seems like the issue does not happen anymore (even with EventLoop::Wait). It's probably related to a drivers issue.
        [] [Linux] Window stutters if dragged when whatsapp's loading
    [] Tray support
    [] Support for notifications
        [] Linux
        [] Windows
        [] MacOS


#######################
#    DONE - v0.2.1    #
#######################
- Fixed binary downloads when the file already exists
- Fixed window freezes in certain occasions
- Added hide_taskbar_icon option
