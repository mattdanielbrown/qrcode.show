# Set output file
Output demo.gif

# Set theme and terminal settings
Set Shell "zsh"
Set FontSize 20
Set Width 900
Set Height 700
Set Padding 30
Set WindowBar "Colorful"
Set Theme "Dracula"
Set PlaybackSpeed 1.0
Set TypingSpeed 100ms

# Start the recording
Sleep 1s

# Command 1: GET request with path input
Type "curl -s qrcode.show/https://example.com"
Sleep 1s
Enter
Sleep 3s

# Clear for the next command
Type "clear"
Enter
Sleep 500ms

# Command 2: POST request with text input
Type "curl -s qrcode.show -d 'Hello, World!'"
Sleep 1s
Enter
Sleep 3s

# Clear for the next command
Type "clear"
Enter
Sleep 500ms

# Command 3: Piping data into curl via stdin
Type "echo 'https://github.com/sayanarijit/qrcode.show' | curl -s qrcode.show -d @-"
Sleep 1.5s
Enter
Sleep 5s
