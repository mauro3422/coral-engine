// Mesh geometry definitions - Voxel-first architecture
use bytemuck::{Pod, Zeroable};
use wgpu;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct ColoredVertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

// Instance data for GPU instancing
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct VoxelInstance {
    pub position: [f32; 3],
    pub block_type: u32,
}

#[derive(Clone)]
pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
}

impl Mesh {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Self {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Vertex Buffer"),
            size: (std::mem::size_of::<Vertex>() * vertices.len()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(vertices));

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Index Buffer"),
            size: (std::mem::size_of::<u32>() * indices.len()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(indices));

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }

    pub fn new_colored(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[ColoredVertex],
        indices: &[u32],
    ) -> Self {
        let vertex_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Colored Vertex Buffer"),
            size: (std::mem::size_of::<ColoredVertex>() * vertices.len()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&vertex_buffer, 0, bytemuck::cast_slice(vertices));

        let index_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Colored Index Buffer"),
            size: (std::mem::size_of::<u32>() * indices.len()) as u64,
            usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&index_buffer, 0, bytemuck::cast_slice(indices));

        Self {
            vertex_buffer,
            index_buffer,
            index_count: indices.len() as u32,
        }
    }

    /// Create a mesh with instance buffer for GPU instancing
    pub fn new_instanced(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        vertices: &[Vertex],
        indices: &[u32],
        instances: &[VoxelInstance],
    ) -> (Self, wgpu::Buffer, u32) {
        let mesh = Self::new(device, queue, vertices, indices);

        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Instance Buffer"),
            size: (std::mem::size_of::<VoxelInstance>() * instances.len()) as u64,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(&instance_buffer, 0, bytemuck::cast_slice(instances));

        (mesh, instance_buffer, instances.len() as u32)
    }

    /// Update instance buffer with new data
    pub fn update_instances(&self, queue: &wgpu::Queue, instance_buffer: &wgpu::Buffer, instances: &[VoxelInstance]) {
        queue.write_buffer(instance_buffer, 0, bytemuck::cast_slice(instances));
    }

    /// Single cube mesh for GPU instancing
    /// Size is configurable to support different voxel scales
    pub fn cube_voxel(size: f32) -> (Vec<Vertex>, Vec<u32>) {
        let s = size;
        let vertices = vec![
            // Front (+Z)
            Vertex { position: [0.0, 0.0, s], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [s, 0.0, s], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [s, s, s], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [0.0, s, s], normal: [0.0, 0.0, 1.0] },
            // Back (-Z)
            Vertex { position: [s, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [0.0, 0.0, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [0.0, s, 0.0], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [s, s, 0.0], normal: [0.0, 0.0, -1.0] },
            // Top (+Y)
            Vertex { position: [0.0, s, 0.0], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [0.0, s, s], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [s, s, s], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [s, s, 0.0], normal: [0.0, 1.0, 0.0] },
            // Bottom (-Y)
            Vertex { position: [0.0, 0.0, s], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [0.0, 0.0, 0.0], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [s, 0.0, 0.0], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [s, 0.0, s], normal: [0.0, -1.0, 0.0] },
            // Right (+X)
            Vertex { position: [s, 0.0, s], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [s, 0.0, 0.0], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [s, s, 0.0], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [s, s, s], normal: [1.0, 0.0, 0.0] },
            // Left (-X)
            Vertex { position: [0.0, 0.0, 0.0], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [0.0, 0.0, s], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [0.0, s, s], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [0.0, s, 0.0], normal: [-1.0, 0.0, 0.0] },
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3,       // Front
            4, 5, 6, 4, 6, 7,       // Back
            8, 9, 10, 8, 10, 11,    // Top
            12, 13, 14, 12, 14, 15, // Bottom
            16, 17, 18, 16, 18, 19, // Right
            20, 21, 22, 20, 22, 23, // Left
        ];
        (vertices, indices)
    }

    /// Wireframe box with given size, centered at origin
    pub fn wireframe_box(size: f32) -> (Vec<ColoredVertex>, Vec<u32>) {
        Self::wireframe_box_custom(size, size, size)
    }

    /// Wireframe box with custom dimensions (width, height, depth), centered at origin
    pub fn wireframe_box_custom(w: f32, h: f32, d: f32) -> (Vec<ColoredVertex>, Vec<u32>) {
        let hw = w / 2.0; // half-width
        let hh = h / 2.0; // half-height
        let hd = d / 2.0; // half-depth
        let color = [1.0, 0.5, 0.0]; // Bright orange wireframe
        let vertices = vec![
            // Bottom face edges
            ColoredVertex { position: [-hw, -hh, -hd], color },
            ColoredVertex { position: [ hw, -hh, -hd], color },
            ColoredVertex { position: [ hw, -hh, -hd], color },
            ColoredVertex { position: [ hw, -hh,  hd], color },
            ColoredVertex { position: [ hw, -hh,  hd], color },
            ColoredVertex { position: [-hw, -hh,  hd], color },
            ColoredVertex { position: [-hw, -hh,  hd], color },
            ColoredVertex { position: [-hw, -hh, -hd], color },
            // Top face edges
            ColoredVertex { position: [-hw,  hh, -hd], color },
            ColoredVertex { position: [ hw,  hh, -hd], color },
            ColoredVertex { position: [ hw,  hh, -hd], color },
            ColoredVertex { position: [ hw,  hh,  hd], color },
            ColoredVertex { position: [ hw,  hh,  hd], color },
            ColoredVertex { position: [-hw,  hh,  hd], color },
            ColoredVertex { position: [-hw,  hh,  hd], color },
            ColoredVertex { position: [-hw,  hh, -hd], color },
            // Vertical edges
            ColoredVertex { position: [-hw, -hh, -hd], color },
            ColoredVertex { position: [-hw,  hh, -hd], color },
            ColoredVertex { position: [ hw, -hh, -hd], color },
            ColoredVertex { position: [ hw,  hh, -hd], color },
            ColoredVertex { position: [ hw, -hh,  hd], color },
            ColoredVertex { position: [ hw,  hh,  hd], color },
            ColoredVertex { position: [-hw, -hh,  hd], color },
            ColoredVertex { position: [-hw,  hh,  hd], color },
        ];
        let indices: Vec<u32> = (0..vertices.len() as u32).collect();
        (vertices, indices)
    }

    /// Internal grid lines showing voxel resolution within a block
    /// Creates lines at each voxel boundary
    pub fn voxel_grid(block_size: f32, divisions: u32) -> (Vec<ColoredVertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let step = block_size / divisions as f32;
        let half = block_size / 2.0;
        let color = [0.4, 0.6, 1.0]; // Blue-ish grid lines

        // Lines along each axis at voxel boundaries
        for i in 0..=divisions {
            let pos = -half + i as f32 * step;

            // X-axis lines (along YZ plane)
            for j in 0..=divisions {
                let y = -half + j as f32 * step;
                // Z direction
                vertices.push(ColoredVertex { position: [pos, y, -half], color });
                vertices.push(ColoredVertex { position: [pos, y,  half], color });
                // Y direction
                vertices.push(ColoredVertex { position: [pos, -half, j as f32 * step - half], color });
                vertices.push(ColoredVertex { position: [pos,  half, j as f32 * step - half], color });
            }

            // Y-axis lines (along XZ plane)
            for j in 0..=divisions {
                let x = -half + j as f32 * step;
                // Z direction
                vertices.push(ColoredVertex { position: [x, pos, -half], color });
                vertices.push(ColoredVertex { position: [x, pos,  half], color });
                // X direction
                vertices.push(ColoredVertex { position: [-half, pos, j as f32 * step - half], color });
                vertices.push(ColoredVertex { position: [ half, pos, j as f32 * step - half], color });
            }

            // Z-axis lines (along XY plane)
            for j in 0..=divisions {
                let x = -half + j as f32 * step;
                // Y direction
                vertices.push(ColoredVertex { position: [x, -half, pos], color });
                vertices.push(ColoredVertex { position: [x,  half, pos], color });
                // X direction
                vertices.push(ColoredVertex { position: [-half, j as f32 * step - half, pos], color });
                vertices.push(ColoredVertex { position: [ half, j as f32 * step - half, pos], color });
            }
        }

        let indices: Vec<u32> = (0..vertices.len() as u32).collect();
        (vertices, indices)
    }

    pub fn quad(size: f32) -> (Vec<Vertex>, Vec<u32>) {
        let h = size / 2.0;
        let vertices = vec![
            Vertex { position: [-h, 0.0, h], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [h, 0.0, h], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [h, 0.0, -h], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [-h, 0.0, -h], normal: [0.0, 1.0, 0.0] },
        ];
        (vertices, vec![0, 1, 2, 0, 2, 3])
    }

    pub fn cube(size: f32) -> (Vec<Vertex>, Vec<u32>) {
        let h = size / 2.0;
        let vertices = vec![
            Vertex { position: [-h, -h, h], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [h, -h, h], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [h, h, h], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [-h, h, h], normal: [0.0, 0.0, 1.0] },
            Vertex { position: [h, -h, -h], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [-h, -h, -h], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [-h, h, -h], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [h, h, -h], normal: [0.0, 0.0, -1.0] },
            Vertex { position: [-h, h, h], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [h, h, h], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [h, h, -h], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [-h, h, -h], normal: [0.0, 1.0, 0.0] },
            Vertex { position: [-h, -h, -h], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [h, -h, -h], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [h, -h, h], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [-h, -h, h], normal: [0.0, -1.0, 0.0] },
            Vertex { position: [h, -h, h], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [h, -h, -h], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [h, h, -h], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [h, h, h], normal: [1.0, 0.0, 0.0] },
            Vertex { position: [-h, -h, -h], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [-h, -h, h], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [-h, h, h], normal: [-1.0, 0.0, 0.0] },
            Vertex { position: [-h, h, -h], normal: [-1.0, 0.0, 0.0] },
        ];
        let indices = vec![
            0, 1, 2, 0, 2, 3, 4, 5, 6, 4, 6, 7, 8, 9, 10, 8, 10, 11, 12, 13, 14, 12, 14, 15, 16,
            17, 18, 16, 18, 19, 20, 21, 22, 20, 22, 23,
        ];
        (vertices, indices)
    }

    fn line_indices(vertex_count: usize) -> Vec<u32> {
        let mut indices = Vec::with_capacity(vertex_count);
        for i in (0..vertex_count).step_by(2) {
            indices.push(i as u32);
            indices.push((i + 1) as u32);
        }
        indices
    }

    pub fn grid(divisions: u32, size: f32) -> (Vec<ColoredVertex>, Vec<u32>) {
        let mut vertices = Vec::new();
        let step = size / divisions as f32;
        let half = size / 2.0;

        for i in 0..=divisions {
            let pos = -half + i as f32 * step;
            let is_center = i == divisions / 2;
            let color = if is_center { [0.5, 0.5, 0.5] } else { [0.3, 0.3, 0.3] };

            vertices.push(ColoredVertex { position: [-half, 0.0, pos], color });
            vertices.push(ColoredVertex { position: [half, 0.0, pos], color });
            vertices.push(ColoredVertex { position: [pos, 0.0, -half], color });
            vertices.push(ColoredVertex { position: [pos, 0.0, half], color });
        }

        let indices = Self::line_indices(vertices.len());
        (vertices, indices)
    }

    pub fn axes(axis_len: f32) -> (Vec<ColoredVertex>, Vec<u32>) {
        let vertices = vec![
            // X axis - red (longer)
            ColoredVertex { position: [-axis_len, 0.0, 0.0], color: [1.0, 0.0, 0.0] },
            ColoredVertex { position: [axis_len, 0.0, 0.0], color: [1.0, 0.0, 0.0] },
            // Y axis - green (longer)
            ColoredVertex { position: [0.0, 0.0, 0.0], color: [0.0, 1.0, 0.0] },
            ColoredVertex { position: [0.0, axis_len, 0.0], color: [0.0, 1.0, 0.0] },
            // Z axis - blue (longer)
            ColoredVertex { position: [0.0, 0.0, -axis_len], color: [0.0, 0.0, 1.0] },
            ColoredVertex { position: [0.0, 0.0, axis_len], color: [0.0, 0.0, 1.0] },
        ];
        let indices = Self::line_indices(vertices.len());
        (vertices, indices)
    }
}
