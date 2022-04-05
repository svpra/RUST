use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

fn main() {
    // Создадим цикл обработки событий.
    let event_loop = EventLoop::new();

    let window_size: PhysicalSize<u32> = (800, 600).into();

    // Создадим окно, задав его параметры.
    let window = WindowBuilder::new()
        .with_fullscreen(None)
        .with_inner_size(window_size)
        .with_title("wgpu first steps")
        .build(&event_loop)
        .unwrap();

    // Запустим цикл обработки событий, передав в него замыкание,
    // которое будет выполнятся на кождой итерации цикла.
    event_loop.run(move |event, _, control_flow| {
        // Будем попадать в тело цикла только при появлении события ОС.
        *control_flow = ControlFlow::Wait;

        match event {
            // Если обработаны все накопившиеся события - перерисовываем содержимое окна.
            Event::MainEventsCleared => {
                // todo: код рендера
            }

            // Если было запрошено закрытие окна, завершаем цикл.
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            // Остальные события нам не интересны.
            _ => {}
        }
    })
}
