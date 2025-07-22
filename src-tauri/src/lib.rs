// src-tauri/src/lib.rs

pub mod vulkan;

use image::GenericImageView;
use std::env;
use std::path::Path;
use tauri::{generate_context, Builder};
use tauri_plugin_dialog::init as dialog_init;
use tauri_plugin_fs::init as fs_init;
use vulkan::VulkanContext;

#[cfg(target_os = "macos")]
fn setup_moltenvk_for_command() -> Result<(), Box<dyn std::error::Error>> {
    use std::fs;
    use std::io::Write;

    // Get the executable path and find the Resources directory
    let exe_path = env::current_exe()?;
    let app_dir = exe_path.parent().ok_or("Could not find app directory")?;

    println!("🔍 Debug: Executable path: {:?}", exe_path);
    println!("🔍 Debug: App directory: {:?}", app_dir);

    // More comprehensive search for MoltenVK files
    let possible_resource_paths = [
        // Tauri development mode
        app_dir.join("../../../src-tauri/moltenvk"),
        app_dir.join("../../src-tauri/moltenvk"),
        app_dir.join("../src-tauri/moltenvk"),
        app_dir.join("src-tauri/moltenvk"),
        app_dir.join("moltenvk"),
        // Tauri bundle mode
        app_dir.join("../Resources"),
        app_dir.join("Resources"),
        app_dir.parent().unwrap_or(app_dir).join("Resources"),
        app_dir
            .parent()
            .unwrap_or(app_dir)
            .parent()
            .unwrap_or(app_dir)
            .join("Resources"),
        // Current working directory (development mode)
        env::current_dir()
            .unwrap_or_else(|_| app_dir.to_path_buf())
            .join("src-tauri/moltenvk"),
        env::current_dir()
            .unwrap_or_else(|_| app_dir.to_path_buf())
            .join("moltenvk"),
        // Absolute fallback for the current project
        std::path::PathBuf::from("/Users/augustusotu/Projects/v-upscale/src-tauri/moltenvk"),
    ];

    println!(
        "🔍 Searching for MoltenVK in {} locations...",
        possible_resource_paths.len()
    );

    let mut moltenvk_dir = None;
    for (i, resource_path) in possible_resource_paths.iter().enumerate() {
        let moltenvk_dylib = resource_path.join("libMoltenVK.dylib");
        println!(
            "🔍 [{}/{}] Checking: {:?}",
            i + 1,
            possible_resource_paths.len(),
            moltenvk_dylib
        );

        if moltenvk_dylib.exists() {
            moltenvk_dir = Some(resource_path.clone());
            println!("✅ Found MoltenVK at: {:?}", resource_path);
            break;
        }
    }

    let moltenvk_dir = moltenvk_dir.ok_or_else(|| {
        println!("❌ Could not find libMoltenVK.dylib in any of the searched locations");
        "Could not find MoltenVK resources"
    })?;

    let moltenvk_dylib = moltenvk_dir.join("libMoltenVK.dylib");
    let temp_icd_path = env::temp_dir().join("MoltenVK_icd.json");

    // Verify the library can be accessed
    if !moltenvk_dylib.exists() {
        return Err("MoltenVK library file not accessible".into());
    }

    // Get the absolute canonical path to avoid relative path issues
    let moltenvk_dylib_absolute = moltenvk_dylib
        .canonicalize()
        .map_err(|e| format!("Failed to get absolute path for MoltenVK: {}", e))?;

    println!("🔧 Using absolute path: {:?}", moltenvk_dylib_absolute);

    // Check library file properties
    let metadata = fs::metadata(&moltenvk_dylib_absolute)?;
    println!("📊 Library file size: {} bytes", metadata.len());
    println!("📊 Library file permissions: {:?}", metadata.permissions());

    // Create a proper ICD file with absolute paths
    let icd_content = format!(
        r#"{{
    "file_format_version": "1.0.0",
    "ICD": {{
        "library_path": "{}",
        "api_version": "1.3.0",
        "is_portability_driver": true
    }}
}}"#,
        moltenvk_dylib_absolute.to_string_lossy().replace('\\', "/")
    );

    let mut icd_file = fs::File::create(&temp_icd_path)?;
    icd_file.write_all(icd_content.as_bytes())?;
    icd_file.sync_all()?;

    // Verify the ICD file was created correctly
    let created_content = fs::read_to_string(&temp_icd_path)?;
    println!("📄 Generated ICD file content:\n{}", created_content);

    // Set environment variables
    env::set_var("VK_ICD_FILENAMES", &temp_icd_path);
    env::set_var("VK_DRIVER_FILES", &temp_icd_path);
    // Enable Vulkan loader debug output
    env::set_var("VK_LOADER_DEBUG", "all");

    println!("🔧 MoltenVK configured:");
    println!("   Library: {:?}", moltenvk_dylib_absolute);
    println!("   ICD: {:?}", temp_icd_path);
    println!("   VK_ICD_FILENAMES = {:?}", env::var("VK_ICD_FILENAMES"));

    Ok(())
}

