// use wgpu::PresentMode;
// use winit::event::{InnerSizeWriter, WindowEvent};
// use winit::window::Window;
//
// pub struct State<'window>{
//     surface: wgpu::Surface<'window>,
//     device: wgpu::Device,
//     queue: wgpu::Queue,
//     config: wgpu::SurfaceConfiguration,
//     pub(crate) size: winit::dpi::PhysicalSize<u32>,
//     pub(crate) window: &'window Window,
//     pub(crate) color: wgpu::Color,
// }
//
// impl<'window> State<'window>{
//     pub async fn new(window: &'window Window) -> Self{
//         let size = window.inner_size();
//
//         // Create a GPU handler
//         let instance = wgpu::Instance::new(
//             wgpu::InstanceDescriptor{
//                 backends: wgpu::Backends::all(),
//                 ..Default::default()
//             }
//         );
//
//         // The Surface need to live for the whole lifeTIme Window
//         //      Surface is the part we will draw on
//         let surface = unsafe { instance.create_surface(window) }.unwrap();
//
//         // Handler for the Graphic Card used to retrieve Info
//         let adapter = instance.request_adapter(
//             &wgpu::RequestAdapterOptions{
//                 // power_preference has two mode low and high wgpu default is Low
//                 //       favor battery life
//                 //      High favor performance
//                 power_preference: wgpu::PowerPreference::default(),
//                 force_fallback_adapter: false,
//                 compatible_surface: Some(&surface),
//             }
//         ).await.unwrap();
//
//         let (device, queue) = adapter.request_device(
//             &wgpu::DeviceDescriptor{
//                 label: None,
//                 required_features: wgpu::Features::empty(),
//                 required_limits: wgpu::Limits::default(),
//             },
//             None
//         ).await.unwrap();
//
//         let surface_caps = surface.get_capabilities(&adapter);
//         // Shader code in this tutorial assumes an sRGB surface texture. Using a different
//         // one will result in all the colors coming out darker. If you want to support non
//         // sRGB surfaces, you'll need to account for that when drawing to the frame.
//         let surface_format = surface_caps.formats.iter()
//             .copied()
//             .filter(|f| f.is_srgb())
//             .next()
//             .unwrap_or(surface_caps.formats[0]);
//
//
//         let config = wgpu::SurfaceConfiguration {
//             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
//             format: surface_format,
//             width: size.width,
//             height: size.height,
//             present_mode: PresentMode::Fifo,
//             desired_maximum_frame_latency: 0,
//             alpha_mode: surface_caps.alpha_modes[0],
//             view_formats: vec![],
//
//         };
//         surface.configure(&device, &config);
//
//         let color = wgpu::Color::BLUE;
//
//         Self {
//             window,
//             surface,
//             device,
//             queue,
//             config,
//             size,
//             color
//         }
//     }
//
//     pub fn window(&self) -> &Window{
//         todo!()
//     }
//
//     pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>){
//         println!("Resize Request with new size {} {}", new_size.height, new_size.width);
//         if new_size.width > 0 && new_size.height > 0 {
//             self.size = new_size;
//             self.config.width = new_size.width;
//             self.config.height = new_size.height;
//             self.surface.configure(&self.device, &self.config)
//         }
//     }
//
//     pub fn resize_using_inner_size(&mut self, size : &InnerSizeWriter){
//         println!("Resize Request");
//     }
//
//     pub fn input(&mut self, event: &WindowEvent) -> bool {
//         match event {
//             WindowEvent::CursorMoved { position, .. } => {
//                 self.color = wgpu::Color {
//                     r: position.x / self.size.width as f64,
//                     g: position.y / self.size.height as f64,
//                     b: 1.0,
//                     a: 1.0,
//                 };
//                 self.render().unwrap();
//                 true
//             }
//             _ => false,
//         }
//     }
//
//     pub fn update(&mut self) {
//         todo!()
//     }
//
//     pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
//         let output = self.surface.get_current_texture().unwrap();
//         let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
//
//         let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor{
//             label: Some("Render Encoder")
//         });
//
//         {
//             let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
//                 label: Some("Render Pass"),
//                 color_attachments: &[Some(wgpu::RenderPassColorAttachment {
//                     view: &view,
//                     resolve_target: None,
//                     ops: wgpu::Operations {
//                         load: wgpu::LoadOp::Clear(self.color),
//                         store: wgpu::StoreOp::Store,
//                     },
//                 })],
//                 depth_stencil_attachment: None,
//                 occlusion_query_set: None,
//                 timestamp_writes: None,
//             });
//         }
//
//         // submit will accept anything that implements IntoIter
//         self.queue.submit(std::iter::once(encoder.finish()));
//         output.present();
//
//         Ok(())
//     }
// }