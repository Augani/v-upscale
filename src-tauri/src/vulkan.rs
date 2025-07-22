use ash::vk;
use std::ffi::CString;

pub struct VulkanContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: ash::Device,
    pub queue_family_index: u32,
    pub queue: vk::Queue,
    pub shader_module: vk::ShaderModule,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub pipeline_layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline,
}

impl VulkanContext {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let entry = unsafe { ash::Entry::load()? };
        let instance = Self::create_instance(&entry)?;
        let (physical_device, queue_family_index) = Self::pick_physical_device(&instance)?;
        let device = Self::create_logical_device(&instance, physical_device, queue_family_index)?;
        let queue = unsafe { device.get_device_queue(queue_family_index, 0) };
        let shader_module = Self::create_shader_module(&device)?;
        let descriptor_set_layout = Self::create_descriptor_set_layout(&device)?;
        let pipeline_layout = Self::create_pipeline_layout(&device, &descriptor_set_layout)?;
        let pipeline = Self::create_compute_pipeline(&device, &pipeline_layout, &shader_module)?;

        Ok(VulkanContext {
            entry,
            instance,
            physical_device,
            device,
            queue_family_index,
            queue,
            shader_module,
            descriptor_set_layout,
            pipeline_layout,
            pipeline,
        })
    }

    fn create_instance(entry: &ash::Entry) -> Result<ash::Instance, Box<dyn std::error::Error>> {
        let app_name = std::ffi::CString::new("V-Upscale")?;
        let engine_name = std::ffi::CString::new("No Engine")?;
        let app_info = vk::ApplicationInfo {
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: app_name.as_ptr(),
            application_version: vk::make_api_version(0, 0, 1, 0),
            p_engine_name: engine_name.as_ptr(),
            engine_version: vk::make_api_version(0, 0, 1, 0),
            api_version: vk::API_VERSION_1_3,
            ..Default::default()
        };

        // Enable portability enumeration for MoltenVK compatibility
        let extension_names = [CString::new("VK_KHR_portability_enumeration").unwrap()];
        let extension_names_raw: Vec<*const i8> =
            extension_names.iter().map(|name| name.as_ptr()).collect();

        let create_info = vk::InstanceCreateInfo {
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR,
            p_application_info: &app_info,
            enabled_layer_count: 0,
            pp_enabled_layer_names: std::ptr::null(),
            enabled_extension_count: extension_names_raw.len() as u32,
            pp_enabled_extension_names: extension_names_raw.as_ptr(),
            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&create_info, None)? };
        Ok(instance)
    }

    fn pick_physical_device(instance: &ash::Instance) -> Result<(vk::PhysicalDevice, u32), String> {
        let physical_devices = unsafe {
            instance
                .enumerate_physical_devices()
                .map_err(|e| format!("Failed to enumerate physical devices: {}", e))?
        };

        for device in physical_devices {
            let queue_family_properties =
                unsafe { instance.get_physical_device_queue_family_properties(device) };
            if let Some(index) = queue_family_properties
                .iter()
                .position(|p| p.queue_flags.contains(vk::QueueFlags::COMPUTE) && p.queue_count > 0)
            {
                return Ok((device, index as u32));
            }
        }

        Err("No suitable physical device found".to_string())
    }

    fn create_logical_device(
        instance: &ash::Instance,
        physical_device: vk::PhysicalDevice,
        queue_family_index: u32,
    ) -> Result<ash::Device, vk::Result> {
        let queue_priorities = [1.0];
        let queue_create_info = vk::DeviceQueueCreateInfo {
            s_type: vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceQueueCreateFlags::empty(),
            queue_family_index,
            queue_count: queue_priorities.len() as u32,
            p_queue_priorities: queue_priorities.as_ptr(),
            ..Default::default()
        };

        let device_features = vk::PhysicalDeviceFeatures::default();
        let create_info = vk::DeviceCreateInfo {
            s_type: vk::StructureType::DEVICE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DeviceCreateFlags::empty(),
            queue_create_info_count: 1,
            p_queue_create_infos: &queue_create_info,
            enabled_extension_count: 0,
            pp_enabled_extension_names: std::ptr::null(),
            p_enabled_features: &device_features,
            ..Default::default()
        };

        unsafe { instance.create_device(physical_device, &create_info, None) }
    }

    fn create_shader_module(
        device: &ash::Device,
    ) -> Result<vk::ShaderModule, Box<dyn std::error::Error>> {
        let shader_code = include_bytes!("../shaders/upscale.spv");
        let create_info = vk::ShaderModuleCreateInfo {
            s_type: vk::StructureType::SHADER_MODULE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::ShaderModuleCreateFlags::empty(),
            code_size: shader_code.len(),
            p_code: shader_code.as_ptr() as *const u32,
            ..Default::default()
        };

        let shader_module = unsafe { device.create_shader_module(&create_info, None)? };
        Ok(shader_module)
    }

    fn create_descriptor_set_layout(
        device: &ash::Device,
    ) -> Result<vk::DescriptorSetLayout, vk::Result> {
        let bindings = [
            // Input image buffer
            vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::COMPUTE,
                p_immutable_samplers: std::ptr::null(),
                _marker: std::marker::PhantomData,
            },
            // Output image buffer
            vk::DescriptorSetLayoutBinding {
                binding: 1,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::COMPUTE,
                p_immutable_samplers: std::ptr::null(),
                _marker: std::marker::PhantomData,
            },
        ];

        let create_info = vk::DescriptorSetLayoutCreateInfo {
            s_type: vk::StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            binding_count: bindings.len() as u32,
            p_bindings: bindings.as_ptr(),
            ..Default::default()
        };
        unsafe { device.create_descriptor_set_layout(&create_info, None) }
    }

    fn create_pipeline_layout(
        device: &ash::Device,
        descriptor_set_layout: &vk::DescriptorSetLayout,
    ) -> Result<vk::PipelineLayout, vk::Result> {
        let layouts = [*descriptor_set_layout];
        let push_constant_ranges = [vk::PushConstantRange {
            stage_flags: vk::ShaderStageFlags::COMPUTE,
            offset: 0,
            size: 12, // 3 uint32 values: input_width, input_height, upscale_factor
        }];
        let create_info = vk::PipelineLayoutCreateInfo {
            s_type: vk::StructureType::PIPELINE_LAYOUT_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::PipelineLayoutCreateFlags::empty(),
            set_layout_count: layouts.len() as u32,
            p_set_layouts: layouts.as_ptr(),
            push_constant_range_count: push_constant_ranges.len() as u32,
            p_push_constant_ranges: push_constant_ranges.as_ptr(),
            ..Default::default()
        };
        unsafe { device.create_pipeline_layout(&create_info, None) }
    }

    fn create_compute_pipeline(
        device: &ash::Device,
        pipeline_layout: &vk::PipelineLayout,
        shader_module: &vk::ShaderModule,
    ) -> Result<vk::Pipeline, vk::Result> {
        let shader_entry_name = CString::new("main").unwrap();
        let shader_stage_info = vk::PipelineShaderStageCreateInfo {
            s_type: vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::PipelineShaderStageCreateFlags::empty(),
            stage: vk::ShaderStageFlags::COMPUTE,
            module: *shader_module,
            p_name: shader_entry_name.as_ptr(),
            p_specialization_info: std::ptr::null(),
            ..Default::default()
        };

        let create_info = vk::ComputePipelineCreateInfo {
            s_type: vk::StructureType::COMPUTE_PIPELINE_CREATE_INFO,
            p_next: std::ptr::null(),
            flags: vk::PipelineCreateFlags::empty(),
            stage: shader_stage_info,
            layout: *pipeline_layout,
            base_pipeline_handle: vk::Pipeline::null(),
            base_pipeline_index: -1,
            ..Default::default()
        };

        let pipelines = unsafe {
            device.create_compute_pipelines(vk::PipelineCache::null(), &[create_info], None)
        }
        .map_err(|(_, err)| err)?;

        Ok(pipelines[0])
    }
}

