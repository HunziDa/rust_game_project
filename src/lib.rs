//简化binary crate对library的use调用
pub mod prelude {
    pub use crate::obstacle::Obstacle;
    pub use crate::player::Player;
    pub use crate::state::FlappyGameState;
    pub use crate::{FRAME_DURATION, SCREEN_HEIGHT, SCREEN_WIDTH};
}

//Meta data: terminal窗口大小(size:100*60)与游戏帧率(fps:50)参数
pub const SCREEN_WIDTH: i32 = 120;
pub const SCREEN_HEIGHT: i32 = 180;
pub const FRAME_DURATION: f32 = 20.0;

//游戏组件：玩家与障碍物
pub mod obstacle;
pub mod player;
pub mod state;
