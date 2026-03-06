use std::convert::TryInto;
use std::time::Duration;

use smithay_client_toolkit::reexports::calloop::EventLoop;
use smithay_client_toolkit::reexports::calloop_wayland_source::WaylandSource;
use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_keyboard, delegate_output, delegate_pointer, delegate_registry,
    delegate_seat, delegate_shm, delegate_xdg_shell, delegate_xdg_window,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    seat::{
        keyboard::{KeyEvent, KeyboardHandler, Keysym, Modifiers, RawModifiers},
        pointer::{PointerEvent, PointerHandler},
        Capability, SeatHandler, SeatState,
    },
    shell::{
        xdg::{
            window::{Window, WindowConfigure, WindowDecorations, WindowHandler},
            XdgShell,
        },
        WaylandSurface,
    },
    shm::{
        slot::{Buffer, SlotPool},
        Shm, ShmHandler,
    },
};
use wayland_client::{
    globals::registry_queue_init,
    protocol::{wl_keyboard, wl_output, wl_pointer, wl_seat, wl_shm, wl_surface},
    Connection, QueueHandle,
};

fn main() {
    env_logger::init();

    let conn = Connection::connect_to_env().unwrap();
    let (globals, event_queue) = registry_queue_init::<ConkyLikeWidget>(&conn).unwrap();
    let qh: QueueHandle<ConkyLikeWidget> = event_queue.handle();

    let mut event_loop: EventLoop<ConkyLikeWidget> =
        EventLoop::try_new().expect("Failed to init event loop");
    let loop_handle = event_loop.handle();
    WaylandSource::new(conn.clone(), event_queue)
        .insert(loop_handle)
        .unwrap();

    let compositor = CompositorState::bind(&globals, &qh).expect("wl_compositor not available");
    let shm = Shm::bind(&globals, &qh).expect("wl shm not available");
    let xdg_shell = XdgShell::bind(&globals, &qh).expect("xdg shell not available");

    let surface = compositor.create_surface(&qh);
    let window = xdg_shell.create_window(surface, WindowDecorations::None, &qh);
    window.set_title("Conky Widget");
    window.set_app_id("conky-widget");
    window.set_min_size(Some((300, 100)));
    window.commit();

    let pool = SlotPool::new(300 * 100 * 4, &shm).expect("Failed to create pool");

    let mut widget = ConkyLikeWidget {
        registry_state: RegistryState::new(&globals),
        seat_state: SeatState::new(&globals, &qh),
        output_state: OutputState::new(&globals, &qh),
        shm,
        exit: false,
        first_configure: true,
        pool,
        width: 300,
        height: 100,
        buffer: None,
        window,
    };

    loop {
        event_loop
            .dispatch(Duration::from_millis(16), &mut widget)
            .unwrap();
        if widget.exit {
            break;
        }
    }
}

struct ConkyLikeWidget {
    registry_state: RegistryState,
    seat_state: SeatState,
    output_state: OutputState,
    shm: Shm,
    exit: bool,
    first_configure: bool,
    pool: SlotPool,
    width: u32,
    height: u32,
    buffer: Option<Buffer>,
    window: Window,
}

impl CompositorHandler for ConkyLikeWidget {
    fn scale_factor_changed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: i32,
    ) {
    }
    fn transform_changed(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: wl_output::Transform,
    ) {
    }
    fn frame(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: u32,
    ) {
        self.draw(conn, qh);
    }
    fn surface_enter(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: &wl_output::WlOutput,
    ) {
    }
    fn surface_leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_surface::WlSurface,
        _: &wl_output::WlOutput,
    ) {
    }
}

impl OutputHandler for ConkyLikeWidget {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }
    fn new_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
    fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_output::WlOutput) {}
}

impl WindowHandler for ConkyLikeWidget {
    fn request_close(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &Window) {
        self.exit = true;
    }
    fn configure(
        &mut self,
        conn: &Connection,
        qh: &QueueHandle<Self>,
        _: &Window,
        configure: WindowConfigure,
        _: u32,
    ) {
        self.buffer = None;
        self.width = configure.new_size.0.map(|v| v.get()).unwrap_or(300);
        self.height = configure.new_size.1.map(|v| v.get()).unwrap_or(100);
        if self.first_configure {
            self.first_configure = false;
            self.draw(conn, qh);
        }
    }
}

