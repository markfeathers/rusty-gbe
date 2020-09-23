use pixels::SurfaceTexture;
use log::error;
use pixels::Pixels;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

/*mod cpu;
mod gpu;*/

mod gameboy;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

fn main() {
    let foo = "a,1,3,4,5";
    let values: Vec<i32> = foo.split(",").filter_map(|x| x.parse::<i32>().ok()).collect();


    //.remove_at(0)
    //.map(|x| x.parse::<i32>().unwrap()).collect();
    
    //println!("{:?}", values)




    env_logger::init();

    let bios = std::fs::read("./src/roms/bios.gb").unwrap();
    let rom: Vec<u8> = std::fs::read("./src/roms/Tetris.gb").unwrap();
    //let rom: Vec<u8> = std::fs::read("./src/roms/opus5.gb").unwrap();
    //let rom: Vec<u8> = std::fs::read("./src/roms/drmario.gb").unwrap();
    //let rom: Vec<u8> = std::fs::read("./src/roms/blaarg/cpu_instrs/cpu_instrs.gb").unwrap();
    //let mut gb = cpu::GameboyCPU::new(bios, rom).unwrap();

    let mut gb = gameboy::GBEmulator::new(bios, rom);
    
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Rusty GB")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture).unwrap()
    };

    loop {
        let cycles = gb.cpu_run_op();
        gb.gpu_run(cycles);
        gb.timers_run(cycles);
        gb.handle_irqs();
        /*if gb.regs.pc == 0x8E {
            break;
        }*/
    }

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();
            
            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                let rgba = &gb.framebuffer[(i*4)..((i*4)+4)];
                //println!("{}: {:?}", i, rgba);
                pixel.copy_from_slice(rgba)
            }

            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            window.request_redraw();
        }

       /* let mut framecycles = 0;
        while framecycles < 70224 {
            let cycles = gb.cpu_run_op();
            gb.gpu_run(cycles);
            gb.timers_run(cycles);
            gb.handle_irqs();
            
            framecycles += cycles;
        }*/
    });
}
