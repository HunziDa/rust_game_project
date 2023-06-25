use bracket_lib::prelude::*;
use rust_course_project::prelude::*;

//main函数返回BError类型，便于使用bracket_lib中函数时进行错误传播
fn main() -> BError {
    //游戏参数meta data在库中保存
    let context = BTermBuilder::simple(SCREEN_WIDTH, SCREEN_HEIGHT)?
        .with_title("Rust Course Projiect: Flappy @")
        .build()?;
    //进入bracket_lib提供game loop功能的函数main_loop()中循环
    main_loop(context, FlappyGameState::new())?;
    Ok(())
}
