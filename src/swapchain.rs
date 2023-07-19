use crate::device::{Device, QueueFamily, SwapChainSupportDetails};
use ash::{
    vk::{self, TaggedStructure},
    Entry,
};
use std::ptr::{self};

const MAX_FRAMES_IN_FLIGHT: usize = 2;

struct SwapchainKHR {
    swapchain_loader: ash::extensions::khr::Swapchain,
    swapchain: vk::SwapchainKHR,
}

pub struct Swapchain {
    swapchain_image_format: Option<vk::Format>,
    swapchain_depth_format: Option<vk::Format>,
    swapchain_extent: Option<vk::Extent2D>,
    swapchain_framebuffers: Option<Vec<vk::Framebuffer>>,
    renderpass: Option<vk::RenderPass>,
    depth_images: Option<Vec<vk::Image>>,
    depth_image_memories: Option<Vec<vk::DeviceMemory>>,
    depth_image_views: Option<Vec<vk::ImageView>>,
    swapchain_images: Option<Vec<vk::Image>>,
    pub swapchain_image_views: Option<Vec<vk::ImageView>>,
    window_extent: Option<vk::Extent2D>,
    swapchain: Option<SwapchainKHR>,
    image_available_semaphores: Option<Vec<vk::Semaphore>>,
    render_finished_semaphores: Option<Vec<vk::Semaphore>>,
    in_flight_fences: Option<Vec<vk::Fence>>,
    images_in_flight: Option<Vec<vk::Fence>>,
    current_frame: usize,
}

impl Swapchain {
    pub fn new(device: &Device, window_extent: vk::Extent2D) -> Swapchain {
        let mut swapchain = Swapchain::default();
        Swapchain::init(&mut swapchain, None, device);

        return swapchain;
    }

    pub fn renew(
        device: &Device,
        window_extent: vk::Extent2D,
        previous: &mut Swapchain,
    ) -> Swapchain {
        let mut swapchain = Swapchain::default();

        Swapchain::init(&mut swapchain, Some(previous), device);

        return swapchain;
    }

    fn init(self: &mut Swapchain, old_swapchain: Option<&mut Swapchain>, device: &Device) {
        if old_swapchain.is_some() {
            Swapchain::create_swapchain(
                self,
                device,
                Some(&mut old_swapchain.unwrap().swapchain.as_ref().unwrap()),
            );
        } else {
            Swapchain::create_swapchain(self, device, None);
        }
        Swapchain::create_image_views(self, device);
        Swapchain::create_renderpass(self, device);
        Swapchain::create_depth_resources(self, device);
        Swapchain::create_framebuffers(self, device);
        Swapchain::create_sync_objects(self, device);
    }

    pub fn default() -> Swapchain {
        return Swapchain {
            swapchain_image_format: None,
            swapchain_depth_format: None,
            swapchain_extent: None,
            swapchain_framebuffers: None,
            renderpass: None,
            depth_images: None,
            depth_image_memories: None,
            depth_image_views: None,
            swapchain_images: Some(Vec::new()),
            swapchain_image_views: Some(Vec::new()),
            window_extent: None,
            swapchain: None,
            image_available_semaphores: Some(Vec::new()),
            render_finished_semaphores: Some(Vec::new()),
            in_flight_fences: Some(Vec::new()),
            images_in_flight: Some(Vec::new()),
            current_frame: 0,
        };
    }

    pub fn get_framebuffer(&self, index: usize) -> vk::Framebuffer {
        return self.swapchain_framebuffers.as_ref().unwrap()[index];
    }

    pub fn get_renderpass(&self) -> vk::RenderPass {
        return self.renderpass.unwrap();
    }

    pub fn get_image_view(&self, index: usize) -> vk::ImageView {
        return self.swapchain_image_views.as_ref().unwrap()[index];
    }

    pub fn image_count(&self) -> usize {
        return self.swapchain_images.as_ref().unwrap().len();
    }

    pub fn get_swapchain_image_format(&self) -> vk::Format {
        return self.swapchain_image_format.unwrap();
    }

    pub fn get_swapchain_extent(&self) -> vk::Extent2D {
        return self.swapchain_extent.unwrap();
    }

