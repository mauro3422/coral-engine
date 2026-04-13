// RenderState - Voxel-first architecture
// Only manages voxel rendering with GPU instancing + grid/axes debug visualization
use crate::ocean::render::{WaterFace, block_types};
use super::mesh::{Mesh, VoxelInstance};
use super::pipeline::{CameraUniforms, VoxelPipeline, GridPipeline};
use crate::core::scene::{DebugMesh, DebugObject};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use wgpu::SurfaceError;

pub struct VoxelMesh {
    mesh: Mesh,
    instance_buffer: wgpu::Buffer,
    pub instance_count: u32,
}

pub struct RenderState {
    pub surface: wgpu::Surface<'static>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>,
    
    // Pipelines
    pub voxel_pipeline: VoxelPipeline,
    pub grid_pipeline: GridPipeline,
    pub voxel_bind_group: wgpu::BindGroup,
    pub grid_bind_group: wgpu::BindGroup,
    
    // Meshes
    pub voxel_cube_mesh: Mesh, // Shared cube mesh (size determined at render time via shader)
    pub grid_mesh: Mesh,
    pub axes_mesh: Mesh,

    // Block visualization meshes (debug wireframe + voxel grid)
    pub block_wireframe: Option<Mesh>,
    pub block_voxel_grid: Option<Mesh>,
    
    // Voxel mesh (dynamic)
    pub voxel_mesh: Option<VoxelMesh>,
    
    // Depth
    pub depth_texture: wgpu::TextureView,
    
    // Egui
    pub egui_renderer: egui_wgpu::Renderer,
}

