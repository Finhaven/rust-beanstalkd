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

#[test]
fn delete_all_delayed() {
    let tube_name = "delete_all_delayed";
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube(tube_name).unwrap();
    let num_messages = 5;
    for idx in 0..num_messages {
        let message = format!("Message {}", idx);
        beanstalkd.put(&message, 0, 0, 10000).unwrap();
    }

    beanstalkd.watch(tube_name).unwrap();
    for _ in 0 .. num_messages {
        let (job_id, _) = beanstalkd.reserve().unwrap();
        beanstalkd.release(job_id, 0, RELEASE_DELAY).unwrap();
    }

    beanstalkd.delete_all_delayed().unwrap();

    // There shouldn't be anything left in the tube
    let result = beanstalkd.peek_delayed();
    assert_eq!(result, Ok(None));
}

#[test]
fn delete_all_buried() {
    let tube_name = "delete_all_buried";
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube(tube_name).unwrap();
    let num_messages = 5;
    for idx in 0..num_messages {
        let message = format!("Message {}", idx);
        beanstalkd.put(&message, 0, 0, 10000).unwrap();
    }

    beanstalkd.watch(tube_name).unwrap();
    for _ in 0 .. num_messages {
        let (job_id, _) = beanstalkd.reserve().unwrap();
        beanstalkd.bury(job_id, 0).unwrap();
    }

    beanstalkd.delete_all_buried().unwrap();

    // There shouldn't be anything left in the tube
    let result = beanstalkd.peek_buried();
    assert_eq!(result, Ok(None));
}