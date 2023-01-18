use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{
    input::RenderArgs, EventLoop, EventSettings, Events, RenderEvent, UpdateEvent, WindowSettings,
};

static CANVAS_SIZE: [u32; 2] = [800, 800];
static BG_COLOR: [f32; 4] = [0.105, 0.1, 0.1, 0.01];
static SNAKE_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
static RATIO: i32 = 10;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Snake {
    pos: [i32; 2],
    direction: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        let square = graphics::rectangle::square(self.pos[0] as f64, self.pos[1] as f64, 10 as f64);
        gl.draw(args.viewport(), |_c, _gl| {
            let transform = _c.transform;
            graphics::rectangle(SNAKE_COLOR, square, transform, _gl)
        })
    }

    fn update(&mut self) {
        if self.pos[1] >= CANVAS_SIZE[1] as i32 {
            self.direction = Direction::Up;
        }
        if self.pos[1] <= 1 as i32 {
            self.direction = Direction::Down;
        }
        if self.pos[0] >= CANVAS_SIZE[0] as i32 {
            self.direction = Direction::Left
        }
        if self.pos[0] <= 1 as i32 {
            self.direction = Direction::Right
        }
        match self.direction {
            Direction::Up => self.pos[1] -= 1 * RATIO,
            Direction::Right => self.pos[0] += 1 * RATIO,
            Direction::Down => self.pos[1] += 1 * RATIO,
            Direction::Left => self.pos[0] -= 1 * RATIO,
        }
    }
}

pub struct Game {
    gl: GlGraphics,
    entities: [Snake; 2],
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        self.gl
            .draw(args.viewport(), |_c, gl| graphics::clear(BG_COLOR, gl));
        for e in &mut self.entities {
            e.render(&mut self.gl, args)
        }
    }
    fn update(&mut self) {
        for e in &mut self.entities {
            e.update()
        }
    }
}

fn main() {
    let open_gl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new("hello-snake", CANVAS_SIZE)
        .graphics_api(open_gl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(open_gl),
        entities: [
            Snake {
                pos: [200, 200],
                direction: Direction::Right,
            },
            Snake {
                pos: [200, 200],
                direction: Direction::Up,
            },
        ],
    };

    let mut events = Events::new(EventSettings::new()).ups(60);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }
        if let Some(_) = e.update_args() {
            game.update();
        }
    }
}
