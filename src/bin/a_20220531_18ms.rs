#![allow(non_snake_case, unused)]
use proconio::*;
use std::collections::VecDeque;
use rand::seq::SliceRandom;

// グローバル変数たち
// グラフ上で進める方向の定義
const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

// 得点を計算する関数
fn compute_score(operation: f64, tree_size: f64, n: f64, t: f64) -> i32 {
    let mut score = 0.0;
    if tree_size < n.powf(2.0) - 1.0 {
        score = (500000.0 * tree_size / (n.powf(2.0) - 1.0)).round();
    } else {
        score = (500000.0 * (2.0 - operation / t)).round();
    }
    score as i32
}

// 1つのタイルを3x3のマスに変換する関数
// 普通のグラフとして扱えるようにするため
// 例:1101(d)のタイル(上だけ行けない)が来たら，
// # # #
// . . .
// # . #
// のような形状に変換する
fn convert_single_tile_to_3x3(tile: &char) -> Vec<Vec<char>> {
    match tile {
        '0' => vec![vec!['#', '#', '#'], vec!['#', '#', '#'], vec!['#', '#', '#']],
        '1' => vec![vec!['#', '#', '#'], vec!['.', '.', '#'], vec!['#', '#', '#']],
        '2' => vec![vec!['#', '.', '#'], vec!['#', '.', '#'], vec!['#', '#', '#']],
        '3' => vec![vec!['#', '.', '#'], vec!['.', '.', '#'], vec!['#', '#', '#']],
        '4' => vec![vec!['#', '#', '#'], vec!['#', '.', '.'], vec!['#', '#', '#']],
        '5' => vec![vec!['#', '#', '#'], vec!['.', '.', '.'], vec!['#', '#', '#']],
        '6' => vec![vec!['#', '.', '#'], vec!['#', '.', '.'], vec!['#', '#', '#']],
        '7' => vec![vec!['#', '.', '#'], vec!['.', '.', '.'], vec!['#', '#', '#']],
        '8' => vec![vec!['#', '#', '#'], vec!['#', '.', '#'], vec!['#', '.', '#']],
        '9' => vec![vec!['#', '#', '#'], vec!['.', '.', '#'], vec!['#', '.', '#']],
        'a' => vec![vec!['#', '.', '#'], vec!['#', '.', '#'], vec!['#', '.', '#']],
        'b' => vec![vec!['#', '.', '#'], vec!['.', '.', '#'], vec!['#', '.', '#']],
        'c' => vec![vec!['#', '#', '#'], vec!['#', '.', '.'], vec!['#', '.', '#']],
        'd' => vec![vec!['#', '#', '#'], vec!['.', '.', '.'], vec!['#', '.', '#']],
        'e' => vec![vec!['#', '.', '#'], vec!['#', '.', '.'], vec!['#', '.', '#']],
        'f' => vec![vec!['#', '.', '#'], vec!['.', '.', '.'], vec!['#', '.', '#']],
        _ => unreachable!(),
    }
}

// ある始点(x, y)からの木を大きさをBFSにより求める
fn compute_tree_size_by_bfs(x: usize, y: usize, n: usize, big_board: &Vec<Vec<char>>) -> i32 {
    let mut deque = VecDeque::new();
    let mut visited = vec![vec![false; 3*n]; 3*n]; // -1は未訪問を示す
    let mut tree_size = 1;
    deque.push_back((x, y));
    visited[x][y] = true;
    while !deque.is_empty() {
        let (frm_x, frm_y) = deque.pop_front().unwrap();
        // 4方向それぞれに進めるかチェック
        for i in 0..4 {
            // 範囲外参照を防ぐ
            if frm_x as i32 + DX[i] < 0 || (frm_x as i32 + DX[i]) as usize > 3*n-1 || frm_y as i32 + DY[i] < 0 || (frm_y as i32 + DY[i]) as usize > 3*n-1 {
                continue;
            }
            let to_x = (frm_x as i32 + DX[i]) as usize;
            let to_y = (frm_y as i32 + DY[i]) as usize;
            // 進めるかつ未訪問なら進む
            if big_board[to_x][to_y] == '.' && !visited[to_x][to_y] {
                // 訪れた頂点がタイルの真ん中のとき，tree_sizeをインクリメントする
                // なんでこうなるかは図を描いたらわかります
                if to_x % 3 == 1 && to_y % 3 == 1 {
                    tree_size += 1;
                }
                // 訪問先を次の始点候補にする
                deque.push_back((to_x, to_y));
                // 訪問済みにする
                visited[to_x][to_y] = true;
            }
        }
    }
    tree_size
}

// 二次元配列の(i1, j1)と(i2, j2)を入れ替える
fn mat_swap_swap_remove<T>(v: &mut Vec<Vec<T>>, i1: usize, j1: usize, i2: usize, j2: usize) {
    if i1 == i2 {
        v[i1].swap(j1, j2);
        return;
    }
    let n = v[i1].len();
    let mut e1 = v[i1].swap_remove(j1);
    std::mem::swap(&mut v[i2][j2], &mut e1);
    v[i1].push(e1);
    v[i1].swap(j1, n - 1);
}

