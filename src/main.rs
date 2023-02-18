//use std::sync::{Arc, Mutex}; 
use ros2_esp32_interfaces::msg::PinValues;
use ros2_esp32_interfaces::srv::{SetPin, SetPin_Request, SetPin_Response};
use rclrs::{Publisher, rmw_request_id_t};


fn handle_subscription(msg: PinValues) {
    println!("{:?}", msg.vals);
}

fn run_publisher(publisher: Publisher<PinValues>) {
    loop {
        use std::time::Duration;
        std::thread::sleep(Duration::from_millis(1000));
        let msg = PinValues{vals: [0.; 10]};
        publisher.publish(msg).unwrap();
    }
}

fn handle_set_pin(_: &rmw_request_id_t, req: SetPin_Request) -> SetPin_Response {
    println!("{:?}", req);
    SetPin_Response{is_ok: true}
}

fn main() {
    let context = rclrs::Context::new(std::env::args()).unwrap();
    let mut node = rclrs::Node::new(&context, "esp32_interface").unwrap();

    let _subscriber = node.create_subscription(
        "esp32_write_pins",
        rclrs::QOS_PROFILE_DEFAULT,
        move |msg: PinValues| {
            handle_subscription(msg);
        },
    ).unwrap();

    let publisher: Publisher<PinValues> = node.create_publisher(
        "esp32_read_pins", 
        rclrs::QOS_PROFILE_DEFAULT
    ).unwrap();

    let _service = node.create_service::<SetPin, _>("esp32_set_pin", handle_set_pin);

    std::thread::spawn(move || {
        run_publisher(publisher);
    });

    rclrs::spin(&node).unwrap();
}
