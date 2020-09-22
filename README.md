<img src="./web/gale/rooster.png" href="http://icons8.com/" alt="Gale Application Server" width="128"/>

# Gale JS - Application Server
Note: NOT READY FOR PRODUCTION YET

A small footprint, batteries included application server built with Rust. Gale JS has out of box Javascript server scripting along with default set of commonly used APIs. It is currently just a single executable of about ~6 MB for Windows and ~11 MB for Linux with zero dependencies.

# Announcements
## Sep 2020
- Added SSL support
- Linux 64-bit binary added
- Script API now has a good base of frequently used functions
    - File IO
    - Zip IO
    - Crypto
        - MD5
        - SHA2
        - SHA3
    - Properties
    - Encoding
      - Base 64 
      - URL Encoding
    - HTTP
      - GET
      - POST 
      - POST Multipart
    - Web/HTML Scraping
      - String
      - File
      - URL
    - Utils
      - Random
      - UUID

## Aug 2020
- Script Support
  - Gale now has a very basic JS server script support
  - It is ES5 compliant and uses [Duktape engine](https://duktape.org/) via the [Ducc](https://github.com/SkylerLipthay/ducc) rust bindings
  - It can load and serve script files including external libraries such as Moment JS, Math JS and more.
  - Examples can be found in the test repository [here](https://github.com/elasmojs/gale-test)

# Getting Started
## Downloads
The following downloads are available.
| OS  | Option 1   | Option 2   |
|---|---|---|
| Windows  | [64 bit](./dist/gale-win64.exe)  | [32 bit](./dist/gale-win32.exe)  |
| Linux  | [64 bit](./dist/gale-linux64)   |   |
| Mac  | Coming soon   |   |

## Installation
- Install folder
  - Your web root folder
    - gale (default app)
      - box (sandbox folder)
      - index.html
  - gale (executable)
  - gale.cfg

## Custom Configuration
Adding a gale.cfg file in the same folder as the gale excutable file can be used to modify the following properties

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
- Improved log handling
- Daemon/Windows Service support
