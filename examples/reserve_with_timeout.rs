extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    loop {
        match beanstalkd.reserve_with_timeout(2) {
            Ok(Some((id, body))) => {
                if id > 0 && &body != "TIMED_OUT" {
                    println!("id: {} body: {}", id, body);
                    break;
                } else {
                    println!("no job, do some other stuff...");
                }
            },
            Ok(None) => println!("timedout"),
            Err(err) => println!("{}", err),
        }
    }
}