pub fn process_image(
    context: &VulkanContext,
    input_image_path: &str,
    output_image_path: &str,
    factor: u32,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Processing {} -> {}", input_image_path, output_image_path);

    // Check if input file exists
    if !std::path::Path::new(input_image_path).exists() {
        return Err(format!("Input image file does not exist: {}", input_image_path).into());
    }

    // 1. Load the image
    let input_image = image::open(input_image_path)?.to_rgba8();
    let (width, height) = input_image.dimensions();
    let input_image_data = input_image.into_raw();

    let output_width = width * factor;
    let output_height = height * factor;
    let output_image_data = vec![0u8; (output_width * output_height * 4) as usize];

    // 2. Create Buffers
    let (input_buffer, input_memory) = create_buffer(
        context,
        &input_image_data,
        vk::BufferUsageFlags::STORAGE_BUFFER,
    )?;

    let (output_buffer, output_memory) = create_buffer(
        context,
        &output_image_data,
        vk::BufferUsageFlags::STORAGE_BUFFER,
    )?;

    // 3. Update Descriptor Sets
    let pool_sizes = [vk::DescriptorPoolSize {
        ty: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 2,
    }];
    let pool_info = vk::DescriptorPoolCreateInfo {
        s_type: vk::StructureType::DESCRIPTOR_POOL_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::DescriptorPoolCreateFlags::empty(),
        max_sets: 1,
        pool_size_count: pool_sizes.len() as u32,
        p_pool_sizes: pool_sizes.as_ptr(),
        ..Default::default()
    };
    let descriptor_pool = unsafe { context.device.create_descriptor_pool(&pool_info, None)? };

    let layouts = [context.descriptor_set_layout];
    let alloc_info = vk::DescriptorSetAllocateInfo {
        s_type: vk::StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        descriptor_pool,
        descriptor_set_count: layouts.len() as u32,
        p_set_layouts: layouts.as_ptr(),
        ..Default::default()
    };
    let descriptor_sets = unsafe { context.device.allocate_descriptor_sets(&alloc_info)? };

    let input_buffer_info = vk::DescriptorBufferInfo {
        buffer: input_buffer,
        offset: 0,
        range: vk::WHOLE_SIZE,
    };
    let output_buffer_info = vk::DescriptorBufferInfo {
        buffer: output_buffer,
        offset: 0,
        range: vk::WHOLE_SIZE,
    };

    let writes = [
        vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: std::ptr::null(),
            dst_set: descriptor_sets[0],
            dst_binding: 0,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            p_image_info: std::ptr::null(),
            p_buffer_info: &input_buffer_info,
            p_texel_buffer_view: std::ptr::null(),
            ..Default::default()
        },
        vk::WriteDescriptorSet {
            s_type: vk::StructureType::WRITE_DESCRIPTOR_SET,
            p_next: std::ptr::null(),
            dst_set: descriptor_sets[0],
            dst_binding: 1,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
            p_image_info: std::ptr::null(),
            p_buffer_info: &output_buffer_info,
            p_texel_buffer_view: std::ptr::null(),
            ..Default::default()
        },
    ];
    unsafe { context.device.update_descriptor_sets(&writes, &[]) };

    // 4. Record and Submit Commands
    let pool_info = vk::CommandPoolCreateInfo {
        s_type: vk::StructureType::COMMAND_POOL_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::CommandPoolCreateFlags::empty(),
        queue_family_index: context.queue_family_index,
        ..Default::default()
    };
    let command_pool = unsafe { context.device.create_command_pool(&pool_info, None)? };

    let alloc_info = vk::CommandBufferAllocateInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        command_pool,
        level: vk::CommandBufferLevel::PRIMARY,
        command_buffer_count: 1,
        ..Default::default()
    };
    let command_buffers = unsafe { context.device.allocate_command_buffers(&alloc_info)? };
    let command_buffer = command_buffers[0];

    let begin_info = vk::CommandBufferBeginInfo {
        s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
        p_next: std::ptr::null(),
        flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        p_inheritance_info: std::ptr::null(),
        ..Default::default()
    };
    unsafe {
        context
            .device
            .begin_command_buffer(command_buffer, &begin_info)?
    };

    unsafe {
        context.device.cmd_bind_pipeline(
            command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            context.pipeline,
        );
        context.device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::COMPUTE,
            context.pipeline_layout,
            0,
            &descriptor_sets,
            &[],
        );
        let push_constants = [width, height, factor];
        context.device.cmd_push_constants(
            command_buffer,
            context.pipeline_layout,
            vk::ShaderStageFlags::COMPUTE,
            0,
            std::slice::from_raw_parts(push_constants.as_ptr() as *const u8, 12),
        );
        // Calculate dispatch groups with proper rounding
        let group_x = (output_width + 15) / 16; // Round up to nearest multiple of 16
        let group_y = (output_height + 15) / 16; // Round up to nearest multiple of 16
        println!(
            "Dispatching compute shader: {}x{} groups for {}x{} image",
            group_x, group_y, output_width, output_height
        );
        context
            .device
            .cmd_dispatch(command_buffer, group_x, group_y, 1);
    }

    unsafe { context.device.end_command_buffer(command_buffer)? };

    let submit_info = vk::SubmitInfo {
        s_type: vk::StructureType::SUBMIT_INFO,
        p_next: std::ptr::null(),
        wait_semaphore_count: 0,
        p_wait_semaphores: std::ptr::null(),
        p_wait_dst_stage_mask: std::ptr::null(),
        command_buffer_count: 1,
        p_command_buffers: &command_buffer,
        signal_semaphore_count: 0,
        p_signal_semaphores: std::ptr::null(),
        ..Default::default()
    };
    unsafe {
        context.device.queue_submit(
            context.queue,
            std::slice::from_ref(&submit_info),
            vk::Fence::null(),
        )?;
        context.device.queue_wait_idle(context.queue)?;
    }

    // 5. Read back the result
    let mut output_data = vec![0u8; output_image_data.len()];
    let ptr = unsafe {
        context.device.map_memory(
            output_memory,
            0,
            output_image_data.len() as u64,
            vk::MemoryMapFlags::empty(),
        )?
    };
    unsafe {
        std::ptr::copy_nonoverlapping(
            ptr as *const u8,
            output_data.as_mut_ptr(),
            output_image_data.len(),
        );
        context.device.unmap_memory(output_memory);
    }

    // 6. Save the image
    println!("Saving upscaled image to: {}", output_image_path);
    image::save_buffer(
        output_image_path,
        &output_data,
        output_width,
        output_height,
        image::ColorType::Rgba8,
    )
    .map_err(|e| format!("Failed to save output image: {}", e))?;

    // Verify the file was created
    if std::path::Path::new(output_image_path).exists() {
        println!(
            "✓ Successfully created upscaled image: {}",
            output_image_path
        );
    } else {
        return Err("Output file was not created despite successful processing".into());
    }

    // Cleanup
    unsafe {
        context.device.destroy_command_pool(command_pool, None);
        context
            .device
            .destroy_descriptor_pool(descriptor_pool, None);
        context.device.destroy_buffer(input_buffer, None);
        context.device.free_memory(input_memory, None);
        context.device.destroy_buffer(output_buffer, None);
        context.device.free_memory(output_memory, None);
    }

    Ok(())
}

