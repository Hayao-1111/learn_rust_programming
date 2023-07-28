#[derive(Copy, Clone, Debug)]
pub struct Tetris {
    /// 0-7分别表示 TSZJLIO型方块
    pub kind: usize,
    /// 表示了在游戏中的位置，分别为x和y轴，认为position[0]是纵方向，position[1]是横方向
    pub position: [i32; 2],
    /// 表示了方块在游戏中的指向，0为初始方向，1-3依次顺时针旋转90°
    pub direc: usize,
    /// 表示了方块在游戏中的颜色
    pub color: u8,
}

fn main() {
    println!("Hello, world!");

    let a = Tetris{
        kind: 2,
        position: [2,3],
        direc: 2,
        color: 111
    };

    let b = a;

    // println!("{}", a.color);
    println!("{:?}", a);
    println!("{}", b.color);
}