    pub fn extent_aspect_ratio(&self) -> f64 {
        return self.swapchain_extent.unwrap().width as f64
            / self.swapchain_extent.unwrap().height as f64;
    }

    pub fn find_depth_format(&self, device: &Device) -> vk::Format {
        return device.find_support_format(
            &[
                vk::Format::D32_SFLOAT,
                vk::Format::D32_SFLOAT_S8_UINT,
                vk::Format::D24_UNORM_S8_UINT,
            ],
            vk::ImageTiling::OPTIMAL,
            vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT,
        );
    }

    pub fn acquire_next_image(image_index: u32) /*-> vk::Result*/ {}

    pub fn submit_command_buffers(buffers: vk::CommandBuffer, image_index: u32) /*-> vk::Result*/ {}

    pub fn compare_swap_formats(&self, swapchain: &Swapchain) -> bool {
        return swapchain.swapchain_depth_format.unwrap() == self.swapchain_depth_format.unwrap()
            && swapchain.swapchain_image_format.unwrap() == self.swapchain_image_format.unwrap();
    }

    fn create_swapchain(
        self: &mut Swapchain,
        device: &Device,
        old_swapchain: Option<&SwapchainKHR>,
    ) {
        let swapchain_support: SwapChainSupportDetails = device.get_swapchain_support();

        let surface_format: vk::SurfaceFormatKHR = self
            .choose_swap_surface_format(&swapchain_support.surface_formats.unwrap())
            .unwrap();
        let present_mode: vk::PresentModeKHR = self
            .choose_swap_present_mode(&swapchain_support.present_modes.unwrap())
            .unwrap();
        let extent: vk::Extent2D =
            self.choose_swap_extent(&swapchain_support.surface_capabilities.unwrap());

        let mut image_count: u32 = swapchain_support
            .surface_capabilities
            .unwrap()
            .min_image_count
            + 1;
        if swapchain_support
            .surface_capabilities
            .unwrap()
            .max_image_count
            > 0
            && image_count
                > swapchain_support
                    .surface_capabilities
                    .unwrap()
                    .max_image_count
        {
            image_count = swapchain_support
                .surface_capabilities
                .unwrap()
                .max_image_count;
        }

        let mut create_info: vk::SwapchainCreateInfoKHR = vk::SwapchainCreateInfoKHR::default();

        let swapchain_loader = ash::extensions::khr::Swapchain::new(
            &device.instance.as_ref().unwrap(),
            &device.device(),
        );

        create_info.s_type = vk::StructureType::SWAPCHAIN_CREATE_INFO_KHR;
        create_info.surface = device.surface();
        create_info.min_image_count = image_count;
        create_info.image_format = surface_format.format;
        create_info.image_color_space = surface_format.color_space;
        create_info.image_extent = extent;
        create_info.image_array_layers = 1;
        create_info.image_usage = vk::ImageUsageFlags::COLOR_ATTACHMENT;

        let indices: QueueFamily = device.find_physical_queue_families();
        let queue_family_indices: [u32; 2] = [indices.graphics_family, indices.present_family];

        if indices.graphics_family != indices.present_family {
            create_info.image_sharing_mode = vk::SharingMode::CONCURRENT;
            create_info.queue_family_index_count = 0;
            create_info.p_queue_family_indices = queue_family_indices.as_ptr();
        } else {
            create_info.image_sharing_mode = vk::SharingMode::EXCLUSIVE;
            create_info.queue_family_index_count = 0;
            create_info.p_queue_family_indices = ptr::null();
        }

        create_info.pre_transform = swapchain_support
            .surface_capabilities
            .unwrap()
            .current_transform;
        create_info.composite_alpha = vk::CompositeAlphaFlagsKHR::OPAQUE;
        create_info.present_mode = present_mode;
        create_info.clipped = vk::TRUE;

        if old_swapchain.is_none() {
            create_info.old_swapchain = vk::SwapchainKHR::default();
        } else {
            create_info.old_swapchain = old_swapchain.unwrap().swapchain;
        }

        unsafe {
            let _swapchain = swapchain_loader
                .create_swapchain(&create_info, None)
                .expect("Failed to create swapchain!");

            self.swapchain = Some(SwapchainKHR {
                swapchain: _swapchain,
                swapchain_loader: swapchain_loader,
            });

            self.swapchain_images = Some(
                self.swapchain
                    .as_ref()
                    .unwrap()
                    .swapchain_loader
                    .get_swapchain_images(self.swapchain.as_ref().unwrap().swapchain)
                    .unwrap(),
            );

            self.swapchain_image_format = Some(surface_format.format);
            self.swapchain_extent = Some(extent);
        }
    }

