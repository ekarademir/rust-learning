use std::sync::Arc;

#[macro_use(app_info_from_cargo_toml)]
extern crate vulkano;

use vulkano::{
    buffer::{
        BufferUsage, 
        CpuAccessibleBuffer
    },
    command_buffer::{
        AutoCommandBufferBuilder,
        DynamicState,
    },
    device::{
        Device, 
        DeviceExtensions, 
        Queue
    },
    framebuffer::{
        Framebuffer, 
        FramebufferAbstract, 
        RenderPassAbstract,
        Subpass, 
    },
    instance::{
        Instance, 
        PhysicalDevice, 
        PhysicalDeviceType
    },
    image::swapchain::SwapchainImage,
    pipeline::{
        GraphicsPipeline,
        viewport::Viewport,
    },
    swapchain,
    swapchain::{
        AcquireError,
        PresentMode, 
        Surface, 
        Swapchain, 
        SwapchainCreationError,
        SurfaceTransform,
    },
    sync,
    sync::{
        GpuFuture,
        FlushError,
    },
};

use vulkano_win::VkSurfaceBuild;

use winit::{
    Event, 
    EventsLoop, 
    Window, 
    WindowBuilder,
    WindowEvent, 
};

struct Vulkan {
    instance: Arc<Instance>,
    surface: Arc<Surface<Window>>,  // Can access to the window by calling `window()`
    event_loop: EventsLoop,
    device: Arc<Device>,
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    swapchain_images: Vec<Arc<SwapchainImage<Window>>>
}

struct App {
    
}

#[derive(Default, Debug, Clone)]
struct Vertex { position: [f32; 2] }

