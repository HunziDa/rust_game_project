# rust_game_project
Rust程序设计课程项目
## Module state
Module state是该游戏开发的最重要的库，其包含控制游戏模式的enum类型GameMode以及存储游戏上下文信息的struct类型FlappyGameState。
1. 在FlappyGameState中存储有：
   - 当前游戏的模式：主菜单、游戏中、游戏结束；
   - 帧计时：用于物体运动时的位置更新；
   - 玩家信息与障碍物信息：包括位置、速度等；
   - 游戏得分：记录游戏结果；
2. 在FlappyGameState中实现了以下方法：
   - new()：用于创建新的游戏状态结构；
   - start_game()：初始化游戏数据，并将游戏模式设置为游戏中；
   - main_menu()：主菜单界面，按N开始新游戏，按Q退出；
   - game_playing()：游戏界面，在每次被tick调用后接收玩家输入更新玩家和游戏中障碍物的位置等参数
   - game_over()：游戏结束界面，按R重新开始游戏，按Q退出；
3. 在FlappyGameState中实现了bracket_lib::bracket_terminal::GameState的trait：
   - tick()：使用match函数判断当下的模式类别，分别进行不同的处理。
