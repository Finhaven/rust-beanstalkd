// Test the peek functionality

extern crate beanstalkd;

use beanstalkd::Beanstalkd;

#[test]
fn no_peek_on_empty_tube() {
    let tube_name = "no_peek_on_empty_tube";
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube(tube_name).unwrap();
    // Don't put anything into this tube

    beanstalkd.watch(tube_name).unwrap();
    let result = beanstalkd.peek_ready();
    assert_eq!(result, Ok(None));
}

#[test]
fn peek_ready_finds_a_message() {
    let tube_name = "peek_ready_finds_a_message";
    let message = "Hello World";
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube(tube_name).unwrap();
    beanstalkd.put(message, 0, 0, 10000).unwrap();

    beanstalkd.watch(tube_name).unwrap();
    let (job_id, actual_message) = beanstalkd.peek_ready().unwrap().unwrap();
    assert_eq!(actual_message, message);

    // Clean up
    beanstalkd.delete(job_id).unwrap();
}

#[test]
fn peek_ready_ignores_delayed_job() {
    let tube_name = "peek_ready_ignores_delayed_job";
    let message = "Hello World";
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube(tube_name).unwrap();
    beanstalkd.put(message, 0, 0, 10000).unwrap();

    beanstalkd.watch(tube_name).unwrap();

    // Get a job and delay it
    let (job_id, _) = beanstalkd.reserve().unwrap();
    // use a huge delay so the test will finish before the delay
    beanstalkd.release(job_id, 0, 5000).unwrap();

    // Because we're using peek_ready, it won't catch a job in the delayed state
    beanstalkd.watch(tube_name).unwrap();
    let result = beanstalkd.peek_ready();
    assert_eq!(result, Ok(None));

    // Clean up
    beanstalkd.delete(job_id).unwrap();
}