extern crate beanstalkd;

use beanstalkd::Beanstalkd;

fn main() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    let (id, body) = beanstalkd.reserve().unwrap();
    println!("{}", body);
    let _ = beanstalkd.release(id, 1024, 10).unwrap();
}