#[tauri::command]
fn upscale_image_enhanced(
    path: String,
    factor: u32,
    apply_sharpening: Option<bool>,
    apply_contrast_enhancement: Option<bool>,
    apply_noise_reduction: Option<bool>,
) -> Result<String, String> {
    println!(
        "🚀 Starting ENHANCED upscale process for: {} with factor: {}",
        path, factor
    );

    // Default values for optional parameters - optimized for sharpness
    let sharpening = apply_sharpening.unwrap_or(true);
    let contrast = apply_contrast_enhancement.unwrap_or(true);
    let noise_reduction = apply_noise_reduction.unwrap_or(false); // Keep false by default to avoid blur

    println!("🎛️  Post-processing settings:");
    println!("   - Sharpening: {}", sharpening);
    println!("   - Contrast Enhancement: {}", contrast);
    println!("   - Noise Reduction: {}", noise_reduction);

    // Validate inputs
    if !Path::new(&path).exists() {
        let error_msg = format!("❌ Input file does not exist: {}", path);
        println!("{}", error_msg);
        return Err(error_msg);
    }

    if factor == 0 || factor > 8 {
        let error_msg = format!(
            "❌ Invalid upscale factor: {}. Must be between 1 and 8.",
            factor
        );
        println!("{}", error_msg);
        return Err(error_msg);
    }

    // Ensure MoltenVK is properly configured for macOS
    #[cfg(target_os = "macos")]
    {
        setup_moltenvk_for_command().map_err(|e| {
            let error_msg = format!("❌ MoltenVK setup failed: {}", e);
            println!("{}", error_msg);
            error_msg
        })?;
    }

    println!("📦 Initializing Vulkan context...");
    let vulkan_context = VulkanContext::new().map_err(|e| {
        let error_msg = format!("❌ Vulkan initialization failed: {}", e);
        println!("{}", error_msg);
        println!("💡 Debug info:");
        println!("   VK_ICD_FILENAMES = {:?}", env::var("VK_ICD_FILENAMES"));
        println!("   VK_DRIVER_FILES = {:?}", env::var("VK_DRIVER_FILES"));
        error_msg
    })?;
    println!("✅ Vulkan context initialized successfully");

    let output_path = {
        let temp_dir = env::temp_dir();
        let file_name = format!(
            "upscaled_enhanced_{}x_{}.png",
            factor,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        temp_dir.join(file_name).to_str().unwrap().to_string()
    };

    println!("🎯 Output path: {}", output_path);

    vulkan::process_image_enhanced(
        &vulkan_context,
        &path,
        &output_path,
        factor,
        sharpening,
        contrast,
        noise_reduction,
    )
    .map_err(|e| {
        let error_msg = format!("❌ Enhanced image processing failed: {}", e);
        println!("{}", error_msg);
        error_msg
    })?;

    println!("🎉 Enhanced upscaling completed successfully!");
    Ok(output_path)
}

#[tauri::command]
fn upscale_image_nearest_neighbor(path: String, factor: u32) -> Result<String, String> {
    println!(
        "🚀 Starting NEAREST NEIGHBOR upscale (pixel-perfect) for: {} with factor: {}",
        path, factor
    );

    // Validate inputs
    if !Path::new(&path).exists() {
        let error_msg = format!("❌ Input file does not exist: {}", path);
        println!("{}", error_msg);
        return Err(error_msg);
    }

    if factor == 0 || factor > 8 {
        let error_msg = format!(
            "❌ Invalid upscale factor: {}. Must be between 1 and 8.",
            factor
        );
        println!("{}", error_msg);
        return Err(error_msg);
    }

    // For nearest neighbor, we can use simple image library upscaling
    let input_image = image::open(&path).map_err(|e| {
        let error_msg = format!("❌ Failed to open input image: {}", e);
        println!("{}", error_msg);
        error_msg
    })?;

    let (width, height) = input_image.dimensions();
    let output_width = width * factor;
    let output_height = height * factor;

    // Use image library's nearest neighbor resize
    let resized = input_image.resize_exact(
        output_width,
        output_height,
        image::imageops::FilterType::Nearest,
    );

    let output_path = {
        let temp_dir = env::temp_dir();
        let file_name = format!(
            "upscaled_nearest_{}x_{}.png",
            factor,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        temp_dir.join(file_name).to_str().unwrap().to_string()
    };

    resized.save(&output_path).map_err(|e| {
        let error_msg = format!("❌ Failed to save output image: {}", e);
        println!("{}", error_msg);
        error_msg
    })?;

    println!("🎉 Nearest neighbor upscaling completed: {}", output_path);
    Ok(output_path)
}

#[tauri::command]
fn upscale_image(path: String, factor: u32) -> Result<String, String> {
    println!(
        "🚀 Starting upscale process for: {} with factor: {}",
        path, factor
    );

    // Validate inputs
    if !Path::new(&path).exists() {
        let error_msg = format!("❌ Input file does not exist: {}", path);
        println!("{}", error_msg);
        return Err(error_msg);
    }

    if factor == 0 || factor > 8 {
        let error_msg = format!(
            "❌ Invalid upscale factor: {}. Must be between 1 and 8.",
            factor
        );
        println!("{}", error_msg);
        return Err(error_msg);
    }

    // Ensure MoltenVK is properly configured for macOS
    #[cfg(target_os = "macos")]
    {
        setup_moltenvk_for_command().map_err(|e| {
            let error_msg = format!("❌ MoltenVK setup failed: {}", e);
            println!("{}", error_msg);
            error_msg
        })?;
    }

    println!("📦 Initializing Vulkan context...");
    let vulkan_context = VulkanContext::new().map_err(|e| {
        let error_msg = format!("❌ Vulkan initialization failed: {}", e);
        println!("{}", error_msg);
        println!("💡 Debug info:");
        println!("   VK_ICD_FILENAMES = {:?}", env::var("VK_ICD_FILENAMES"));
        println!("   VK_DRIVER_FILES = {:?}", env::var("VK_DRIVER_FILES"));
        error_msg
    })?;
    println!("✅ Vulkan context initialized successfully");

    let output_path = {
        let temp_dir = env::temp_dir();
        let file_name = format!(
            "upscaled_{}x_{}.png",
            factor,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
        temp_dir.join(file_name).to_str().unwrap().to_string()
    };

    println!("🎯 Output path: {}", output_path);

    vulkan::process_image(&vulkan_context, &path, &output_path, factor).map_err(|e| {
        let error_msg = format!("❌ Image processing failed: {}", e);
        println!("{}", error_msg);
        error_msg
    })?;

    println!("🎉 Upscaling completed successfully!");
    Ok(output_path)
}

pub fn run() {
    Builder::default()
        .setup(|_app| {
            #[cfg(target_os = "macos")]
            {
                println!("🍎 Running on macOS - MoltenVK will be configured when needed");
                // Basic setup - detailed MoltenVK configuration happens in upscale_image command
            }
            Ok(())
        })
        .plugin(fs_init())
        .plugin(dialog_init())
        .invoke_handler(tauri::generate_handler![
            upscale_image,
            upscale_image_enhanced,
            upscale_image_nearest_neighbor
        ])
        .run(generate_context!())
        .expect("error while running tauri application");
}
