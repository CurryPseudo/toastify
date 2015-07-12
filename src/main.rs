extern crate notify_rust;
#[macro_use]
extern crate clap;

use notify_rust::Notification;
//use clap::{App, SubCommand, Arg};
use clap::{App, Arg};

arg_enum!{
pub enum NotificationUrgency{ Low, Medium, High }
}

fn main() {
    let urgencies = ["low","normal","high"];

    let matches = App::new("notify-rust")
                        .version(&crate_version!()[..])
                        .author("Hendrik Sollich <hendrik@hoodie.de>")
                        .about("notify-send in rust")

                        .arg( Arg::with_name("summary")
                              .help("Title of the Notification.")
                              .required(true))

                        .arg( Arg::with_name("body")
                              .help("Message body"))

                        .arg( Arg::with_name("app-name")
                              .help("Set a specific app-name manually.")
                              .short("a")
                              .long("app-name")
                              .takes_value(true))

                        .arg( Arg::with_name("expire-time")
                              .help("Time until expiration in milliseconds. 0 means forever. ")
                              .short("t")
                              .long("expire-time")
                              .takes_value(true))

                        .arg( Arg::with_name("icon")
                              .short("i")
                              .help("Icon of notification.")
                              .long("icon")
                              .takes_value(true))

                        .arg( Arg::with_name("category")
                              .help("Set a category.")
                              .short("c")
                              .long("category")
                              .takes_value(true))

                        .arg( Arg::with_name("urgency")
                              .help("How urgent is it.")
                              .short("u")
                              .long("urgency")
                              .takes_value(true)
                              .possible_values(&urgencies))

                        .arg( Arg::with_name("debug")
                              .help("Also prints notification to stdout")
                              .short("d")
                              .long("debug"))

                        .get_matches();

    let mut notification = Notification::new();

    let summary = matches.value_of("summary").unwrap();

    if let Some(appname) = matches.value_of("app-name"){
        notification.appname(appname);
    }

    notification.summary(summary);

    if let Some(icon) = matches.value_of("icon"){
        notification.icon(icon);
    }

    if let Some(body) = matches.value_of("body"){
        notification.body(body);
    }

    if let Some(categories) = matches.value_of("category"){
        //notification.body(body);
        for category in categories.split(':'){
            notification.hint(notify_rust::NotificationHint::Category(category.to_owned()));
        }
    }

    if let Some(timeout_string) = matches.value_of("expire-time"){
        if let Ok(timeout) = timeout_string.parse::<i32>(){
            notification.timeout(timeout); }
        else {println!("can't parse timeout {:?}, please use a number", timeout_string);}
    }

    if matches.is_present("urgency") {
        let urgency = value_t_or_exit!(matches.value_of("urgency"), NotificationUrgency);
        // TODO: somebody make this a cast, please!
        match urgency {
            NotificationUrgency::Low => notification.urgency(notify_rust::NotificationUrgency::Low),
            NotificationUrgency::Medium => notification.urgency(notify_rust::NotificationUrgency::Medium),
            NotificationUrgency::High => notification.urgency(notify_rust::NotificationUrgency::High),
        };
    }

    if matches.is_present("debug"){ notification.show_debug(); }
    else{                           notification.show();       }

}
