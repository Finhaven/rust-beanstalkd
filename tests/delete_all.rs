// Test the delete all functionality

extern crate beanstalkd;

use beanstalkd::Beanstalkd;

// Delay is in seconds. Use a big delay so the test will finish before the job becomes ready again
const RELEASE_DELAY: u32 = 60;

#[test]
fn delete_all_ready() {
    let tube_name = "delete_all_ready";
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube(tube_name).unwrap();
    for idx in 0..5 {
        let message = format!("Message {}", idx);
        beanstalkd.put(&message, 0, 0, 10000).unwrap();
    }

    beanstalkd.watch(tube_name).unwrap();
    beanstalkd.delete_all_ready().unwrap();

    // There shouldn't be anything left in the tube
    let result = beanstalkd.peek_ready();
    assert_eq!(result, Ok(None));
}