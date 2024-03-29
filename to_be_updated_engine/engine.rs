// use image::{Rgb, ImageBuffer};
// use winit::event::{Event, WindowEvent};
// use winit::event_loop::{ControlFlow, EventLoop};
// use winit::window::Window;
// use crate::to_be_updated_engine::window::State;
//
// pub struct Engine {}
//
// impl Engine {
//     pub async fn run(){
//         env_logger::init();
//         let event_loop = EventLoop::new().unwrap();
//         let window = Window::new(&event_loop).unwrap();
//         let mut state = State::new(&window).await;
//
//         event_loop.set_control_flow(ControlFlow::Wait);
//
//
//         event_loop.run(move |event, target|{
//             match event {
//                 Event::WindowEvent {
//                     window_id, ref event
//                 } => if !state.input(event) {
//                     match event {
//
//                         WindowEvent::CloseRequested => {
//                             target.exit()
//                         }
//
//                         WindowEvent::Resized(physical_size) => {
//                             state.resize(*physical_size);
//                         }
//
//                         WindowEvent::ScaleFactorChanged {
//                             scale_factor, inner_size_writer
//                         } => {
//                             // new_inner_size is &&mut, so we have to dereference it twice
//                             state.resize_using_inner_size(inner_size_writer);
//                         }
//
//                         WindowEvent::RedrawRequested => {
//                             state.update();
//                             match state.render() {
//                                 Ok(_) => {}
//                                 // Reconfigure the surface if lost
//                                 Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
//                                 // The system is out of memory, we should probably quit
//                                 Err(wgpu::SurfaceError::OutOfMemory) => target.exit(),
//                                 // All other errors (Outdated, Timeout) should be resolved by the next frame
//                                 Err(e) => eprintln!("{:?}", e),
//                             }
//
//                             // Example: Accessing RGB values of a pixel at coordinates (x, y)
//
//
//                             // In this example, we're just printing the RGB values of a single pixel.
//                             // You can perform any other image processing operations here.
//
//                         }
//                         // WindowEvent::KeyboardInput {
//                         //
//                         // }
//                         _=>{}
//                     }
//                 }
//
//                 _ => {}
//             }
//         }).unwrap()
//     }
// }