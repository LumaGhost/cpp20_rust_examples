

use chrono::prelude::*;

#[no_mangle]
pub extern "C" fn meep() {
    println!("hello from rust uwu");
    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();
    println!("utc time: {}, local time: {}", utc, local);
}
