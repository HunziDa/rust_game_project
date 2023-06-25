use crate::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
use bracket_lib::prelude::*;

/*  Module state是该游戏开发的最重要的库，其包含控制游戏模式的enum类型GameMode以及存储游戏
   上下文信息的struct类型FlappyGameState。
   在FlappyGameState中存储有：
       当前游戏的模式：主菜单、游戏中、游戏结束；
       帧计时：用于物体运动时的位置更新；
       玩家信息与障碍物信息：包括位置、速度等；
       游戏得分：记录游戏结果；
   在FlappyGameState中实现了以下方法：
       new()：用于创建新的游戏状态结构；
       start_game()：初始化游戏数据，并将游戏模式设置为游戏中；
       main_menu()：主菜单界面，按N开始新游戏，按Q退出；
       game_playing()：游戏界面，在每次被tick调用后接收玩家输入更新玩家和游戏中障碍物的位置等参数
       game_over()：游戏结束界面，按R重新开始游戏，按Q退出；
   在FlappyGameState中实现了bracket_lib::bracket_terminal::GameState的trait：
       tick()：使用match函数判断当下的模式类别，分别进行不同的处理。
*/

pub enum GameMode {
    MainMenu,
    GamePlaying,
    GameOver,
}

pub struct FlappyGameState {
    game_mode: GameMode,                  //游戏模式
    frame_time_interval: f32,             //帧计时，相当于时钟
    player: crate::player::Player,        //玩家信息
    obstacle1: crate::obstacle::Obstacle, //障碍物1
    obstacle2: crate::obstacle::Obstacle, //障碍物2
    score: i32,                           //游戏得分
}

impl FlappyGameState {
    pub fn new() -> Self {
        //初始化游戏状态
        FlappyGameState {
            game_mode: GameMode::MainMenu,
            frame_time_interval: 0.0,
            player: crate::player::Player::new(SCREEN_WIDTH / 10, SCREEN_HEIGHT / 4),
            obstacle1: crate::obstacle::Obstacle::new(SCREEN_WIDTH / 2, 0),
            obstacle2: crate::obstacle::Obstacle::new(SCREEN_WIDTH, 0),
            score: 0,
        }
    }
    pub fn start_game(&mut self) -> () {
        //初始化游戏状态
        self.player = crate::player::Player::new(SCREEN_WIDTH / 10, SCREEN_HEIGHT / 4);
        self.frame_time_interval = 0.0;
        self.score = 0;
        self.obstacle1 = crate::obstacle::Obstacle::new(SCREEN_WIDTH / 2, 0);
        self.obstacle2 = crate::obstacle::Obstacle::new(SCREEN_WIDTH, 0);
        self.game_mode = GameMode::GamePlaying;
    }
    pub fn main_menu(&mut self, ctx: &mut BTerm) -> () {
        ctx.cls();
        ctx.print_centered(SCREEN_HEIGHT / 2 - 3, "Rust Course Project: Flappy @ !");
        ctx.print_centered(SCREEN_HEIGHT / 2 - 1, "(N) New Game");
        ctx.print_centered(SCREEN_HEIGHT / 2, "(Q) Quit Game");
        //接收用户输入并反应
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::N => self.start_game(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            };
        };
    }
    pub fn game_playing(&mut self, ctx: &mut BTerm) -> () {
        ctx.cls_bg(NAVYBLUE);
        ctx.print(0, 0, "Press (Space) to Flap");
        ctx.print(0, 1, &format!("Score: {}", self.score));
        //时钟计时，每满一帧的时间就刷新玩家的位置信息
        self.frame_time_interval += ctx.frame_time_ms;
        if self.frame_time_interval > FRAME_DURATION {
            self.frame_time_interval = 0.0;
            self.player.gravity_and_move();
        }
        //接受玩家输入的跳越信息
        if let Some(VirtualKeyCode::Space) = ctx.key {
            self.player.flap();
        }
        //在屏幕中渲染玩家和障碍物
        self.player.render(ctx);
        self.obstacle1.render(ctx);
        self.obstacle2.render(ctx);
        // 判断是否得分
        if self.player.x == self.obstacle1.x {
            self.score += 1;
        }
        if self.player.x == self.obstacle2.x {
            self.score += 1;
        }
        //判断是否生成新的障碍物
        if self.obstacle1.x <= 1 {
            self.obstacle1 = crate::obstacle::Obstacle::new(SCREEN_WIDTH, self.score);
        }
        if self.obstacle2.x <= 1 {
            self.obstacle2 = crate::obstacle::Obstacle::new(SCREEN_WIDTH, self.score);
        }
        // 判断是否撞到障碍物或者掉出屏幕外
        if self.player.y > SCREEN_HEIGHT + 1
            || self.obstacle1.hit_obstacle(&self.player)
            || self.obstacle2.hit_obstacle(&self.player)
        {
            self.game_mode = GameMode::GameOver;
        }
    }
    pub fn game_over(&mut self, ctx: &mut BTerm) -> () {
        ctx.cls();
        ctx.print_centered(SCREEN_HEIGHT / 2 - 4, "Flappy @: Game Over!");
        ctx.print_centered(
            SCREEN_HEIGHT / 2 - 3,
            format!("Your score is: {}.", self.score),
        );
        ctx.print_centered(SCREEN_HEIGHT / 2 - 1, "(R) Restart Game");
        ctx.print_centered(SCREEN_HEIGHT / 2, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::R => self.start_game(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            };
        };
    }
}

impl GameState for FlappyGameState {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.game_mode {
            GameMode::MainMenu => self.main_menu(ctx),
            GameMode::GamePlaying => self.game_playing(ctx),
            GameMode::GameOver => self.game_over(ctx),
        };
    }
}
