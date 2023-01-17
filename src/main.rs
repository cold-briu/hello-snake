use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{input::RenderArgs, EventLoop, EventSettings, Events, RenderEvent, WindowSettings};

static BG_COLOR: [f32; 4] = [0.105, 0.1, 0.1, 0.01];
static SNAKE_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
pub struct Snake {
    pos: [i32; 2],
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square = graphics::rectangle::square(self.pos[0] as f64, self.pos[1] as f64, 10 as f64);
        gl.draw(args.viewport(), |_c, _gl| {
            let transform = _c.transform;
            graphics::rectangle(SNAKE_COLOR, square, transform, _gl)
        })
    }
}

pub struct Game {
    gl: GlGraphics,
    snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        self.gl
            .draw(args.viewport(), |_c, gl| graphics::clear(BG_COLOR, gl));
        self.snake.render(&mut self.gl, args);
    }
}

fn main() {
    let open_gl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("hello-snake", [800, 800])
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(open_gl),
        snake: Snake { pos: [200, 200] },
    };

    let mut events = Events::new(EventSettings::new()).ups(15);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
    }
}