    fn create_image_views(self: &mut Swapchain, device: &Device) {
        self.swapchain_image_views.as_mut().unwrap().resize(
            self.swapchain_images.as_ref().unwrap().len(),
            vk::ImageView::default(),
        );
        for i in 0..self.swapchain_images.as_ref().unwrap().len() {
            let mut view_info: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo::default();
            view_info.s_type = vk::StructureType::IMAGE_VIEW_CREATE_INFO;
            view_info.image = self.swapchain_images.as_ref().unwrap()[i];
            view_info.view_type = vk::ImageViewType::TYPE_2D;
            view_info.format = self.swapchain_image_format.unwrap();
            view_info.subresource_range.aspect_mask = vk::ImageAspectFlags::COLOR;
            view_info.subresource_range.base_mip_level = 0;
            view_info.subresource_range.level_count = 1;
            view_info.subresource_range.base_array_layer = 0;
            view_info.subresource_range.layer_count = 1;

            unsafe {
                self.swapchain_image_views.as_mut().unwrap()[i] = device
                    .device()
                    .create_image_view(&view_info, None)
                    .expect("Failed to create image view!");
            }
        }
    }

    fn create_renderpass(self: &mut Swapchain, device: &Device) {
        let color_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: self.get_swapchain_image_format(),
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        };

