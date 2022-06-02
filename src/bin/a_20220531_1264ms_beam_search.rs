#![allow(non_snake_case, unused)]
use proconio::*;
use std::collections::VecDeque;
use rand::seq::SliceRandom;
use std::cmp::PartialOrd;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

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

// 盤面の状態を保持する構造体
// なるべくよい(tree_sizeの大きい)状態をビームサーチの幅数分保持し，探索を進めたい
#[derive(Eq, Clone)]
struct GameState {
    n: usize,
    big_board: Vec<Vec<char>>,
    empty: (usize, usize),
    tree_size: i32,
    operation_list: Vec<char>,
}
// GameStateをpriority_queueに入れるとき，tree_sizeの大きい順に取り出すため，partialordを実装する
impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.tree_size.cmp(&other.tree_size)
    }
}
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.tree_size == other.tree_size
    }
}
impl GameState {
    // 合法手を取得する
    // 前回の手を相殺する手と，範囲外参照を防ぐ
    fn get_legal_actions(&self) -> Vec<usize> {
        let mut actions: Vec<usize> = vec![];
        let mut action_candidates: Vec<usize> = vec![];
        if !self.operation_list.is_empty() {
            action_candidates = match self.operation_list[self.operation_list.len()-1] {
                'D' => vec![0, 1, 3],
                'R' => vec![0, 1, 2],
                'U' => vec![1, 2, 3],
                'L' => vec![0, 2, 3],
                _ => vec![0, 1, 2, 3],
            };
        } else {
            action_candidates = vec![0, 1, 2, 3];
        }

        for &action in &action_candidates {
            if self.empty.0 as i32 + DX[action] < 0 || (self.empty.0 as i32 + DX[action]) as usize > self.n-1 || self.empty.1 as i32 + DY[action] < 0 || (self.empty.1 as i32 + DY[action]) as usize > self.n-1 {
                continue;
            } else {
                actions.push(action);
            }
        }
        actions
    }

    // actionを受けてstateをひとつ進める
    fn advance(&mut self, action: usize) {
        let next_to_empty_x = (self.empty.0 as i32 + DX[action]) as usize;
        let next_to_empty_y = (self.empty.1 as i32 + DY[action]) as usize;
        // big_boardのタイルを交換する
        for k in 0..3 {
            for l in 0..3 {
                // 一度元空きタイル側に引っ張ってくるタイルの情報を入れこむ
                self.big_board[3*self.empty.0+k][3*self.empty.1+l] = self.big_board[3*next_to_empty_x+k][3*next_to_empty_y+l];
                // 移動後，空きタイルにする
                self.big_board[3*next_to_empty_x+k][3*next_to_empty_y+l] = '#';
            }
        }
        self.tree_size = compute_tree_size_by_bfs(3*self.empty.0+1, 3*self.empty.1+1, self.n, &self.big_board);
        self.empty = (next_to_empty_x, next_to_empty_y);
        match action {
            0 => self.operation_list.push('D'),
            1 => self.operation_list.push('R'),
            2 => self.operation_list.push('U'),
            3 => self.operation_list.push('L'),
            _ => unreachable!(),
        }
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
    
    // 実行時間制限まで以下シミュレートする
    // [TODO] 単純な山登り法は盤面の状態を常に1つだけ保持してそれよりいいか悪いかだけを評価しているが
    // 盤面の状態を複数保持できるようにしてビームサーチライクにする
    // 次の状態の計算に必要十分な変数を保持した構造体を定義するとたぶん便利
    // big_board, empty 
    let mut max_tree_size = 0;
    let mut max_tree_operation = vec![]; // [TODO]たぶんないが，一度も更新されないと表示部でpanic
    // [TODO]nの大きさによって変えるなりなんなりしてギリギリを見積もる．
    // epoch=1: 18ms
    // epoch=100: 429ms
    // epoch=1000: 3308ms (40AC, 10TLE, たぶんn=10がTLEしている)
    // let epoch = match n {
    //     6 => 2500,
    //     7 => 2000,
    //     8 => 1500,
    //     9 => 1000,
    //     10 => 600, // 500: 2873ms
    //     _ => unreachable!(),
    // };
    let epoch = 1;
    // ビームサーチの探索幅．各探索時点においていくつの状態を保持することができるか
    // [TODO] nの大きさによってビーム幅を変える
    let beam_width = 100;
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
        // 最初のGameState構造体を初期化
        let game_state = GameState {n: n, big_board: big_board, empty: empty, tree_size: prev_tree_size, operation_list: operation_list};
        // ビームサーチに使用する優先度付きキュー
        let mut current_beam = BinaryHeap::new();
        current_beam.push(game_state);
        // 回数制限(探索木の深さの限界)まで以下シミュレートする
        for _ in 0..t as usize{
            let mut next_beam = BinaryHeap::new();
            // ビーム幅分状態を保持する
            for bw in 0..beam_width {
                if current_beam.is_empty() {
                    break;
                }
                let current_game_state: GameState = current_beam.pop().unwrap();
                // 合法手を取得する
                let legal_actions = current_game_state.get_legal_actions();
                for &action in &legal_actions {
                    let mut next_game_state: GameState = current_game_state.clone();
                    next_game_state.advance(action);
                    next_beam.push(next_game_state);
                }
            }
            current_beam = next_beam;
            let best_state: GameState = current_beam.pop().unwrap(); // [TODO] peek()が使えない
            // [Debug]
            // println!("{}", best_state.tree_size);
            // println!("{}", best_state.operation_list.len());
            // println!("{:?}", best_state.operation_list);
            // 歴代最大を更新したら，手番を保存する
            if max_tree_size < best_state.tree_size {
                max_tree_size = best_state.tree_size;
                max_tree_operation = best_state.operation_list.clone();
            }
            if best_state.tree_size as usize == n*n - 1 {
                break;
            }
            current_beam.push(best_state);
        }        
    }
    // [Debug]
    // println!("{}", max_tree_size);
    for &mtoi in &max_tree_operation {
        print!("{}", &mtoi);
    }
}