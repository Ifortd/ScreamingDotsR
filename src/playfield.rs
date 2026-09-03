use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use std::sync::Arc;
use macroquad::color::{BLUE, RED};
use macroquad::prelude::{draw_rectangle_lines, rand};
use macroquad::shapes::draw_line;
use crate::{Agent, Tile, PLAYFIELD_TILES};
use crate::point::Point;

pub struct Playfield<'a> {
    coord: Point,
    size: Point,
    tick_update_rate: u64,
    graphics_update_rate: u64,
    current_tick: u64,
    //agents: Vec<Agent>,
    agents: RefCell<Vec<Agent>>,
    tiles: [ [Tile<'a>; PLAYFIELD_TILES]; PLAYFIELD_TILES]
}

impl<'a> Playfield<'a> {
    const DEBUG_LINES: bool = true;

    fn draw(&self) {
        draw_rectangle_lines(self.coord.x(), self.coord.y(),  self.size.x(),  self.size.y(), 2.0, RED);
    }

    pub fn spawn_agent(&mut self) {
        let mut agent_pos_x: i32 = rand::gen_range( 0, self.size.x() as i32 );
        let mut agent_pos_y: i32 = rand::gen_range( 0, self.size.y() as i32 );
        self.push_agent( Agent::new( Point::new( agent_pos_x as f32, agent_pos_y as f32) ) )
    }

    fn push_agent(&mut self, agent: Agent) {
        self.agents.get_mut().push(agent);
    }

    pub fn set_upd_rate(&mut self, tick_update_rate: u64) {
        self.tick_update_rate = tick_update_rate as u64;
    }
    pub fn set_graphics_upd_rate(&mut self, update_rate: u64) {
        self.graphics_update_rate = update_rate as u64;
    }



    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Playfield<'a> {
        let mut tiles: [ [Tile; PLAYFIELD_TILES]; PLAYFIELD_TILES] = std::array::from_fn(|_| ( std::array::from_fn(|_| Tile::new()) ));
        return Playfield{ coord: Point::new(x, y),
            size: Point::new(width, height),
            tick_update_rate: 1,
            graphics_update_rate: 1,
            agents: RefCell::new(vec![]),
            current_tick: 1,  tiles }
    }

    fn tiles_width(&self) -> f32 {
        ( self.size.x() / PLAYFIELD_TILES as f32 )
    }
    fn tiles_height(&self) -> f32 {
        ( self.size.y() / PLAYFIELD_TILES as f32 )
    }
    fn get_agent_column(&self, agent: &Agent) -> usize {
        ( agent.coord.x() / (self.size.x() +1. ) * PLAYFIELD_TILES as f32   )as usize
    }

    fn get_agent_row(&self,agent: &Agent) -> usize {
        ( agent.coord.y() / (self.size.y() +1. ) * PLAYFIELD_TILES as f32   )as usize
    }

    fn attach_agent_to_tile(&mut self, agent: &'a Agent) {
        let x: usize = self.get_agent_column(agent);
        let y: usize = self.get_agent_row(agent);

        if Self::DEBUG_LINES {
          draw_line(
              agent.coord.x(),
              agent.coord.y(),
              x as f32 * self.tiles_width()  + self.tiles_width()/2. ,
              y as f32 * self.tiles_height()  + self.tiles_height()/2.,
              2.,
              BLUE
          )
        }
        self.tiles[x][y].push_agent(agent);
    }

    fn detach_agent_from_title(&mut self, agent: &'a Agent) {
        let x: usize = self.get_agent_column(agent);
        let y: usize = self.get_agent_row(agent);
        //self.tiles[x][y];
    }

    pub fn render_everyone(&self) {
        self.draw();

        for agent in self.agents.borrow().iter() {
            self.render_agent(&agent);
        }

    }

    fn walk_agent(&mut self, agent: &mut Agent) {

        agent.step_forward();
        agent.rotate_randomly();
        if agent.coord.x() < 0. {agent.coord.set_x( self.size.x() )}
        if agent.coord.y() < 0. {agent.coord.set_y( self.size.y() )}

        if agent.coord.x() > self.size.x() {agent.coord.set_x( 0. )}
        if agent.coord.y() > self.size.y() {agent.coord.set_y( 0. )}

    }



    pub fn logic_update(&mut self) {
        self.current_tick += 1;

        if (self.current_tick % self.tick_update_rate == 0) {
            //let mut agents = std::mem::take(&mut self.agents);
            let mut agents =  self.agents.as_ptr();
            unsafe {
                for i in 0..(*agents).len() {
                    let mut agent = (&mut (*agents)).get_mut(i).unwrap();
                    self.walk_agent(  agent) ;
                    self.attach_agent_to_tile( agent);
                }
            }
        }
    }

    pub fn logic_update2(&mut self) {
        self.current_tick += 1;

        if (self.current_tick % self.tick_update_rate == 0) {
            //let mut agents = std::mem::take(&mut self.agents);
            let mut agents =  self.agents.as_ptr();
            unsafe {
                for i in 0..(*agents).len() {
                    let mut agent = (&mut (*agents)).get_mut(i).unwrap();
                    self.walk_agent(  agent) ;
                    self.attach_agent_to_tile( agent);
                }
            }
        }
    }

    fn render_agent(&self, agent: &Agent) {
        agent.draw(self.coord.x(), self.coord.y())
    }

    pub fn new_test() -> Playfield<'a> {
        return Playfield::new(15.0, 15.0, 400.0, 400.0);
    }
}