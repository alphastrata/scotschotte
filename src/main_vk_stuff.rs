use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

fn main() {
    println!("Hello, world!");

    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("Failed to create vulkan instance");
    println!("Created a Vulkan Instance");

    let mut physical_itr = PhysicalDevice::enumerate(&instance);
    // TODO: Actually go through devices and find the "best" one
    let found_device = physical_itr.next().expect("No physical devices found");

    for family in found_device.queue_families() {
        println!("Found a queue with {:?} queue(s)", family.queues_count());
    }

    let queue_family = found_device
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Couldn't find graphics queue family");
    let (device, mut queues) = {
        Device::new(
            found_device,
            &Features::none(),
            &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("Failed to create a device")
    };

    // TODO: Should definitely not just claim the first queue and be done with it...
    let queue = queues.next().unwrap();
}
