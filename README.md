<img src="./webroot/rooster.png" href="http://icons8.com/" alt="Gale Application Server" width="128"/>

# Gale
A small footprint, batteries included application server built with Rust. Gale JS has out of box Javascript server scripting along with default set of commonly used APIs. It is currently just a single executable of about ~6 MB in size with zero dependencies.

# Announcements
## Aug 2020
- Script Support
  - Gale now has a very basic JS server script support
  - It is ES5 compliant and uses [Duktape engine](https://duktape.org/) via the [Ducc](https://github.com/SkylerLipthay/ducc) rust bindings
  - It can load and serve script files including external libraries such as Moment JS, Math JS and more.
  - Examples can be found in the test repository [here](https://github.com/elasmojs/gale-test)

## Downloads
[Windows 32 bit](./dist/gale-win32.exe)

[Windows 64 bit](./dist/gale-win64.exe)

## Installation
Just copy Gale executable to your web root folder and run!

### default mode
- Your web root folder
  - index.html
  - gale.exe
  
### with customized configuration
- Your web root folder
  - index.html
- gale.exe
- gale.cfg

## Custom Configuration
Adding a gale.cfg file in the same folder as the gale.exe file can be used to modify the following properties

[Sample Configuration](gale.cfg)

## Administration
Gale is windowless and currently does not also support system tray. All administration is through the /_gale route

### /_gale/about
A small about Gale page

### /_gale/shutdown
Shuts the server down. (Allows requests orginating from localhost only)

## Tests Repo
The Gale tests repo is [here](https://github.com/elasmojs/gale-test). Check it out for some simple examples of using Gale.

## Upcoming
- Script support
  - Script APIs roadmap
- SSL support
- Improved log handling
- Daemon/Windows Service support
