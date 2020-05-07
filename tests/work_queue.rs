extern crate beanstalkd;

use beanstalkd::Beanstalkd;

#[test]
fn produce_and_consume_simple_message() {
    let message = "Hello World";
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube("hello-world").unwrap();
    let _ = beanstalkd.put(message, 0, 0, 10000);

    beanstalkd.watch("hello-world").unwrap();
    let (id, body) = beanstalkd.reserve().unwrap();
    assert_eq!(message, body);
    let _ = beanstalkd.delete(id);
}

#[test]
fn handle_envelope_signed_by_investor() {
    let message = include_str!("../data/signed-by-investor.json");
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube("signed-enveloped").unwrap();
    let _ = beanstalkd.put(message, 0, 0, 10000);

    beanstalkd.watch("signed-enveloped").unwrap();
    let (id, body) = beanstalkd.reserve().unwrap();

    assert_eq!(message, body);

    let result = beanstalkd.delete(id);
    assert_eq!(result.is_ok(), true)
}

#[test]
fn handle_envelope_signed_by_investor_in_loop() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube("signed-enveloped-loop").unwrap();
    for _ in 0..100 {
        let message = include_str!("../data/signed-by-investor.json");

        let _ = beanstalkd.put(message, 0, 0, 10000);

        beanstalkd.watch("signed-enveloped-loop").unwrap();
        let (id, body) = beanstalkd.reserve().unwrap();

        assert_eq!(message, body);

        let result = beanstalkd.delete(id);
        assert_eq!(result.is_ok(), true)
    }
}

#[test]
fn handle_large_message() {
    let mut beanstalkd = Beanstalkd::localhost().unwrap();
    beanstalkd.tube("large-file").unwrap();
    let message = include_str!("../data/very-large-json-file.json");
    let _ = beanstalkd.put(message, 0, 0, 10000);

    beanstalkd.watch("large-file").unwrap();
    let (id, body) = beanstalkd.reserve().unwrap();

    assert_eq!(message, body);

    let result = beanstalkd.delete(id);
    assert_eq!(result.is_ok(), true)
}