fn create_buffer(
    context: &VulkanContext,
    data: &[u8],
    usage: vk::BufferUsageFlags,
) -> Result<(vk::Buffer, vk::DeviceMemory), Box<dyn std::error::Error>> {
    let buffer_info = vk::BufferCreateInfo {
        s_type: vk::StructureType::BUFFER_CREATE_INFO,
        p_next: std::ptr::null(),
        flags: vk::BufferCreateFlags::empty(),
        size: data.len() as u64,
        usage,
        sharing_mode: vk::SharingMode::EXCLUSIVE,
        queue_family_index_count: 0,
        p_queue_family_indices: std::ptr::null(),
        ..Default::default()
    };

    let buffer = unsafe { context.device.create_buffer(&buffer_info, None)? };

    let mem_requirements = unsafe { context.device.get_buffer_memory_requirements(buffer) };
    let mem_properties = unsafe {
        context
            .instance
            .get_physical_device_memory_properties(context.physical_device)
    };

    let memory_type_index = mem_properties
        .memory_types
        .iter()
        .enumerate()
        .find(|(i, mem_type)| {
            (mem_requirements.memory_type_bits & (1 << i)) != 0
                && mem_type.property_flags.contains(
                    vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT,
                )
        })
        .map(|(i, _)| i as u32)
        .ok_or("Failed to find suitable memory type")?;

    let alloc_info = vk::MemoryAllocateInfo {
        s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
        p_next: std::ptr::null(),
        allocation_size: mem_requirements.size,
        memory_type_index,
        ..Default::default()
    };

    let memory = unsafe { context.device.allocate_memory(&alloc_info, None)? };
    unsafe { context.device.bind_buffer_memory(buffer, memory, 0)? };

    let ptr = unsafe {
        context
            .device
            .map_memory(memory, 0, data.len() as u64, vk::MemoryMapFlags::empty())?
    };
    unsafe {
        std::ptr::copy_nonoverlapping(data.as_ptr(), ptr as *mut u8, data.len());
        context.device.unmap_memory(memory);
    }

    Ok((buffer, memory))
}

impl Drop for VulkanContext {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_pipeline(self.pipeline, None);
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
            self.device
                .destroy_descriptor_set_layout(self.descriptor_set_layout, None);
            self.device.destroy_shader_module(self.shader_module, None);
            if self.device.handle() != vk::Device::null() {
                self.device.destroy_device(None);
            }
            self.instance.destroy_instance(None);
        }
    }
}
