use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use wgpu::util::DeviceExt;


// lib.rs
use winit::window::Window;

// lib.rs
#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}

// lib.rs
impl Vertex {
    fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                }
            ]
        }
    }
}


const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0], color: [0.5, 0.0, 0.5] }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];


const VERTICES2: &[Vertex] = &[
    Vertex { position: [0.0, 0.0, 0.0], color: [0.0, 1.0, 0.5] },
    Vertex { position: [0.5, 0.0, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4996954135095479, 0.017449748351250485, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4987820251299121, 0.03487823687206265, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.49726094768413664, 0.05226423163382673, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4951340343707852, 0.06958655048003272, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.492403876506104, 0.08682408883346517, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.48907380036690284, 0.10395584540887966, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.48514786313799824, 0.12096094779983387, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.48063084796915945, 0.13781867790849958, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.47552825814757677, 0.1545084971874737, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4698463103929542, 0.17101007166283436, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4635919272833937, 0.187303296707956, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.45677272882130043, 0.20336832153790008, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4493970231495835, 0.2191855733945387, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4414737964294635, 0.2347357813929454, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.43301270189221935, 0.24999999999999997, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.424024048078213, 0.26495963211660245, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.41451878627752087, 0.2795964517353734, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4045084971874737, 0.29389262614623657, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.39400537680336095, 0.30783073766282915, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.383022221559489, 0.3213938048432696, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3715724127386971, 0.3345653031794291, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3596699001693256, 0.3473291852294986, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3473291852294987, 0.35966990016932554, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3345653031794291, 0.3715724127386971, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3213938048432697, 0.383022221559489, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3078307376628292, 0.39400537680336095, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.29389262614623657, 0.4045084971874737, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.2795964517353734, 0.41451878627752087, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.26495963211660245, 0.424024048078213, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.25000000000000006, 0.4330127018922193, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.23473578139294543, 0.44147379642946344, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.21918557339453873, 0.44939702314958346, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.2033683215379002, 0.4567727288213004, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.1873032967079561, 0.46359192728339366, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.1710100716628344, 0.46984631039295416, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.15450849718747373, 0.47552825814757677, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.13781867790849958, 0.48063084796915945, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.12096094779983384, 0.48514786313799824, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.10395584540887962, 0.48907380036690284, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.08682408883346521, 0.492403876506104, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.06958655048003273, 0.4951340343707852, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.05226423163382673, 0.49726094768413664, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.034878236872062617, 0.4987820251299121, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.01744974835125054, 0.4996954135095479, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [3.061616997868383e-17, 0.5, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.017449748351250367, 0.4996954135095479, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.034878236872062665, 0.4987820251299121, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.052264231633826666, 0.4972609476841367, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.06958655048003257, 0.4951340343707852, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.08682408883346515, 0.492403876506104, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.10395584540887956, 0.48907380036690284, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.12096094779983378, 0.48514786313799824, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.13781867790849953, 0.48063084796915945, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.15450849718747367, 0.4755282581475768, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.17101007166283436, 0.4698463103929542, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.18730329670795604, 0.4635919272833937, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.20336832153790002, 0.4567727288213005, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.21918557339453876, 0.44939702314958346, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.23473578139294526, 0.44147379642946355, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.24999999999999992, 0.43301270189221935, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2649596321166024, 0.42402404807821303, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.27959645173537334, 0.41451878627752087, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2938926261462365, 0.4045084971874737, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.30783073766282915, 0.394005376803361, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.3213938048432697, 0.383022221559489, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.33456530317942895, 0.37157241273869723, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.3473291852294987, 0.35966990016932554, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.3596699001693255, 0.34732918522949874, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.3715724127386972, 0.334565303179429, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.38302222155948895, 0.32139380484326974, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.39400537680336095, 0.3078307376628292, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.40450849718747367, 0.2938926261462366, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4145187862775208, 0.27959645173537345, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.424024048078213, 0.2649596321166025, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.43301270189221935, 0.24999999999999997, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4414737964294635, 0.23473578139294535, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4493970231495834, 0.21918557339453884, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4567727288213005, 0.20336832153790002, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.46359192728339366, 0.18730329670795612, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.46984631039295416, 0.17101007166283444, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.47552825814757677, 0.15450849718747375, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.48063084796915945, 0.1378186779084996, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.48514786313799824, 0.12096094779983387, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.48907380036690284, 0.10395584540887966, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.492403876506104, 0.08682408883346536, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4951340343707852, 0.06958655048003266, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.49726094768413664, 0.05226423163382687, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4987820251299121, 0.03487823687206276, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4996954135095479, 0.01744974835125057, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.5, 6.123233995736766e-17, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4996954135095479, -0.01744974835125045, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.49878202512991215, -0.034878236872062415, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.49726094768413664, -0.05226423163382675, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4951340343707851, -0.06958655048003276, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.492403876506104, -0.08682408883346523, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.48907380036690284, -0.10395584540887953, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.48514786313799824, -0.12096094779983375, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4806308479691595, -0.1378186779084993, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.47552825814757677, -0.15450849718747386, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4698463103929542, -0.17101007166283433, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4635919272833937, -0.187303296707956, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.45677272882130054, -0.2033683215378999, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4493970231495836, -0.21918557339453854, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.44147379642946355, -0.23473578139294524, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4330127018922193, -0.25000000000000006, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.42402404807821303, -0.2649596321166024, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4145187862775209, -0.27959645173537334, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.4045084971874737, -0.2938926261462365, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.3940053768033611, -0.3078307376628289, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.383022221559489, -0.3213938048432696, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.3715724127386971, -0.3345653031794291, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.35966990016932554, -0.3473291852294987, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.34732918522949874, -0.35966990016932543, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.33456530317942923, -0.371572412738697, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.32139380484326974, -0.38302222155948895, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.30783073766282903, -0.39400537680336106, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2938926261462366, -0.40450849718747367, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2795964517353736, -0.4145187862775207, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2649596321166025, -0.424024048078213, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.2500000000000002, -0.4330127018922192, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.23473578139294538, -0.4414737964294635, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.21918557339453887, -0.4493970231495834, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.20336832153790005, -0.4567727288213005, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.18730329670795615, -0.46359192728339366, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.1710100716628347, -0.4698463103929541, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.15450849718747378, -0.47552825814757677, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.13781867790849944, -0.4806308479691595, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.12096094779983389, -0.48514786313799824, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.1039558454088799, -0.4890738003669028, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.08682408883346517, -0.492403876506104, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.06958655048003291, -0.4951340343707851, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.05226423163382712, -0.49726094768413664, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.03487823687206279, -0.4987820251299121, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-0.01744974835125038, -0.4996954135095479, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [-9.184850993605148e-17, -0.5, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.017449748351250197, -0.4996954135095479, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.03487823687206261, -0.49878202512991215, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.05226423163382694, -0.49726094768413664, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.06958655048003273, -0.4951340343707852, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.08682408883346499, -0.49240387650610407, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.10395584540887973, -0.4890738003669028, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.12096094779983373, -0.4851478631379983, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.13781867790849925, -0.4806308479691595, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.15450849718747361, -0.4755282581475768, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.1710100716628345, -0.46984631039295416, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.18730329670795598, -0.4635919272833937, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.20336832153789988, -0.45677272882130054, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.2191855733945387, -0.4493970231495835, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.2347357813929452, -0.44147379642946355, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.25000000000000006, -0.4330127018922193, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.26495963211660234, -0.4240240480782131, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.2795964517353735, -0.4145187862775208, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.29389262614623646, -0.4045084971874738, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3078307376628289, -0.3940053768033612, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3213938048432696, -0.38302222155948906, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.33456530317942923, -0.371572412738697, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3473291852294986, -0.3596699001693256, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.35966990016932543, -0.3473291852294988, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.37157241273869684, -0.3345653031794294, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.3830222215594889, -0.3213938048432698, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.39400537680336106, -0.3078307376628291, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.40450849718747367, -0.2938926261462367, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.41451878627752065, -0.2795964517353737, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4240240480782129, -0.2649596321166025, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4330127018922192, -0.2500000000000002, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4414737964294635, -0.2347357813929454, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4493970231495834, -0.2191855733945389, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.45677272882130043, -0.20336832153790008, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.46359192728339366, -0.18730329670795617, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.46984631039295405, -0.17101007166283472, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.47552825814757677, -0.15450849718747384, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.48063084796915945, -0.13781867790849947, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.48514786313799824, -0.12096094779983393, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4890738003669028, -0.10395584540887993, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.492403876506104, -0.0868240888334652, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4951340343707851, -0.06958655048003294, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.49726094768413664, -0.05226423163382715, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4987820251299121, -0.03487823687206282, 0.0], color: [0.5, 0.0, 0.5] },
    Vertex { position: [0.4996954135095479, -0.017449748351250412, 0.0], color: [0.5, 0.0, 0.5] },
];


const INDICES2: &[u16] = &[
    0, 1, 2,
    0, 2, 3,
    0, 3, 4,
    0, 4, 5,
    0, 5, 6,
    0, 6, 7,
    0, 7, 8,
    0, 8, 9,
    0, 9, 10,
    0, 10, 11,
    0, 11, 12,
    0, 12, 13,
    0, 13, 14,
    0, 14, 15,
    0, 15, 16,
    0, 16, 17,
    0, 17, 18,
    0, 18, 19,
    0, 19, 20,
    0, 20, 21,
    0, 21, 22,
    0, 22, 23,
    0, 23, 24,
    0, 24, 25,
    0, 25, 26,
    0, 26, 27,
    0, 27, 28,
    0, 28, 29,
    0, 29, 30,
    0, 30, 31,
    0, 31, 32,
    0, 32, 33,
    0, 33, 34,
    0, 34, 35,
    0, 35, 36,
    0, 36, 37,
    0, 37, 38,
    0, 38, 39,
    0, 39, 40,
    0, 40, 41,
    0, 41, 42,
    0, 42, 43,
    0, 43, 44,
    0, 44, 45,
    0, 45, 46,
    0, 46, 47,
    0, 47, 48,
    0, 48, 49,
    0, 49, 50,
    0, 50, 51,
    0, 51, 52,
    0, 52, 53,
    0, 53, 54,
    0, 54, 55,
    0, 55, 56,
    0, 56, 57,
    0, 57, 58,
    0, 58, 59,
    0, 59, 60,
    0, 60, 61,
    0, 61, 62,
    0, 62, 63,
    0, 63, 64,
    0, 64, 65,
    0, 65, 66,
    0, 66, 67,
    0, 67, 68,
    0, 68, 69,
    0, 69, 70,
    0, 70, 71,
    0, 71, 72,
    0, 72, 73,
    0, 73, 74,
    0, 74, 75,
    0, 75, 76,
    0, 76, 77,
    0, 77, 78,
    0, 78, 79,
    0, 79, 80,
    0, 80, 81,
    0, 81, 82,
    0, 82, 83,
    0, 83, 84,
    0, 84, 85,
    0, 85, 86,
    0, 86, 87,
    0, 87, 88,
    0, 88, 89,
    0, 89, 90,
    0, 90, 91,
    0, 91, 92,
    0, 92, 93,
    0, 93, 94,
    0, 94, 95,
    0, 95, 96,
    0, 96, 97,
    0, 97, 98,
    0, 98, 99,
    0, 99, 100,
    0, 100, 101,
    0, 101, 102,
    0, 102, 103,
    0, 103, 104,
    0, 104, 105,
    0, 105, 106,
    0, 106, 107,
    0, 107, 108,
    0, 108, 109,
    0, 109, 110,
    0, 110, 111,
    0, 111, 112,
    0, 112, 113,
    0, 113, 114,
    0, 114, 115,
    0, 115, 116,
    0, 116, 117,
    0, 117, 118,
    0, 118, 119,
    0, 119, 120,
    0, 120, 121,
    0, 121, 122,
    0, 122, 123,
    0, 123, 124,
    0, 124, 125,
    0, 125, 126,
    0, 126, 127,
    0, 127, 128,
    0, 128, 129,
    0, 129, 130,
    0, 130, 131,
    0, 131, 132,
    0, 132, 133,
    0, 133, 134,
    0, 134, 135,
    0, 135, 136,
    0, 136, 137,
    0, 137, 138,
    0, 138, 139,
    0, 139, 140,
    0, 140, 141,
    0, 141, 142,
    0, 142, 143,
    0, 143, 144,
    0, 144, 145,
    0, 145, 146,
    0, 146, 147,
    0, 147, 148,
    0, 148, 149,
    0, 149, 150,
    0, 150, 151,
    0, 151, 152,
    0, 152, 153,
    0, 153, 154,
    0, 154, 155,
    0, 155, 156,
    0, 156, 157,
    0, 157, 158,
    0, 158, 159,
    0, 159, 160,
    0, 160, 161,
    0, 161, 162,
    0, 162, 163,
    0, 163, 164,
    0, 164, 165,
    0, 165, 166,
    0, 166, 167,
    0, 167, 168,
    0, 168, 169,
    0, 169, 170,
    0, 170, 171,
    0, 171, 172,
    0, 172, 173,
    0, 173, 174,
    0, 174, 175,
    0, 175, 176,
    0, 176, 177,
    0, 177, 178,
    0, 178, 179,
    0, 179, 180,
    0, 180, 1,
];



#[derive(Debug)]
struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}

#[derive(PartialEq, Eq)]
enum PipelineAlternatives {
    Default,
    Alternative1,
}

struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    bg_color: Color,
    pipeline_alternative: PipelineAlternatives,
    render_pipeline: wgpu::RenderPipeline,
    num_vertices: u32,
    num_indices: u32,
    index_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: Window) -> Self {
        let size = window.inner_size();
        let num_vertices = VERTICES.len() as u32;

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });
        
        // # Safety
        //
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        // Shader code in this tutorial assumes an sRGB surface texture. Using a different
        // one will result all the colors coming out darker. If you want to support non
        // sRGB surfaces, you'll need to account for that when drawing to the frame.
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())            
            .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let bg_color = Color{
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        };

        let pipeline_alternative = PipelineAlternatives::Default;

        let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[],
                push_constant_ranges: &[],
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    Vertex::desc(),
                ], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: None, // 1.
                multisample: wgpu::MultisampleState {
                    count: 1, // 2.
                    mask: !0, // 3.
                    alpha_to_coverage_enabled: false, // 4.
                },
                multiview: None, // 5.
            });

        // BUFFERS
        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(INDICES),
                usage: wgpu::BufferUsages::INDEX,
            }
        );
        let num_indices = INDICES.len() as u32;




        Self {
            window,
            surface,
            device,
            queue,
            config,
            size,
            bg_color,
            pipeline_alternative,
            render_pipeline,
            num_vertices,
            num_indices,
            vertex_buffer,
            index_buffer,
        }

    }

    pub fn window(&self) -> &Window {
        &self.window
    }

    // impl State
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }


    fn input(&mut self, event: &WindowEvent) -> bool {
        match event{
            WindowEvent::CursorMoved { position, .. } => {
                let phys_pos = position.to_owned();
                let normalized_x = phys_pos.x / self.size.width as f64;
                let normalized_y = phys_pos.y / self.size.height as f64;

                self.bg_color.r = normalized_x;
                self.bg_color.g = normalized_y;
            },
            WindowEvent::KeyboardInput { 
                input:
                    KeyboardInput { 
                        state: ElementState::Pressed, 
                        virtual_keycode: Some(VirtualKeyCode::Space), 
                        ..
                    },
                ..
            } => {

                let device = &self.device;


                let shader = device.create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
                let render_pipeline_layout =
                    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("Render Pipeline Layout"),
                        bind_group_layouts: &[],
                        push_constant_ranges: &[],
                    });
                let render_pipeline: wgpu::RenderPipeline;

                if self.pipeline_alternative == PipelineAlternatives::Default {
                    self.pipeline_alternative = PipelineAlternatives::Alternative1;

                    let vertex_buffer = device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(VERTICES),
                            usage: wgpu::BufferUsages::VERTEX,
                        }
                    );

                    let index_buffer = device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(INDICES),
                            usage: wgpu::BufferUsages::INDEX,
                        }
                    );
                    let num_indices = INDICES.len() as u32;

                    self.vertex_buffer = vertex_buffer;
                    self.index_buffer = index_buffer;
                    self.num_indices = num_indices;

                    render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: "vs_main", // 1.
                            buffers: &[
                                Vertex::desc(),
                            ], // 2.
                        },
                        fragment: Some(wgpu::FragmentState { // 3.
                            module: &shader,
                            entry_point: "fs_main",
                            targets: &[Some(wgpu::ColorTargetState { // 4.
                                format: self.config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: wgpu::PrimitiveState {
                            topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                            strip_index_format: None,
                            front_face: wgpu::FrontFace::Ccw, // 2.
                            cull_mode: Some(wgpu::Face::Back),
                            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                            polygon_mode: wgpu::PolygonMode::Fill,
                            // Requires Features::DEPTH_CLIP_CONTROL
                            unclipped_depth: false,
                            // Requires Features::CONSERVATIVE_RASTERIZATION
                            conservative: false,
                        },
                        depth_stencil: None, // 1.
                            multisample: wgpu::MultisampleState {
                                count: 1, // 2.
                                mask: !0, // 3.
                                alpha_to_coverage_enabled: false, // 4.
                            },
                            multiview: None, // 5.
                        });
                } else {
                    self.pipeline_alternative = PipelineAlternatives::Default;

                    let vertex_buffer = device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Vertex Buffer"),
                            contents: bytemuck::cast_slice(VERTICES2),
                            usage: wgpu::BufferUsages::VERTEX,
                        }
                    );

                    let index_buffer = device.create_buffer_init(
                        &wgpu::util::BufferInitDescriptor {
                            label: Some("Index Buffer"),
                            contents: bytemuck::cast_slice(INDICES2),
                            usage: wgpu::BufferUsages::INDEX,
                        }
                    );
                    let num_vertices = VERTICES2.len() as u32;
                    let num_indices = INDICES2.len() as u32;

                    self.vertex_buffer = vertex_buffer;
                    self.index_buffer = index_buffer;
                    self.num_vertices = num_vertices;
                    self.num_indices = num_indices;

                    render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        label: Some("Render Pipeline"),
                        layout: Some(&render_pipeline_layout),
                        vertex: wgpu::VertexState {
                            module: &shader,
                            entry_point: "vs_main", // 1.
                            buffers: &[
                                Vertex::desc(),
                            ], // 2.
                        },
                        fragment: Some(wgpu::FragmentState { // 3.
                            module: &shader,
                            entry_point: "fs2_main",
                            targets: &[Some(wgpu::ColorTargetState { // 4.
                                format: self.config.format,
                                blend: Some(wgpu::BlendState::REPLACE),
                                write_mask: wgpu::ColorWrites::ALL,
                            })],
                        }),
                        primitive: wgpu::PrimitiveState {
                            topology: wgpu::PrimitiveTopology::TriangleList, // 1.
                            //topology: wgpu::PrimitiveTopology::LineStrip, // 1.
                            strip_index_format: None,
                            front_face: wgpu::FrontFace::Ccw, // 2.
                            cull_mode: Some(wgpu::Face::Back),
                            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                            polygon_mode: wgpu::PolygonMode::Fill,
                            // Requires Features::DEPTH_CLIP_CONTROL
                            unclipped_depth: false,
                            // Requires Features::CONSERVATIVE_RASTERIZATION
                            conservative: false,
                        },
                        depth_stencil: None, // 1.
                            multisample: wgpu::MultisampleState {
                                count: 1, // 2.
                                mask: !0, // 3.
                                alpha_to_coverage_enabled: false, // 4.
                            },
                            multiview: None, // 5.
                        });
                }
                self.render_pipeline = render_pipeline;
            },
            _ => ()
        }
        false
    }

    fn update(&mut self) {
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            b: self.bg_color.b,
                            g: self.bg_color.g,
                            r: self.bg_color.r,
                            a: self.bg_color.a,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);

        }

        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();

        Ok(())

    }
}


pub async fn run() {
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    event_loop.run(move |event, _, control_flow| { 
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window().id() => 
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit,
                    WindowEvent::CursorMoved { .. } => {
                        state.input(event);
                    },
                    WindowEvent::KeyboardInput { 
                        input:
                            KeyboardInput { 
                                state: ElementState::Pressed, 
                                virtual_keycode: Some(VirtualKeyCode::Space), 
                                ..
                            },
                        ..
                    } => {
                        state.input(event);
                    },
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        state.resize(**new_inner_size);
                    },
                    _ => {}
                },
            Event::RedrawRequested(window_id) if window_id == state.window().id() => {
                state.update();
                match state.render() {
                    Ok(_) => {}
                    // Reconfigure the surface if lost
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                    // The system is out of memory, we should probably quit
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    // All other errors (Outdated, Timeout) should be resolved by the next frame
                    Err(e) => eprintln!("{:?}", e),
                }
            },
            Event::MainEventsCleared => {
                // RedrawRequested will only trigger once, unless we manually
                // request it.
                state.window().request_redraw();
            },
            _ => {}
        }
    });
}


