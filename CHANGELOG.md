# 0.3.9
- `command` will now be killed when application is closed

# 0.3.3
- Removed wrongly imported `platform::unix::WindowBuilderExtUnix` 


# 0.3.1
- Fixed binary downloads when the file already exists
- Fixed window freezes in certain occasions
- Added `hide_taskbar_icon` option
- Added warning when icon is not found on website
- Added error handling with icon validation, now it shouldn't crash when an icon is not found/incorrect
- Now `icon` can be a local file or a url to download it from
- Removed `icon` from naty.toml, now it will be used only if found in the executable folder
- Added `command` option for each supported OS, allowing to execute a command after the program execution