use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance");
    let physical = PhysicalDevice::enumerate(&instance).next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!("Found family queue family with {:?} queues", family.queues_count());
    }

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
 
    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
            [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };

    let queue = queues.next().unwrap();

    let source_content = 0..64;
    let dest_content = (0..64).map(|_| 0);

    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                     source_content).expect("Failed to create buffer");
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                   dest_content).expect("Failed to create buffer");


}
