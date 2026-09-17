mod point;
mod playfield;

use std::f32::consts::PI;
use crate::point::Point;
use crate::playfield::Playfield;

use macroquad::prelude::*;


const PLAYFIELD_TILES: usize = 10;
const GRAPHICS_UPDATE_RATE: u32 = 1;



#[macroquad::main("test")]
async fn main() {
    let mut field1 = Playfield::new(0.,0., macroquad::window::screen_width(),macroquad::window::screen_height());
    field1.set_upd_rate(1);
    field1.set_graphics_upd_rate(5000);
    for i in 0..160 {
        field1.spawn_agent();
    }
    let mut tick = 0;
    loop {
        tick += 1;
        if ( tick % GRAPHICS_UPDATE_RATE == 0) {
            clear_background(BLACK);
            field1.render_everyone();
            field1.draw_tiles_connections();

            field1.logic_update();
            println!("ticck {tick}");
            next_frame().await;
        } else {

            field1.logic_update();
            println!("ticck {tick}");
        }
    }
}

struct Agent {
    coord: Point,
    direction: Point,
    speed: f32,
    rad: f32,
}

struct Tile<'a> {
    agents: Vec<&'a Agent>,
    //owner: &'owner_life  Playfield<'owner_life>
}
impl<'a> Tile<'a> {
    pub fn new() -> Tile<'a> {
        Tile{ agents: vec![]}
    }

    pub fn push_agent(&mut self, agent: &'a Agent) {
        self.agents.push(agent);
    }
    pub fn pop_agent(&mut self, agent: Agent) {
       // self.agents.
    }



    pub fn clear(&mut self) {
        self.agents.clear();
    }
}


impl Default for Agent {
    fn default() -> Self {
        Agent{coord: Point::default(), direction: Point::default(), speed: Agent::DEFAULT_AGENT_SPEED, rad: Agent::DEFAULT_AGENT_SIZE}
    }
}

impl Agent {
    const DEFAULT_AGENT_SPEED: f32 = 1.;
    const DEFAULT_AGENT_SPEED_DEVIATION: f32 = 0.5;
    const DEFAULT_AGENT_SIZE: f32 = 5.;
    const AGENT_ROT_DEVIATION: f32 = 0.3;

    const DIRECTION_LINE_MULT: f32 = 10.;

    fn rotate_randomly(&mut self) {
        let rotation = ( ( (rand::gen_range((Self::AGENT_ROT_DEVIATION * -10.) as i32,(Self::AGENT_ROT_DEVIATION * 10.) as i32))as f32 ) ) / 10.;
        self.direction.rotate_by(rotation);
        self.direction.normalise_self_to(self.speed);
    }

    fn new(pos: Point ) -> Agent {
        let speed = rand::gen_range((Self::DEFAULT_AGENT_SPEED- Self::DEFAULT_AGENT_SPEED_DEVIATION) as i64 * 10,(Self::DEFAULT_AGENT_SPEED + Self::DEFAULT_AGENT_SPEED_DEVIATION)as i64*10 ) as f32 / 10.;
        let rotation = (rand::gen_range(0,(PI*2. * 10.) as usize))as f32 / 10.;
        let mut direction = Point::default( );
        direction.rotate_by(rotation);
        direction.normalise_self_to( speed );
        Agent{coord:pos, direction:direction, speed: speed, rad: Self::DEFAULT_AGENT_SIZE}
    }

    fn draw(&self, x_offset: f32, y_offset: f32) {
        draw_circle(self.coord.x() + x_offset,
                    self.coord.y() + y_offset,
                    self.rad, RED);
        draw_line(self.coord.x() + x_offset ,
                  self.coord.y() + y_offset,
                  self.coord.x() + x_offset + self.direction.x() * Self::DIRECTION_LINE_MULT,
                  self.coord.y() + y_offset + self.direction.y() * Self::DIRECTION_LINE_MULT,
                  3., WHITE);
    }



    fn step_forward(&mut self) {
        self.coord.add( &self.direction);
    }
}