fn solve(n: usize, t: f64, board: &Vec<Vec<char>>) {
    // 各タイルを3x3のマスに変換し，3Nx3Nの盤面を作る
    // そのついでに空きタイルの初期位置をもらう
    let mut big_board = vec![vec!['.'; 3*n]; 3*n];
    let mut empty = (0_usize, 0_usize);
    for i in 0..n {
        for j in 0..n {
            if board[i][j] == '0' {
                empty = (i, j);
            }
            let tmp_tile = convert_single_tile_to_3x3(&board[i][j]);
            for k in 0..3 {
                for l in 0..3 {
                    big_board[3*i+k][3*j+l] = tmp_tile[k][l];
                }
            }
        }
    }

    let mut prev_tree_size = compute_tree_size_by_bfs(3*empty.0+1, 3*empty.1+1, n, &big_board);
    
    // 実行時間制限or回数制限まで以下シミュレートする
    let mut num_operation = 0;
    let mut operation_list = vec![];
    let mut max_tree_size = 0;
    let mut max_tree_operation = vec![]; // [TODO]一度も更新されないと表示部でpanic
    
    while num_operation < t as i32 {
        num_operation += 1;
        // 空きタイルの上下左右のタイルをランダムに引っ張ってきて，木が大きくなったら採用する
        // 方向のインデックスをシャッフルし，順に木を評価する
        // どれを引っぱっても大きくならなければ，最後を採用する
        let mut dir_index: Vec<usize> = vec![0, 1, 2, 3];
        let mut rng = rand::thread_rng();
        dir_index.shuffle(&mut rng);
        'inner: for (i, &dir) in dir_index.iter().enumerate() {
            // 0~3の順番で，下右上左です．
            // 範囲外参照を防ぐ
            if empty.0 as i32 + DX[dir] < 0 || (empty.0 as i32 + DX[dir]) as usize > n-1 || empty.1 as i32 + DY[dir] < 0 || (empty.1 as i32 + DY[dir]) as usize > n-1 {
                continue;
            }
            let next_to_empty_x = (empty.0 as i32 + DX[dir]) as usize;
            let next_to_empty_y = (empty.1 as i32 + DY[dir]) as usize;

            // // emptyとnext_to_emptyを交換する (TODO: boardはいらない？)
            // mat_swap_swap_remove(&mut board, empty.0, empty.1, next_to_empty_x, next_to_empty_y);
            // println!("{:?}", board);

            // big_boardのタイルを交換する
            for k in 0..3 {
                for l in 0..3 {
                    // 一度元空きタイル側に引っ張ってくるタイルの情報を入れこむ
                    big_board[3*empty.0+k][3*empty.1+l] = big_board[3*next_to_empty_x+k][3*next_to_empty_y+l];
                    // 移動後，空きタイルにする
                    big_board[3*next_to_empty_x+k][3*next_to_empty_y+l] = '#';
                }
            }

            // emptyを始点(注:big_board側の座標に変換する必要がある)に，木の大きさを測る
            // tree_sizeが大きくなるなら，big_boardはこのままにして，emptyの座標を更新し，操作をアルファベットに変換してベクタへ保存する
            // 大きくならないなら，big_boardをもとに戻す
            let next_tree_size = compute_tree_size_by_bfs(3*empty.0+1, 3*empty.1+1, n, &big_board);
            if next_tree_size > prev_tree_size || i == 3 {
                match dir {
                    0 => operation_list.push("D"),
                    1 => operation_list.push("R"),
                    2 => operation_list.push("U"),
                    3 => operation_list.push("L"),
                    _ => unreachable!(),
                }
                // [TODO]こんなブサイクなif文ありえる？
                if max_tree_size < next_tree_size {
                    max_tree_size = next_tree_size;
                    max_tree_operation = operation_list.clone();
                }
                // 次のループのためにprevとemptyを更新する
                prev_tree_size = next_tree_size;
                empty = (next_to_empty_x, next_to_empty_y);
                // 4方向のシミュレーションを終わる
                break 'inner;
            } else {
                // big_boardのタイルをもとに戻す
                for k in 0..3 {
                    for l in 0..3 {
                        big_board[3*next_to_empty_x+k][3*next_to_empty_y+l] = big_board[3*empty.0+k][3*empty.1+l];
                        big_board[3*empty.0+k][3*empty.1+l] = '#';
                    }
                }
            }
        }
        // println!("{}", prev_tree_size);
    }

    // println!("{}", max_tree_size);
    // println!("{:?}", max_tree_operation);
    // 木のサイズを最大にするoperationを表示
    for &mtoi in &max_tree_operation {
        print!("{}", &mtoi);
    }
}

#[fastout]
fn main() {
    // 入力の受け取り
    // 型は後で悩む
    input!{
        n: usize,
        t: f64,
    }
    let mut board = vec![];
    for _ in 0..n {
        input!{
            line: marker::Chars,
        }
        board.push(line);
    }
    solve(n, t, &board);
}