impl SeatHandler for ConkyLikeWidget {
    fn seat_state(&mut self) -> &mut SeatState {
        &mut self.seat_state
    }
    fn new_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}
    fn new_capability(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: wl_seat::WlSeat,
        _: Capability,
    ) {
    }
    fn remove_capability(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: wl_seat::WlSeat,
        _: Capability,
    ) {
    }
    fn remove_seat(&mut self, _: &Connection, _: &QueueHandle<Self>, _: wl_seat::WlSeat) {}
}

impl KeyboardHandler for ConkyLikeWidget {
    fn enter(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: &wl_surface::WlSurface,
        _: u32,
        _: &[u32],
        _: &[Keysym],
    ) {
    }
    fn leave(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: &wl_surface::WlSurface,
        _: u32,
    ) {
    }
    fn press_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _: KeyEvent,
    ) {
    }
    fn repeat_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _: KeyEvent,
    ) {
    }
    fn release_key(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _: KeyEvent,
    ) {
    }
    fn update_modifiers(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_keyboard::WlKeyboard,
        _: u32,
        _: Modifiers,
        _: RawModifiers,
        _: u32,
    ) {
    }
}

impl PointerHandler for ConkyLikeWidget {
    fn pointer_frame(
        &mut self,
        _: &Connection,
        _: &QueueHandle<Self>,
        _: &wl_pointer::WlPointer,
        _: &[PointerEvent],
    ) {
    }
}

impl ShmHandler for ConkyLikeWidget {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm
    }
}

impl ConkyLikeWidget {
    pub fn draw(&mut self, _: &Connection, qh: &QueueHandle<Self>) {
        let width = self.width;
        let height = self.height;
        let stride = width as i32 * 4;

        let buffer = self.buffer.get_or_insert_with(|| {
            self.pool
                .create_buffer(
                    width as i32,
                    height as i32,
                    stride,
                    wl_shm::Format::Argb8888,
                )
                .expect("create buffer")
                .0
        });

        let canvas = match self.pool.canvas(buffer) {
            Some(canvas) => canvas,
            None => {
                let (second_buffer, canvas) = self
                    .pool
                    .create_buffer(
                        width as i32,
                        height as i32,
                        stride,
                        wl_shm::Format::Argb8888,
                    )
                    .expect("create buffer");
                *buffer = second_buffer;
                canvas
            }
        };

        let text = "Hello Conky!";
        let text_x = 20i32;
        let text_y = 40i32;

        canvas
            .chunks_exact_mut(4)
            .enumerate()
            .for_each(|(index, chunk)| {
                let x = (index / width as usize) as i32;
                let y = (index % width as usize) as i32;

                let in_text = is_point_in_text(x, y, text, text_x, text_y);

                let (r, g, b, a) = if in_text {
                    (0xFF, 0xFF, 0xFF, 0xFF)
                } else {
                    (0x1E, 0x1E, 0x1E, 0xCC)
                };
                let color: u32 = (a << 24) + (r << 16) + (g << 8) + b;

                let array: &mut [u8; 4] = chunk.try_into().unwrap();
                *array = color.to_le_bytes();
            });

        self.window
            .wl_surface()
            .damage_buffer(0, 0, width as i32, height as i32);
        self.window
            .wl_surface()
            .frame(qh, self.window.wl_surface().clone());
        buffer
            .attach_to(self.window.wl_surface())
            .expect("buffer attach");
        self.window.commit();
    }
}

fn is_point_in_text(px: i32, py: i32, text: &str, start_x: i32, start_y: i32) -> bool {
    let char_width = 12;
    let char_height = 24;
    for (i, _) in text.chars().enumerate() {
        let char_x = start_x + (i as i32 * char_width);
        if px >= char_x && px < char_x + char_width && py >= start_y && py < start_y + char_height {
            return true;
        }
    }
    false
}

delegate_compositor!(ConkyLikeWidget);
delegate_output!(ConkyLikeWidget);
delegate_shm!(ConkyLikeWidget);
delegate_seat!(ConkyLikeWidget);
delegate_keyboard!(ConkyLikeWidget);
delegate_pointer!(ConkyLikeWidget);
delegate_xdg_shell!(ConkyLikeWidget);
delegate_xdg_window!(ConkyLikeWidget);
delegate_registry!(ConkyLikeWidget);

impl ProvidesRegistryState for ConkyLikeWidget {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }
    registry_handlers![OutputState, SeatState];
}
