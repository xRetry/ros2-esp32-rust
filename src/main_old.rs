use std::sync::{Arc, Mutex}; 
use std_msgs::msg::String as StringMsg;

struct Esp32Node {
    node: rclrs::Node,
    subscriber: Arc<rclrs::Subscription<StringMsg>>,
    publisher: rclrs::Publisher<StringMsg>,
    data: Arc<Mutex<Option<StringMsg>>>, 
}

impl Esp32Node {
    fn new(context: &rclrs::Context) -> Result<Self, rclrs::RclrsError> {
        let mut node = rclrs::Node::new(context, "esp32_interface")?;

        let data = Arc::new(Mutex::new(None)); 
        let data_cb = Arc::clone(&data);

        let subscriber = node.create_subscription(
            "esp32_write_pins",
            rclrs::QOS_PROFILE_DEFAULT,
            move |msg: StringMsg| {
                *data_cb.lock().unwrap() = Some(msg); 
            },
        )?;

        let publisher = node.create_publisher(
            "esp32_read_pins", 
            rclrs::QOS_PROFILE_DEFAULT
        )?;

        Ok(Self {
            node,
            subscriber,
            publisher,
            data,
        })
    }

    fn republish(&self) -> Result<(), rclrs::RclrsError> {
        if let Some(s) = &*self.data.lock().unwrap() {
            self.publisher.publish(s)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), rclrs::RclrsError> {
    let context = rclrs::Context::new(std::env::args())?;
    let republisher = Arc::new(Esp32Node::new(&context)?);
    let republisher_other_thread = Arc::clone(&republisher);
    std::thread::spawn(move || -> Result<(), rclrs::RclrsError> {
        loop {
            use std::time::Duration;
            std::thread::sleep(Duration::from_millis(1000));
            republisher_other_thread.republish()?;
        }
    });
    rclrs::spin(&republisher.node)
}
