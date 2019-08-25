// -----------------------------------------
// A very, simple and niave app which opens photos (jpeg or png)
// and sends them via the Vulkan API to a GPU buffer which displays them
// on screen directly (not via the cpu)
// -----------------------------------------
#[allow(unused_imports)]
use vulkano::buffer::{
    BufferUsage, CpuAccessibleBuffer, CpuBufferPool, DeviceLocalBuffer, ImmutableBuffer,
};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::device::{Device, DeviceExtensions};
use vulkano::format::Format;
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, RenderPassAbstract, Subpass};
use vulkano::image::{Dimensions, ImmutableImage, SwapchainImage};
use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::sampler::{Filter, MipmapMode, Sampler, SamplerAddressMode};
use vulkano::swapchain;
use vulkano::swapchain::{
    AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError,
};
use vulkano::sync;
use vulkano::sync::{FlushError, GpuFuture};

use vulkano_win::VkSurfaceBuild;

#[allow(unused_imports)]
use image::{DynamicImage, GenericImageView, ImageBuffer, ImageFormat};
use winit::{Event, EventsLoop, Window, WindowBuilder, WindowEvent};

use std::sync::Arc;
extern crate stopwatch;
use stopwatch::Stopwatch;

fn main() {
    let sw = Stopwatch::start_new(); // The timer is here for debugging atm..
                                     // Setup API handles
    let extensions = vulkano_win::required_extensions();
    let instance = Instance::new(None, &extensions, None).unwrap();
    let physical = PhysicalDevice::enumerate(&instance).next().unwrap();
    println!(
        ">>>DEBUG:GPU FOUND={} IN: {}ms",
        physical.name(),
        sw.elapsed_ms()
    );
    // ==========================================================
    // Setup a Window for our App to live in... we will close this properly later
    let mut events_loop = EventsLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&events_loop, instance.clone())
        .unwrap();
    let window = surface.window();
    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics() && surface.is_supported(q).unwrap_or(false))
        .unwrap();
    // What features & capabilities does your hardware have?
    #[allow(non_snake_case)]
    let PRIORITY = 0.9;
    let extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };
    //timestamp_msg(sw, "CREATED EXTENSIONS");
    let (device, mut queues) = Device::new(
        physical,
        physical.supported_features(),
        &extensions,
        [(queue_family, PRIORITY)].iter().cloned(),
    )
    .unwrap();
    // ==========================================================
    // get a 'queue' suitable for our needs: i.e one that supports graphics and drawing stuff
    // to a screen etc.
    let queue = queues.next().unwrap();

    // ==========================================================
    // Build a fucking swapchain.
    let (mut swapchain, images) = {
        let capabilities = surface.capabilities(physical).unwrap();
        let usage = capabilities.supported_usage_flags;
        let alpha = capabilities
            .supported_composite_alpha
            .iter()
            .next()
            .unwrap();
        let format = capabilities.supported_formats[0].0;
        let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
            let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
            [dimensions.0, dimensions.1]
        } else {
            return;
        };
        Swapchain::new(
            device.clone(),
            surface.clone(),
            capabilities.min_image_count,
            format,
            initial_dimensions,
            1,
            usage,
            &queue,
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            true,
            None,
        )
        .unwrap()
    };
    //timestamp_msg(sw, "CREATED SWAPCHAIN AND IMAGES");
    // ==========================================================
    // Prepare structs to send to the GPU buffer:
    #[derive(Default, Debug, Clone)]
    struct Vertex {
        position: [f32; 2],
    }
    vulkano::impl_vertex!(Vertex, position);
    // Prepare a buffer for the GPU which your CPU can access (Read and Write data to)
    let vertex_buffer = CpuAccessibleBuffer::<[Vertex]>::from_iter(
        device.clone(),
        BufferUsage::vertex_buffer(), // BufferUsage::all() is reccomended for prototyping...
        // BufferUsage::all(),
        [
            // Images have a width and height of 2,
            // it's the coordinate system you're used to seeing from graphs but rotated by 90degrees.
            Vertex {
                position: [-1.0, -1.0], // TOP LEFT
            },
            Vertex {
                position: [-1.0, 1.0], // BOTTOM LEFT
            },
            Vertex {
                position: [1.0, -1.0], // TOP RIGHT
            },
            Vertex {
                position: [1.0, 1.0], // BOTTOM RIGHT
            },
        ]
        .iter()
        .cloned(),
    )
    .unwrap();

    // ==========================================================
    // Let's load up some shaders!
    mod vs {
        vulkano_shaders::shader! {
        ty: "vertex",
        path: "shaders/vertex.glsl"
        }
    }

    mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            path: "shaders/fragment.glsl"
        }

    }
    let vs = vs::Shader::load(device.clone()).unwrap();
    let fs = fs::Shader::load(device.clone()).unwrap();

    // Create a render pass
    let render_pass = Arc::new(
        vulkano::single_pass_renderpass!(device.clone(),
        // We only need a single pass because we're displaying an image, not something dynamic
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color : [color],
            depth_stencil: {}
        }
        )
        .unwrap(),
    );
    // ==========================================================
    // Create Textures
    let (texture, tex_future) = {
        // Note this image is open currently ONLY in CPU (HOST) Memory...
        let image = image::load_from_memory_with_format(
            include_bytes!("../test_imgs/test2.jpg"),
            ImageFormat::JPEG,
        )
        .unwrap()
        .to_rgba();

        // ==========================================================
        // THIS [LOADING IMAGES] WILL BE EXTREMELY SLOW IF YOU RUN IN DEBUG MODE!!
        // ==========================================================
        let image_dimensions = &image.dimensions();
        let image_data = image.into_raw().clone();
        ImmutableImage::from_iter(
            image_data.iter().cloned(),
            Dimensions::Dim2d {
                width: image_dimensions.0,
                height: image_dimensions.1,
            },
            Format::R8G8B8A8Srgb,
            queue.clone(),
        )
        .unwrap()
    };

    // ==========================================================
    // Create a sampler
    let sampler = Sampler::new(
        device.clone(),
        // Must find a way to swap these filters' between Nearest and Linear depending on screen res & whether or not the user
        // is upsizing or downsizing...
        Filter::Linear,             // Magnification filter
        Filter::Linear,             // Minimisation filter
        MipmapMode::Linear, //mipmap_mode defines how the implementation should choose which mipmap to use.
        SamplerAddressMode::Repeat, // Address_u : how should the implementation should behave when sampling outside
        // of the texture coordinates range [0.0, 1.0] for (Adresses u, w & v).
        SamplerAddressMode::Repeat, // Address_w
        SamplerAddressMode::Repeat, // Address_v
        0.0, //mip_lod_bias is a value to add to // WTF DOES THATE MEAN? @scotty
        1.0, //max_anisotropy Anisotropic filtering (AF) is a method of enhancing the image
        // quality of textures on surfaces that are far away and steeply angled with respect
        // to the point of view.
        0.0, //min_lod is the minimum mipmap level to use
        0.0, //max_lod is the maximum of the above
    )
    .unwrap();
    // ==========================================================
    // Let's fire up the graphics pipeline
    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_strip() // WTF DOES THIS DO?
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .blend_alpha_blending() // WTF DOES THIS DO?
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap(),
    );
    // Create a descriptor set -> Ask Scotty to explain what these
    let set = Arc::new(
        PersistentDescriptorSet::start(pipeline.clone(), 0)
            .add_sampled_image(texture.clone(), sampler.clone())
            .unwrap()
            .build()
            .unwrap(),
    );
    // Dynamic states are just something we can draw on :P, and by 'draw' I mean something that'll
    // show up on your screen, in the window we created up the top.
    let mut dynamic_state = DynamicState {
        line_width: None,
        viewports: None,
        scissors: None,
    };
    // ==========================================================
    // Create a frame buffer
    let mut framebuffers =
        window_size_dependent_setup(&images, render_pass.clone(), &mut dynamic_state);
    // Start the swapchain as false the first time around
    let mut recreate_swapchain = false;
    // The previous frame is a whole bunch of unknown fuckery so it can't be stored on the stack
    let mut previous_frame_end = Box::new(tex_future) as Box<dyn GpuFuture>;

    loop {
        previous_frame_end.cleanup_finished(); // destroys locks on GPU memory and frees GPU resources.
        if recreate_swapchain {
            // Is true
            let dimensions = if let Some(dimensions) = window.get_inner_size() {
                let dimensions: (u32, u32) =
                    dimensions.to_physical(window.get_hidpi_factor()).into();

                [dimensions.0, dimensions.1]
            } else {
                return;
            };
            // Create a new swapchain..
            let (new_swapchain, new_images) = match swapchain.recreate_with_dimension(dimensions) {
                Ok(r) => r,
                Err(SwapchainCreationError::UnsupportedDimensions) => continue,
                Err(err) => panic!("\nFATAL ERROR:\nms\t\t{:?}\n\n", err),
            };
            swapchain = new_swapchain;
            // Create new framebuffer
            framebuffers =
                window_size_dependent_setup(&new_images, render_pass.clone(), &mut dynamic_state);
            recreate_swapchain = false;
        }
        // ** I think this is checking to see whether or not there's a newer frame avaialbe... @scotty
        let (image_num, future) = match swapchain::acquire_next_image(swapchain.clone(), None) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {
                recreate_swapchain = true;
                continue;
            }
            Err(err) => panic!("\nFATAL ERROR:\nms\t\t{:?}\n\n", err),
        };

        let zero_out_values = vec![[0.0, 0.0, 0.0, 1.0].into()];
        let cmd_buffer =
            AutoCommandBufferBuilder::primary_one_time_submit(device.clone(), queue.family())
                .unwrap()
                .begin_render_pass(framebuffers[image_num].clone(), false, zero_out_values)
                // What's the bool <Secondary> type arg for?
                .unwrap()
                .draw(
                    pipeline.clone(),
                    &dynamic_state,
                    vertex_buffer.clone(),
                    set.clone(),
                    (), // What might <Constants> be?
                )
                .unwrap()
                .end_render_pass()
                .unwrap()
                .build()
                .unwrap();
        // WTF IS GOING ON HERE ... @scotty
        let future = previous_frame_end
            .join(future) // HOW CAN I PASS SOMETHING ITSELF?
            .then_execute(queue.clone(), cmd_buffer)
            .unwrap()
            .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
            .then_signal_fence_and_flush();

        match future {
            Ok(future) => {
                previous_frame_end = Box::new(future) as Box<_>;
            }
            Err(FlushError::OutOfDate) => {
                recreate_swapchain = true;
                previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
            }
            Err(e) => {
                println!("\nFATAL ERROR:\nms\t\t{:?}\n\n", e);
                previous_frame_end = Box::new(sync::now(device.clone())) as Box<_>;
            }
        }
        // Enable closing and resizing of the window in which our app runs...
        let mut completed = false;
        events_loop.poll_events(|ev| match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => completed = true,

            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => recreate_swapchain = true,

            _ => (),
        });
        if completed {
            return;
        }
    }
}
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>, // I don't understand the Send and Sync part of this..
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    // I don't really understand what a FrameBuffer's abstract is or why it too
    // requires the Send and Sync capabiliites.
    let dimensions = images[0].dimensions();
    println!(
        ">>>DEBUG:RESIZE_EVENT, NOW={:?} * {:?}",
        dimensions[0], dimensions[1],
    );
    // The part of screen we can actually see (you could in theory, render stuff outside of this)
    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimensions[0] as f32, dimensions[1] as f32],
        depth_range: 0.0..1.0,
    };
    dynamic_state.viewports = Some(vec![viewport]); // Could alternative veiwports be like the multiplayer split scene scenario you were describing? @scotty?
                                                    // which would explain why it takes a vec of <viewport>s ?
    images
        .iter()
        .map(|image| {
            Arc::new(
                Framebuffer::start(render_pass.clone())
                    .add(image.clone())
                    .unwrap()
                    .build()
                    .unwrap(),
            ) as Arc<dyn FramebufferAbstract + Send + Sync>
        })
        .collect::<Vec<_>>() // Why split them up with iter() at the start only to collect them again?
                             // I think i've seen a fancy map_and_collect or something like somewhere in the wild that's very 'rusty'
}
