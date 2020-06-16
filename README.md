<img src="./webroot/rooster.png" href="http://icons8.com/" alt="Rooster Personal Web Server" width="128"/>

# Rooster
A personal static web server with Rust. This is an experimental project created as an attempt to scratch a personal itch and learn Rust.

## Installation
Just copy rooster executable to your web root folder and run!

- Root Folder
  - index.html
  - rooster.exe

## Custom Configuration
Adding a rooster.cfg file in the same folder as the rooster.exe file can be used to modify the following properties

[Sample Configuration](rooster.cfg)

## Administration
Rooster is windowless and currently does not also support system tray. All administration is through the /_rooster route

### /_rooster/about
A small about Rooster page

### /_rooster/shutdown
Shuts the server down. [TODO: check for local host origin to permit shutdown]
