use crate::player::Player;
use crate::SCREEN_HEIGHT;
use bracket_lib::prelude::*;

/*  module obstacle定义了游戏中的障碍物信息，包括其位置、空隙大小、运动速度等；
   其上实现了一下方法：
       new()：生成新的障碍物；
       render()：在屏幕中渲染障碍物；
       hit_obstacle()：判断玩家是否撞到障碍物；
*/

pub struct Obstacle {
    pub x: i32,             //障碍物位置
    pub gap_mid: i32,       //空隙正中间的坐标
    pub gap_size: i32,      //空隙大小：其会随游戏得分增高而减小
    pub move_velocity: f32, //障碍物向左运动速度
}

impl Obstacle {
    pub fn new(x: i32, score: i32) -> Self {
        //使用bracket_lib中自带的module random来实现生成空隙位置随机的障碍物
        let mut random = RandomNumberGenerator::new();
        Obstacle {
            x,
            gap_mid: random.range(SCREEN_HEIGHT / 10, SCREEN_HEIGHT / 10 * 9), // 不包含40
            gap_size: i32::max(7, SCREEN_HEIGHT / 5 - score / 3), // score玩家的积分，积分越多洞越窄
            move_velocity: 1.0,
        }
    }
    pub fn render(&mut self, ctx: &mut BTerm) {
        let render_x = self.x - (self.move_velocity as i32);
        self.x -= self.move_velocity as i32;
        //障碍物在屏幕中的宽度是5
        for offset in [-2, -1, 0, 1, 2] {
            for y in 0..self.gap_mid - self.gap_size / 2 {
                ctx.set(render_x + offset, y, GREEN, BLACK, to_cp437('|'));
            }
            for y in self.gap_mid + self.gap_size / 2..SCREEN_HEIGHT {
                ctx.set(render_x + offset, y, GREEN, BLACK, to_cp437('|'))
            }
        }
    }
    pub fn hit_obstacle(&self, player: &Player) -> bool {
        let half_size = self.gap_size / 2;
        //障碍物的碰撞体积宽度为5，为减小游戏难度玩家，玩家实际碰撞体积是1x1
        let does_x_match = (player.x <= self.x + 2) && (player.x >= self.x - 2);
        let player_above_gap = player.y < self.gap_mid - half_size;
        let player_below_gap = player.y > self.gap_mid + half_size;
        does_x_match && (player_above_gap || player_below_gap)
    }
}
