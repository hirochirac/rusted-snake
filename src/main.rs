extern crate sdl2;
extern crate num_integer;
extern crate rand;

use rand::Rng;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::Duration;
use num_integer::Roots;


const HEIGHT : u32 = 800;
const WIDTH  : u32 = 800;
const SIZE   : u32 = 20;
const SPEED  : i32 = 20;


#[derive(Debug)]
struct SnakeHead {
    x     : i32,
    y     : i32,
    dir_x : i32,
    dir_y : i32
}

impl Copy for SnakeHead {}

impl Clone for SnakeHead {

    fn clone(&self) -> SnakeHead {
        SnakeHead{
            x     : self.x,
            y     : self.y,
            dir_x : self.dir_x,
            dir_y : self.dir_y
        }
    }

}

impl SnakeHead {

    /**
     * 
     */
    fn new (x : i32, y: i32, dir_x : i32, dir_y : i32) -> SnakeHead{
        SnakeHead {
            x     : x,
            y     : y,
            dir_x : dir_x,
            dir_y : dir_y
        }
    }

    /**
     * 
     */
    fn init () -> SnakeHead{
        SnakeHead {
            x : 400,
            y : 400,
            dir_x : 0,
            dir_y : 1
        }
    }

}


#[derive(Debug)]
struct Apple {
    x     : i32,
    y     : i32,
}


impl Apple {
    
    fn reset () -> Apple{
        let mut rng = rand::thread_rng();
        let mut x = rng.gen_range(0 .. 40) * 20;
        let mut y = rng.gen_range(0 .. 40) * 20;
        if x >= 800 {
            x = 780;
        }
        if y >= 800 {
            y = 780;
        }
        Apple {
            x: x,
            y: y
        }
    } 

}


fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Snake",WIDTH, HEIGHT).position_centered().build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();

    let mut hit : usize = 1;
    let mut apple : Apple = Apple::reset();
    let mut tail= [SnakeHead::init();1600];
    let mut tmp = [SnakeHead::init();1600]; 
    let mut event_pump = sdl_context.event_pump().unwrap();

    
    tail[0] = SnakeHead::new(400,400,0,1);
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode : Some(Keycode::Escape), .. } => {break 'running},
                Event::KeyDown {keycode: Some(Keycode::Right), .. } => {
                                                                        tail[0].dir_x = 1;
                                                                        tail[0].dir_y = 0;
                                                                       },
                Event::KeyDown {keycode: Some(Keycode::Left), .. } => {
                                                                        tail[0].dir_x = -1;
                                                                        tail[0].dir_y = 0;
                                                                      },
                Event::KeyDown {keycode: Some(Keycode::Up), .. } => {
                                                                        tail[0].dir_x = 0;
                                                                        tail[0].dir_y = -1;
                                                                    },
                Event::KeyDown {keycode: Some(Keycode::Down), .. } => {
                                                                        tail[0].dir_x = 0;
                                                                        tail[0].dir_y = 1;
                                                                      },
                _ => {}
            }
        }

        if snake_eat_apple (&apple, &tail[0]) == 0  {
            apple = Apple::reset();
            hit = hit + 1;            
        }

        for k in 0 .. hit {
            tmp[k] = tail[k];
        }

        for l in 1 .. hit {
            tail[l] = tmp[l-1];
        }

        tail[0].x = tail[0].x + SPEED * tail[0].dir_x;
        tail[0].y = tail[0].y + SPEED * tail[0].dir_y;

        if tail[0].x <= 0 {
            tail[0].x = 0;
        }
        
        if tail[0].x >= 780 {
            tail[0].x = 780;
        }

        if tail[0].y <= 0 {
            tail[0].y = 0;
        }
        
        if tail[0].y >= 780 {
            tail[0].y = 780;
        }

        if snake_eat_snake(&tail,hit) == true {
            hit = 0;
            tail = [SnakeHead::init();1600];
            tmp  = [SnakeHead::init();1600];
            tail[0] = SnakeHead::new(400,400,0,1);
            apple = Apple::reset();
        }

        for i in 0 .. hit {
            draw_snake_head(&tail[i], &mut canvas);
        }

        draw_apple(&apple,&mut canvas);

        canvas.present();
        ::std::thread::sleep(Duration::new(1, 0));
    }

}

fn snake_eat_apple (apple : &Apple, snake: &SnakeHead) -> i32 {
    let _rectA : Rect = Rect::new(apple.x, apple.y, SIZE, SIZE);
    let _rectS : Rect = Rect::new(snake.x, snake.y, SIZE, SIZE);
    dist(_rectS.center().x().try_into().unwrap(),
         _rectS.center().y().try_into().unwrap(), 
         _rectA.center().x().try_into().unwrap(), 
         _rectA.center().y().try_into().unwrap())
}

fn draw_snake_head (snake: &SnakeHead, canvas : &mut Canvas<Window>) {
    let rect = Rect::new(snake.x,snake.y, SIZE, SIZE);
    canvas.set_draw_color(Color::RGB(0, 255, 0));
    canvas.fill_rect(rect).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

fn draw_apple (apple : &Apple, canvas : &mut Canvas<Window>) {
    let rect = Rect::new(apple.x,apple.y, SIZE, SIZE);
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(rect).unwrap();
    canvas.set_draw_color(Color::RGB(0, 0, 0));
}

fn snake_eat_snake (keu : &[SnakeHead;1600], h : usize) -> bool {
    let head : &SnakeHead = &keu[0];
    let _rectA : Rect = Rect::new(head.x, head.y, SIZE, SIZE);
    let mut _rectS : Rect;
    for i  in 2 .. h {
        _rectS = Rect::new(keu[i].x, keu[i].y, SIZE, SIZE);
        if dist(_rectA.center().x.try_into().unwrap(),
                _rectA.center().y.try_into().unwrap(), 
                _rectS.center().x.try_into().unwrap(), 
                _rectS.center().y.try_into().unwrap()) == 0 {
            return true;
        }
    }
    return false
} 


fn dist (x1 : i32, y1 : i32, x2 : i32, y2 : i32 ) -> i32 {
    let d = (x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1);
    Roots::sqrt(&d) 
}
