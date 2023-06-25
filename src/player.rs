use bracket_lib::prelude::*;

/*  module player定义玩家信息，包括位置坐标、运动速度与加速度
    在Player上实现的方法有：
        new()：生成新玩家的信息；
        render()：在屏幕上渲染出玩家；
        gravity_and_move()：以帧为单位的上抛运动；
        flap()：跳越。
*/

pub struct Player {
    pub x: i32,        //位置横坐标
    pub y: i32,        //位置纵坐标
    velocity: f32,     //玩家当前纵向的运动速度
    acceleration: f32, //玩家的加速度
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            velocity: 0.0,
            acceleration: 0.3, //默认加速度为0.3
        }
    }
    pub fn render(&mut self, ctx: &mut BTerm) {
        //玩家的屏幕体积为3x3
        for offset_x in [-1, 0, 1] {
            for offset_y in [-1, 0, 1] {
                ctx.set(
                    self.x + offset_x,
                    self.y + offset_y,
                    YELLOW,
                    BLACK,
                    to_cp437('@'),
                );
            }
        }
    }
    pub fn gravity_and_move(&mut self) {
        if self.velocity <= 2.0 {
            self.velocity += self.acceleration; //速度增加，速度上限是2.0
        }
        //纵坐标位置更新
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0;
        }
    }
    pub fn flap(&mut self) {
        self.velocity = -3.0; //跳越后玩家的速度为3.0，方向向上
    }
}
