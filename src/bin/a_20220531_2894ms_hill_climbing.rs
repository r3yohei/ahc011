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

// ある始点(x, y)から始まる木がループを持つかどうかDFSで判定する
fn detect_loop_by_dfs(current_x: usize, current_y: usize, prev_x: usize, prev_y: usize, n: usize, big_board: &Vec<Vec<char>>, mut visited: &mut Vec<Vec<bool>>) -> bool {
    visited[current_x][current_y] = true;
    for i in 0..4 {
        // 範囲外参照を防ぐ
        if current_x as i32 + DX[i] < 0 || (current_x as i32 + DX[i]) as usize > 3*n-1 || current_y as i32 + DY[i] < 0 || (current_y as i32 + DY[i]) as usize > 3*n-1 {
            continue;
        }
        let to_x = (current_x as i32 + DX[i]) as usize;
        let to_y = (current_y as i32 + DY[i]) as usize;
        // 行き先が元の頂点だったら飛ばす
        if to_x == prev_x && to_y == prev_y {
            continue;
        }
        // 行き先に道がなかったら飛ばす
        if big_board[to_x][to_y] == '#' {
            continue;
        }
        // 行ったことのある頂点に行こうとしたらループがある
        if visited[to_x][to_y] {
            return true;
        }
        detect_loop_by_dfs(to_x, to_y, current_x, current_y, n, big_board, &mut visited);
    }
    false
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
    
    // 実行時間制限まで以下シミュレートする
    let mut max_tree_size = 0;
    let mut max_tree_operation = vec![]; // [TODO]たぶんないが，一度も更新されないと表示部でpanic
    // [TODO]nの大きさによって変えるなりなんなりしてギリギリを見積もる．
    // epoch=1: 18ms
    // epoch=100: 429ms
    // epoch=1000: 3308ms (40AC, 10TLE, たぶんn=10がTLEしている)
    let epoch = match n {
        6 => 500,
        7 => 400,
        8 => 300,
        9 => 200,
        10 => 100, // 500: 2873ms
        _ => unreachable!(),
    };
    for _ in 0..epoch {
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
        // 初期の木のサイズを取得
        let mut prev_tree_size = compute_tree_size_by_bfs(3*empty.0+1, 3*empty.1+1, n, &big_board);
        // 手番を逐次保存していく
        let mut operation_list = vec![];
        // 前回の操作
        let mut prev_operation = "tmp";
        // 回数制限まで以下シミュレートする
        for _ in 0..t as usize{
            // 空きタイルの上下左右のタイルをランダムに引っ張ってきて，木が大きくなったら採用する
            // 方向のインデックスをシャッフルし，順に木を評価する
            // どれを引っぱっても大きくならなければ，最後を採用する
            // 前回の操作の反対を選ばないようにする(例えば，Uの後でDを選ぶのは何もしないに等しいので意味がない)
            let mut dir_index: Vec<usize> = match prev_operation {
                "D" => vec![0, 1, 3], // 上:2を選ばない
                "R" => vec![0, 1, 2],
                "U" => vec![1, 2, 3],
                "L" => vec![0, 2, 3],
                _ => vec![0, 1, 2, 3],
            };
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
                // [TODO]ループがあってもサイズを測ってしまうので，ループがあった場合に長さを0にする処理を入れる
                let mut visited = vec![vec![false; 3*n]; 3*n];
                let has_loop = detect_loop_by_dfs(3*empty.0+1, 3*empty.1+1, std::usize::MAX, std::usize::MAX, n, &big_board, &mut visited);
                let next_tree_size = if !has_loop {
                    compute_tree_size_by_bfs(3*empty.0+1, 3*empty.1+1, n, &big_board)
                } else {
                    0
                };
                if next_tree_size > prev_tree_size || dir == *dir_index.last().unwrap() {
                    match dir {
                        0 => {
                            operation_list.push("D");
                            prev_operation = "D";
                        },
                        1 => {
                            operation_list.push("R");
                            prev_operation = "R";
                        },
                        2 => {
                            operation_list.push("U");
                            prev_operation = "U";
                        },
                        3 => {
                            operation_list.push("L");
                            prev_operation = "L";
                        },
                        _ => unreachable!(),
                    }
                    // 歴代最大を更新したら，手番を保存する
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
    }

    // println!("{}", max_tree_size);
    // println!("{:?}", max_tree_operation);
    // 木のサイズを最大にするoperationを表示
    // [TODO] 'DU'や'LR'といった並びは意味がないので消せる→もし全域木が完成していたら少ない手番のほうがいいので消す
    // シミュレート中に消したほうがいいかも．
    for &mtoi in &max_tree_operation {
        print!("{}", &mtoi);
    }
}