fn main() {
    let mut vulkan = initialize_vulkan();

    let vertex_buffer = {
        vulkano::impl_vertex!(Vertex, position);

        CpuAccessibleBuffer::from_iter(vulkan.device.clone(), BufferUsage::all(), [
            Vertex { position: [ 1.0, -1.0] },
            Vertex { position: [ 1.0,  1.0] },
            Vertex { position: [-1.0, -1.0] },
            Vertex { position: [-1.0,  1.0] },
        ].iter().cloned()).expect("VertexBufferCreationError")
    };

    let vs = vs::Shader::load(vulkan.device.clone()).expect("VertexShaderCreationError");
    let fs = fs::Shader::load(vulkan.device.clone()).expect("FragmentShaderCreationError");

    let render_pass = Arc::new(vulkano::single_pass_renderpass!(
        vulkan.device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: vulkan.swapchain.format(),
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    ).unwrap());

    let pipeline = Arc::new(
        GraphicsPipeline::start()
            .vertex_input_single_buffer::<Vertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_strip()
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(vulkan.device.clone())
            .unwrap()
    );

    let mut dynamic_state = DynamicState {
        line_width: None,
        viewports: None,
        scissors: None,
        compare_mask: None,
        write_mask: None,
        reference: None,
    };

    let mut framebuffers = update_viewport(
        &vulkan.swapchain_images, 
        render_pass.clone(), 
        &mut dynamic_state,
    );

    // LOOP
    let mut recreate_swapchain = false;
    let mut previous_frame_end = Box::new(sync::now(vulkan.device.clone())) as Box<dyn GpuFuture>;

    loop {
        previous_frame_end.cleanup_finished();

        if recreate_swapchain {
            let dimensions = if let Some(dimensions) = vulkan.surface.window().get_inner_size() {
                let dimensions: (u32, u32) = dimensions.to_physical(vulkan.surface.window().get_hidpi_factor()).into();
                [dimensions.0, dimensions.1]
            } else {
                panic!("WindowNoLongerExists");
            };

            let (new_swapchain, new_images) = match vulkan.swapchain.recreate_with_dimension(dimensions) {
                Ok(r) => r,
                Err(SwapchainCreationError::UnsupportedDimensions) => continue,
                Err(err) => panic!("{:?}", err),
            };

            vulkan.swapchain = new_swapchain;

            framebuffers = update_viewport(
                &new_images, 
                render_pass.clone(), 
                &mut dynamic_state,
            );

            recreate_swapchain = false;
        }

        // Acquire the last image that can be drawn to the surface
        let (image_num, acquire_future) = match swapchain::acquire_next_image(
            vulkan.swapchain.clone(), None
        ) {
            Ok(r) => r,
            Err(AcquireError::OutOfDate) => {recreate_swapchain = true; continue;},
            Err(err) => panic!("{:?}", err),
        };

        let clear_values = vec![[0.2, 0.5, 0.9, 1.0].into()];

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(vulkan.device.clone(), vulkan.queue.family()).unwrap()
            .begin_render_pass(framebuffers[image_num].clone(), false, clear_values)
            .unwrap()
            
            .draw(pipeline.clone(), &dynamic_state, vertex_buffer.clone(), (), ())
            .unwrap()
            
            .end_render_pass()
            .unwrap()
            
            .build()
            .unwrap();
        
        let future = previous_frame_end.join(acquire_future)
            .then_execute(vulkan.queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(vulkan.queue.clone(), vulkan.swapchain.clone(), image_num)
            .then_signal_fence_and_flush();
        
        match future {
            Ok(future) => {
                future.wait(None).unwrap();
                previous_frame_end = Box::new(future) as Box<_>;
            }
            Err(FlushError::OutOfDate) => {
                recreate_swapchain = true;
                previous_frame_end = Box::new(sync::now(vulkan.device.clone())) as Box<_>;
            }
            Err(e) => {
                println!("{:?}", e);
                previous_frame_end = Box::new(sync::now(vulkan.device.clone())) as Box<_>;
            }
        }

        let mut done = false;
        vulkan.event_loop.poll_events(|ev| {
            match ev {
                Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => done = true,
                Event::WindowEvent { event: WindowEvent::Resized(_), .. } => recreate_swapchain = true,
                _ => ()
            }
        });
        if done {return;}
    }
}

fn initialize_vulkan() -> Vulkan {
    // Create Vulkan instance
    let instance = {
        let extensions = vulkano_win::required_extensions();
        Instance::new(None, &extensions, None).expect("VulkanInstanceCreationError")
    };

    // Get the first Vulkan supporting graphics card
    let mut physical_device_index: Option<usize> = None;
    for dev in PhysicalDevice::enumerate(&instance) {
        match dev.ty() {
            PhysicalDeviceType::Cpu => continue,
            PhysicalDeviceType::Other => continue,
            _ => {
                physical_device_index = Some(dev.index());
                break;
            },
        }
    }
    let physical_device = match physical_device_index {
        Some(idx) => PhysicalDevice::from_index(&instance, idx)
            .expect("VulkanPhysicalDeviceNotFoundError"),
        None => panic!("VulkanCantFindPhysicalDeviceError"),
    };  // TODO Save device capabilities somewhere

    // Create event loop and Vulkan drawable surface
    let event_loop = EventsLoop::new();
    let surface = WindowBuilder::new()
        .build_vk_surface(&event_loop, instance.clone())
        .expect("VulkanSurfaceCreationError");

    let queue_family = physical_device
        .queue_families()
        .find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        })
        .expect("VulkanCantFindSuitableQueueFamilyError");
    
    // Create logical device
    let device_ext = DeviceExtensions {
        khr_swapchain: true,
        .. DeviceExtensions::none()
    };

    let (device, mut queues) = Device::new(
        physical_device, 
        physical_device.supported_features(), 
        &device_ext, 
        [(queue_family, 0.5)].iter().cloned()
    ).expect("VulkanLogicalDeviceCreationError");
    let queue = queues.next().unwrap();

    let (swapchain, swapchain_images) = {
        let capabilities = surface.capabilities(physical_device).unwrap();
        let image_usage = capabilities.supported_usage_flags;
        let composite_alpha = capabilities.supported_composite_alpha.iter().next().unwrap();
        let (format, _color_depth) = capabilities.supported_formats[0];

        let initial_dimensions = if let Some(dimensions) = surface.window().get_inner_size() {
            let dimensions: (u32, u32) = dimensions.to_physical(surface.window().get_hidpi_factor()).into();
            [dimensions.0, dimensions.1]
        } else {
            panic!("WindowNoLongerExists");
        };
        
        Swapchain::new(
            device.clone(),
            surface.clone(),
            capabilities.min_image_count,
            format,
            initial_dimensions,
            1,  // Layers
            image_usage,
            &queue,  // sharing mode
            SurfaceTransform::Identity,
            composite_alpha,
            PresentMode::Fifo,
            true,
            None
        ).expect("VulkanSwapchainCreationError")
    };
    
    Vulkan {
        instance,
        surface,
        event_loop,
        device,
        queue,
        swapchain,
        swapchain_images
    }
}

fn update_viewport(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
    dynamic_state: &mut DynamicState,
) -> Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
    let dimension = images[0].dimensions();

    let viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [dimension[0] as f32, dimension[1] as f32],
        depth_range: 0.0 .. 1.0,
    };
    dynamic_state.viewports = Some(vec!(viewport));

    images.iter().map(|image| {
        Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image.clone())
                .unwrap()
                .build()
                .unwrap()
        ) as Arc<dyn FramebufferAbstract + Send + Sync>
    }).collect::<Vec<_>>()
}

mod vs {
    vulkano_shaders::shader!{
        ty: "vertex",
        src: "
#version 450

layout(location = 0) in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}"
    }
}


mod fs {
    vulkano_shaders::shader!{
        ty: "fragment",
        src: "
#version 450

layout(location = 0) out vec4 f_color;

void main() {
    f_color = vec4(1.0, 0.0, 0.0, 1.0);
}
"
    }
}
