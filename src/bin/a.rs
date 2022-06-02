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
    let mut visited = vec![vec![false; 3*n]; 3*n];
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

// 合法手を取得する
// 前回の手を相殺する手と，範囲外参照を防ぐ
fn get_legal_actions(operation_list: &Vec<char>, empty: (usize, usize), n: usize) -> Vec<usize> {
    let mut actions: Vec<usize> = vec![];
    let mut action_candidates: Vec<usize> = vec![];
    if !operation_list.is_empty() {
        action_candidates = match operation_list[operation_list.len()-1] {
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
        if empty.0 as i32 + DX[action] < 0 || (empty.0 as i32 + DX[action]) as usize > n-1 || empty.1 as i32 + DY[action] < 0 || (empty.1 as i32 + DY[action]) as usize > n-1 {
            continue;
        } else {
            actions.push(action);
        }
    }
    actions
}

// 盤面の状態を保持する構造体
// なるべくよい(tree_sizeの大きい)状態をビームサーチの幅数分保持し，探索を進めたい
#[derive(Eq, Clone)]
struct GameState {
    n: usize,
    big_board: Vec<Vec<char>>,
    empty: (usize, usize),
    tree_size: i32,
    has_loop: bool,
    operation_list: Vec<char>,
    turn: usize,
    evaluated_score: i32,
    // [TODO]tree_sizeよりも良い評価値を作る
}
// GameStateをpriority_queueに入れるとき，tree_sizeの大きい順に取り出すため，partialordを実装する
impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        self.evaluated_score.cmp(&other.evaluated_score)
    }
}
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        self.evaluated_score == other.evaluated_score
    }
}
impl GameState {
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
        // 全頂点を始点にしてBFSし，最大の木の大きさを求める (激遅)
        let mut max_tree_size = 0;
        for i in 0..self.n {
            for j in 0..self.n {
                let tmp_tree_size = compute_tree_size_by_bfs(3*i+1, 3*j+1, self.n, &self.big_board);
                if max_tree_size < tmp_tree_size {
                    max_tree_size = tmp_tree_size;
                }
            }
        }
        self.tree_size = max_tree_size;
        // 木にループがあるかを調べる
        let mut visited = vec![vec![false; 3*self.n]; 3*self.n];
        self.has_loop = detect_loop_by_dfs(3*self.empty.0+1, 3*self.empty.1+1, std::usize::MAX, std::usize::MAX, self.n, &self.big_board, &mut visited);
        // [TODO] 木にループがあるときのペナルティを考える
        // 序盤にループができるのは許容したり
        // self.tree_size = compute_tree_size_by_bfs(3*self.empty.0+1, 3*self.empty.1+1, self.n, &self.big_board);
        self.empty = (next_to_empty_x, next_to_empty_y);
        match action {
            0 => self.operation_list.push('D'),
            1 => self.operation_list.push('R'),
            2 => self.operation_list.push('U'),
            3 => self.operation_list.push('L'),
            _ => unreachable!(),
        }
        self.turn += 1;
        // 評価関数の設計
        // 1. 木が大きいほどよい
        // 2. 全域木が完成していれば，手番が少ないほどよい
        // 3. 例えば1番のタイルは左端にない方がよい ← 実装鬼
        // 4. 序盤のループや木の小ささは許容する
        let loop_penalty = if self.has_loop {
            0.01
        } else {
            0.0
        };
        self.evaluated_score = (100.0 * self.tree_size as f64 / self.turn as f64 - loop_penalty * self.turn as f64).round() as i32;


    }
}

#[fastout]
fn main() {
    // 入力の受け取り
    input!{
        n: usize,
        t: usize,
    }
    let mut board = vec![];
    for _ in 0..n {
        input!{
            line: marker::Chars,
        }
        board.push(line);
    }

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
    // 初期の木のサイズを0で初期化
    let mut tree_size = 0;
    // 初期はループなしとする
    let mut has_loop = false;
    // 手番を逐次保存していく
    let mut operation_list = vec![];
    // 現在の手番
    let mut turn = 0;
    // 現在の盤面の評価値
    let mut evaluated_score = 0;
    // 最初のGameState構造体を初期化
    let game_state = GameState {n, big_board, empty, tree_size, has_loop, operation_list, turn, evaluated_score};
    
    // [TODO]chokudai searchに変える
    // このゲームは1回1回の手番でスコアを変えづらい
    // beam serachの結果を見てても収束が早く，局所解から抜け出しにくい
    // 探索に多様性を持たせるため，chokudai searchに変更する
    let mut max_tree_size = 0;
    let mut max_tree_operation = vec![]; // [TODO]たぶんないが，一度も更新されないと表示部でpanic
    // beamの数
    // [TODO]これを大幅に増やすことができないか
    // let beam_number = 4;
    let beam_number = match n {
        6 => 200,
        7 => 150,
        8 => 100,
        9 => 50,
        10 => 30,
        _ => unreachable!(),
    };
    // beam_depthは最大でtだが，早く完成したほうがいいのと，seed=0で80万出してる人いるので，このときは0.4tとかでいいのかも
    // let beam_depth = t;
    let beam_depth = match n {
        6 => t,
        7 => t * 9 / 10,
        8 => t * 8 / 10,
        9 => t * 7 / 10,
        10 => t * 7 / 10,
        _ => unreachable!(),
    };
    // 探索幅．各探索時点においていくつの状態を保持することができるか
    let beam_width = 1;
    // 乱数生成機
    let mut rng = rand::thread_rng();

    // 優先度付きキューの配列．各beamの各ターンで全手番を記憶し，各beamで逐次高いものを取り出す
    let mut beam = vec![];
    let mut first_binary_heap = BinaryHeap::new();
    first_binary_heap.push(game_state);
    beam.push(first_binary_heap);
    for _ in 0..beam_depth {
        let mut bh = BinaryHeap::new();
        beam.push(bh);
    }
    // chokudai search
    for _ in 0..beam_number {
        for depth in 0..beam_depth {
            for _ in 0..beam_width {
                if beam[depth].is_empty() {
                    break;
                    println!("depth {}", depth);
                }
                // 手番depthの中で評価値が最も高いものを取り出す
                // [TODO]確率的に現時点で悪い手も受け入れる？
                let now_state = beam[depth].pop().unwrap();
                // 取りうるアクションをすべて行う
                let mut legal_actions = get_legal_actions(&now_state.operation_list, now_state.empty, n);
                // 各アクション後の評価値が同じ時，いつも同じ順番でnext_stateがheapに突っ込まれるのを防いでみる
                legal_actions.shuffle(&mut rng);
                for &action in &legal_actions {
                    let mut next_state = now_state.clone();
                    next_state.advance(action);
                    // もしtree_size最大を更新するなら手順を保存する
                    if max_tree_size < next_state.tree_size && !next_state.has_loop {
                        max_tree_size = next_state.tree_size;
                        max_tree_operation = next_state.operation_list.clone();
                        // [Debug]
                        // println!("{}", max_tree_size);
                        // println!("{:?}", max_tree_operation);
                    }
                    // [Debug] ループ判定の確認
                    // if next_state.has_loop {
                    //     println!("{}", next_state.tree_size);
                    //     for &tmp in &next_state.operation_list {
                    //         print!("{}", &tmp);
                    //     }
                    // }
                    // println!();

                    // 次のループのために配列にstateを保存する
                    beam[depth+1].push(next_state);
                }
            }
        }        
    }
    
    // [Debug]
    // println!("{}", max_tree_size);
    for &mtoi in &max_tree_operation {
        print!("{}", &mtoi);
    }
}