        let color_attachment_ref = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let depth_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format: self.find_depth_format(device),
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::DONT_CARE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let depth_attachment_ref = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        };

        let subpasses = [vk::SubpassDescription {
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_ref,
            p_depth_stencil_attachment: &depth_attachment_ref,
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: ptr::null(),
            p_resolve_attachments: ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: ptr::null(),
        }];

        let dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        }];

        let attachments: [vk::AttachmentDescription; 2] = [color_attachment, depth_attachment];

        let create_info: vk::RenderPassCreateInfo = vk::RenderPassCreateInfo {
            flags: vk::RenderPassCreateFlags::empty(),
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: subpasses.len() as u32,
            p_subpasses: subpasses.as_ptr(),
            dependency_count: dependencies.len() as u32,
            p_dependencies: dependencies.as_ptr(),
            p_next: std::ptr::null(),
        };

        unsafe {
            self.renderpass = Some(
                device
                    .device()
                    .create_render_pass(&create_info, None)
                    .expect("Failed to create render pass!"),
            );
        }
    }

    fn create_framebuffers(self: &mut Swapchain, device: &Device) {
        for i in 0..self.swapchain_images.as_ref().unwrap().len() {
            let mut view_info: vk::ImageViewCreateInfo = vk::ImageViewCreateInfo::default();
            view_info.s_type = vk::StructureType::IMAGE_VIEW_CREATE_INFO;
            view_info.image = self.swapchain_images.as_ref().unwrap()[i];
            view_info.view_type = vk::ImageViewType::TYPE_2D;
            view_info.format = self.swapchain_image_format.unwrap();
            view_info.subresource_range.aspect_mask = vk::ImageAspectFlags::COLOR;
            view_info.subresource_range.base_mip_level = 0;
            view_info.subresource_range.level_count = 1;
            view_info.subresource_range.base_array_layer = 0;
            view_info.subresource_range.layer_count = 1;

            unsafe {
                self.swapchain_image_views.as_mut().unwrap().push(
                    device
                        .device()
                        .create_image_view(&view_info, None)
                        .expect("Failed to create an image view"),
                );
            }
        }
    }

    fn create_sync_objects(self: &mut Swapchain, device: &Device) {
        self.image_available_semaphores
            .as_mut()
            .unwrap()
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Semaphore::default());
        self.render_finished_semaphores
            .as_mut()
            .unwrap()
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Semaphore::default());
        self.in_flight_fences
            .as_mut()
            .unwrap()
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Fence::default());
        self.images_in_flight
            .as_mut()
            .unwrap()
            .resize(MAX_FRAMES_IN_FLIGHT, vk::Fence::default());

        let semaphore_info: vk::SemaphoreCreateInfo = vk::SemaphoreCreateInfo::default();

        let fence_info: vk::FenceCreateInfo = vk::FenceCreateInfo {
            s_type: vk::StructureType::FENCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::FenceCreateFlags::SIGNALED,
        };

        for i in 0..MAX_FRAMES_IN_FLIGHT {
            unsafe {
                self.image_available_semaphores.as_mut().unwrap()[i] = device
                    .device()
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create the first sync object semaphore!");
                self.render_finished_semaphores.as_mut().unwrap()[i] = device
                    .device()
                    .create_semaphore(&semaphore_info, None)
                    .expect("Failed to create the second sync object semaphore!");
                self.in_flight_fences.as_mut().unwrap()[i] = device
                    .device()
                    .create_fence(&fence_info, None)
                    .expect("Failed to create the sync object fence!");
            }
        }
    }

    fn create_depth_resources(self: &mut Swapchain, device: &Device) {
        let depth_format: vk::Format = self.find_depth_format(device);
        self.swapchain_depth_format = Some(depth_format);
    }

    fn choose_swap_surface_format(
        &self,
        available_formats: &Vec<vk::SurfaceFormatKHR>,
    ) -> Option<vk::SurfaceFormatKHR> {
        for available_format in available_formats {
            if available_format.format == vk::Format::B8G8R8A8_SRGB
                && available_format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            {
                return Some(*available_format);
            }
        }
        return None;
    }

    fn choose_swap_present_mode(
        &self,
        available_present_modes: &Vec<vk::PresentModeKHR>,
    ) -> Option<vk::PresentModeKHR> {
        for present_mode in available_present_modes {
            if *present_mode == vk::PresentModeKHR::MAILBOX {
                println!("Present mode: Mailbox");
                return Some(*present_mode);
            } else if *present_mode == vk::PresentModeKHR::FIFO {
                println!("Present mode: V-Sync");
                return Some(*present_mode);
            }
        }

        return None;
    }

    fn choose_swap_extent(&self, surface_capabilites: &vk::SurfaceCapabilitiesKHR) -> vk::Extent2D {
        if surface_capabilites.current_extent.width != u32::MAX {
            return surface_capabilites.current_extent;
        } else {
            let mut actual_extent: vk::Extent2D = self.window_extent.unwrap();
            actual_extent.width = std::cmp::max(
                surface_capabilites.min_image_extent.width,
                std::cmp::min(
                    surface_capabilites.max_image_extent.width,
                    actual_extent.width,
                ),
            );

            actual_extent.height = std::cmp::max(
                surface_capabilites.min_image_extent.height,
                std::cmp::min(
                    surface_capabilites.max_image_extent.height,
                    actual_extent.height,
                ),
            );

            return actual_extent;
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::device::Device;
    use crate::window::Window;
    use crate::swapchain::Swapchain;

    #[test]
    fn create_image_views_test(){
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Visible(true));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        
        let window = Window::new(&mut glfw,"Revier:DEV BUILD #1",640,480);
        let device = Device::new(&window,&glfw);
        let mut swapchain = Swapchain::default();

        Swapchain::create_image_views(&mut swapchain, &device);

        assert_eq!(swapchain.swapchain_image_views.as_ref().unwrap().len() > 0,true);
    }

    #[test]
    fn create_render_pass_test(){
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::WindowHint::Visible(true));
        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        
        let window = Window::new(&mut glfw,"Revier:DEV BUILD #1",640,480);
        let device = Device::new(&window,&glfw);
        let mut swapchain = Swapchain::default();

        Swapchain::create_renderpass(&mut swapchain, &device);

        assert_eq!(swapchain.renderpass.is_some(),true)
    }
}