use anyhow::{anyhow, Error};
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn clone_keeps_payload_alive() {
    let error = anyhow!("message");
    let clone = error.clone();
    assert_eq!(error.to_string(), clone.to_string());
    drop(error);
    assert_eq!(clone.to_string(), "message");
}

#[test]
fn cloned_error_is_send_and_sync() {
    let error = anyhow!("message");
    let barrier = Arc::new(Barrier::new(2));
    let thread_barrier = Arc::clone(&barrier);
    let thread_error = error.clone();
    let handle = thread::spawn(move || {
        thread_barrier.wait();
        assert_eq!(thread_error.to_string(), "message");
    });
    barrier.wait();
    assert_eq!(error.to_string(), "message");
    handle.join().unwrap();
}

#[test]
fn consuming_operations_do_not_move_shared_payload() {
    let error = anyhow!("message");
    let clone = error.clone();

    assert!(error.downcast::<String>().is_err());
    assert!(clone.downcast::<String>().is_err());

    let error = anyhow!("message");
    let clone = error.clone();
    let boxed: Box<dyn std::error::Error + Send + Sync> = error.into_boxed_dyn_error();
    assert_eq!(boxed.to_string(), "message");
    assert_eq!(clone.to_string(), "message");
}

#[test]
fn nested_shared_error_remains_valid() {
    let inner = anyhow!("inner");
    let inner_clone = inner.clone();
    let outer = inner.context("outer");

    assert!(outer.downcast::<String>().is_err());
    assert_eq!(inner_clone.to_string(), "inner");
}

#[test]
fn unique_error_still_downcasts() {
    let error: Error = anyhow!(String::from("message"));
    assert_eq!(error.downcast::<String>().unwrap(), "message");
}
