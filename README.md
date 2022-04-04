# click4date

![image](https://user-images.githubusercontent.com/29699356/161469291-f8caccfc-9fe9-47df-b574-1bbdcab0e8b6.png)
### click for date
## This a really simple app that copys the current date to your clipbard
![image](https://user-images.githubusercontent.com/29699356/161469327-5d899df8-4906-4b93-abb2-2c0589dd5f6f.png)

## the date is copied to the clipbard in the format: YYYYMMDD
# Download Here:
https://github.com/wisehackermonkey/click4date/releases/
# Code
## thats it. thats the whole program.
```rust
extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

use chrono::{Datelike, Utc};
fn clipboard_paste(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
     ctx.set_contents(text.to_owned()).unwrap();
}
fn formate_date() -> String{
    let now = Utc::now();
    return format!("{:?}{:0>width$}{:0>width$}", now.year(), now.month(), now.day(),width=2);
}
fn main(){
    let current_date = formate_date();
    clipboard_paste(&current_date);
}
```
### pretty simple eh?.
## the python version 
```python
# 20180307
import datetime
import pyperclip
import time
d = datetime.datetime.now()

day = str(d.day).rjust(2,"0")
month = str(d.month).rjust(2,"0")
year = d.year 

pyperclip.copy("{}{}{}".format(year,month,day) )
```
# Pro tips:
### add the exe to your taskbar to have easy access to the app

# how to build from scratch (windows):



### install rust
[Install Rust - Rust Programming Language](https://www.rust-lang.org/tools/install)
![](assets/2022-01-07-11-17-26.png)
### check if cargo is installed
```bash
cargo --version
```
### if cargo command is not found, link it to windows path environment variable 

![](assets/2022-01-07-11-35-10.png)
![](assets/2022-01-07-11-39-00.png)

# add to path `ls ~/.cargo/bin ` (powershell)
### in my case its ` C:\Users\o\.cargo\bin`
![](assets/2022-01-07-11-37-43.png)
![](assets/2022-01-07-11-38-07.png)

### run `cargo -v` 
![](assets/2022-01-07-11-40-02.png)

### success!

# Compile the program
```bash
cargo build
```