impl RenderState {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12 | wgpu::Backends::VULKAN | wgpu::Backends::GL,
            ..Default::default()
        });

        let window_handle = window.window_handle().expect("window handle");
        let display_handle = window.display_handle().expect("display handle");
        let surface = unsafe {
            instance.create_surface_unsafe(wgpu::SurfaceTargetUnsafe::RawHandle {
                raw_display_handle: display_handle.as_raw(),
                raw_window_handle: window_handle.as_raw(),
            })
        }
        .expect("surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("adapter");

        println!(
            "[Render] Adapter: {} ({:?})",
            adapter.get_info().name,
            adapter.get_info().backend
        );

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::downlevel_webgl2_defaults()
                        .using_resolution(adapter.limits()),
                    ..Default::default()
                },
                None,
            )
            .await
            .expect("device");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let depth_texture = Self::create_depth_texture(&device, &config);
        let depth_texture_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // Create pipelines
        let voxel_pipeline = VoxelPipeline::new(&device, surface_format);
        let grid_pipeline = GridPipeline::new(&device, surface_format);
        
        // Create bind groups
        let voxel_bind_group = voxel_pipeline.create_bind_group(&device);
        let grid_bind_group = grid_pipeline.create_bind_group(&device);

        // Create shared cube mesh for voxel instancing (size 1.0, scaled by shader via voxel_size uniform)
        let (cube_verts, cube_indices) = Mesh::cube_voxel(1.0);
        let voxel_cube_mesh = Mesh::new(&device, &queue, &cube_verts, &cube_indices);

        // Create debug meshes
        let (grid_v, grid_i) = Mesh::grid(20, 100.0);
        let grid_mesh = Mesh::new_colored(&device, &queue, &grid_v, &grid_i);
        let (axes_v, axes_i) = Mesh::axes(50.0);
        let axes_mesh = Mesh::new_colored(&device, &queue, &axes_v, &axes_i);

        let egui_renderer = egui_wgpu::Renderer::new(&device, surface_format, None, 1, false);

        println!("[Render] Voxel pipeline + GPU instancing ready");

        Self {
            surface,
            device,
            queue,
            config,
            size,
            voxel_pipeline,
            grid_pipeline,
            voxel_bind_group,
            grid_bind_group,
            voxel_cube_mesh,
            grid_mesh,
            axes_mesh,
            voxel_mesh: None,
            depth_texture: depth_texture_view,
            egui_renderer,
            block_wireframe: None,
            block_voxel_grid: None,
        }
    }

    fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::Texture {
        device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth Texture"),
            size: wgpu::Extent3d {
                width: config.width,
                height: config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        })
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            let depth_texture = Self::create_depth_texture(&self.device, &self.config);
            self.depth_texture = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        }
    }

    /// Rebuild block visualization to match object bounds
    /// Uses OceanDimensions which derives from actual config - ALWAYS in sync
    pub fn rebuild_block_visualization(&mut self, dims: crate::ocean::OceanDimensions) {
        let bounds = dims.bounds();
        let size = bounds.size();
        let (wire_v, wire_i) = Mesh::wireframe_box_at(
            bounds.min[0], bounds.min[1], bounds.min[2],
            size[0], size[1], size[2],
        );
        self.block_wireframe = Some(Mesh::new_colored(&self.device, &self.queue, &wire_v, &wire_i));
        self.block_voxel_grid = None;
    }

    /// Build voxel mesh from visible faces using GPU instancing
    /// Each face is an instance of the cube mesh
    pub fn rebuild_voxel_mesh(&mut self, faces: &[([f32; 3], [f32; 3], u8)]) {
        if faces.is_empty() {
            self.voxel_mesh = None;
            println!("[Render] Voxel mesh cleared (no faces)");
            return;
        }

        // Convert faces to instance data
        // Each face becomes an instance at that position
        let instances: Vec<VoxelInstance> = faces
            .iter()
            .map(|(pos, _normal, block_type)| VoxelInstance {
                position: *pos,
                block_type: *block_type as u32,
            })
            .collect();

        self.rebuild_voxel_mesh_from_instances(&instances);
    }

    /// Build voxel mesh from pre-built instances
    pub fn rebuild_voxel_mesh_from_instances(&mut self, instances: &[VoxelInstance]) {
        if instances.is_empty() {
            self.voxel_mesh = None;
            println!("[Render] Voxel mesh cleared (no instances)");
            return;
        }

        let instance_count = instances.len() as u32;

        let instance_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Voxel Instance Buffer"),
            size: (std::mem::size_of::<VoxelInstance>() * instances.len()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(instances));

        self.voxel_mesh = Some(VoxelMesh {
            mesh: self.voxel_cube_mesh.clone(), // Reuse cube mesh
            instance_buffer,
            instance_count,
        });

        println!("[Render] Voxel mesh rebuilt: {} instances (GPU instancing)", instance_count);
    }

    pub fn render(
        &mut self,
        debug_objects: &[DebugObject],
        view_proj: cgmath::Matrix4<f32>,
        egui_clipped_meshes: &[egui::ClippedPrimitive],
        egui_textures_delta: &egui::TexturesDelta,
        screen_desc: &egui_wgpu::ScreenDescriptor,
        water_faces: &[WaterFace],
        rebuild_needed: bool,
        show_water: bool,
        _show_terrain: bool,
        voxel_size: f32,
    ) -> Result<(), SurfaceError> {
        // Build voxel instances from water faces
        let instances: Vec<VoxelInstance> = water_faces
            .iter()
            .map(|face| VoxelInstance {
                position: face.position,
                block_type: block_types::WATER as u32,
            })
            .collect();

        let water_count = instances.len();
        println!("[Render] Water faces: {}", water_count);

        // Rebuild voxel mesh only when needed
        if rebuild_needed && !instances.is_empty() && show_water {
            self.rebuild_voxel_mesh_from_instances(&instances);
        } else if !show_water {
            self.voxel_mesh = None;
        }

        // Update camera buffer for voxel pipeline
        let cam_u = CameraUniforms {
            view_proj: view_proj.into(),
        };
        self.queue
            .write_buffer(&self.voxel_pipeline.camera_buffer, 0, bytemuck::bytes_of(&cam_u));

        // Update voxel params (voxel_size) for dynamic cube scaling
        let voxel_params = crate::render::pipeline::VoxelUniforms {
            voxel_size: voxel_size,
            _padding: [0.0; 3],
        };
        self.queue
            .write_buffer(&self.voxel_pipeline.voxel_params_buffer, 0, bytemuck::bytes_of(&voxel_params));

        // Update camera buffer for grid pipeline
        self.queue
            .write_buffer(&self.grid_pipeline.camera_buffer, 0, bytemuck::bytes_of(&cam_u));

        for (id, image_delta) in &egui_textures_delta.set {
            self.egui_renderer
                .update_texture(&self.device, &self.queue, *id, image_delta);
        }

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render"),
            });
        self.egui_renderer.update_buffers(
            &mut self.device,
            &mut self.queue,
            &mut encoder,
            egui_clipped_meshes,
            screen_desc,
        );

        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // 3D pass
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3D Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.4,
                            g: 0.6,
                            b: 0.9,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Draw voxels with GPU instancing
            if let Some(ref voxel_mesh) = self.voxel_mesh {
                rpass.set_pipeline(&self.voxel_pipeline.pipeline);
                rpass.set_bind_group(0, &self.voxel_bind_group, &[]);
                rpass.set_vertex_buffer(0, voxel_mesh.mesh.vertex_buffer.slice(..));
                rpass.set_index_buffer(voxel_mesh.mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                rpass.set_vertex_buffer(1, voxel_mesh.instance_buffer.slice(..));
                rpass.draw_indexed(0..voxel_mesh.mesh.index_count, 0, 0..voxel_mesh.instance_count);
            }

            // Draw block wireframe (centered at origin)
            if let Some(ref wireframe) = self.block_wireframe {
                rpass.set_pipeline(&self.grid_pipeline.pipeline);
                rpass.set_bind_group(0, &self.grid_bind_group, &[]);
                rpass.set_vertex_buffer(0, wireframe.vertex_buffer.slice(..));
                rpass.set_index_buffer(wireframe.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                rpass.draw_indexed(0..wireframe.index_count, 0, 0..1);
            }

            // Draw debug objects (grid, axes)
            for debug_obj in debug_objects {
                let (pipeline, bind_group, mesh) = match debug_obj.mesh {
                    DebugMesh::Grid => (
                        &self.grid_pipeline.pipeline,
                        &self.grid_bind_group,
                        &self.grid_mesh,
                    ),
                    DebugMesh::Axes => (
                        &self.grid_pipeline.pipeline,
                        &self.grid_bind_group,
                        &self.axes_mesh,
                    ),
                };

                rpass.set_pipeline(pipeline);
                rpass.set_bind_group(0, bind_group, &[]);
                rpass.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                rpass.set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint32);
                rpass.draw_indexed(0..mesh.index_count, 0, 0..1);
            }
        }

        // Egui pass
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Egui Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                timestamp_writes: None,
                occlusion_query_set: None,
                depth_stencil_attachment: None,
            });
            let rpass_static: &mut wgpu::RenderPass<'static> =
                unsafe { std::mem::transmute(&mut rpass) };
            self.egui_renderer
                .render(rpass_static, egui_clipped_meshes, screen_desc);
        }

        for id in &egui_textures_delta.free {
            self.egui_renderer.free_texture(id);
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        Ok(())
    }
}
