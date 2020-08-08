
    #[derive(Debug, Clone)]
    pub struct Grid {
        xy_grid: Vec<Vec<i32>>,
    }
    impl Grid {
        pub fn new(val: Vec<Vec<i32>>) -> Grid {
            Grid { xy_grid: val }
        }
        pub fn get_as_vec(&self) -> Vec<Vec<i32>>{
            self.xy_grid.clone()
        }
        fn place(&mut self, x: usize, y: usize, val: i32) -> bool {
            let mut can_place: bool = true;
            if val == 0 {
                can_place = true;
            } else if self.xy_grid.get(y).unwrap().iter().any(|x| x == &val) {
                can_place = false;
            } else {
                let mut in_row: bool = false;
                'row_loop: for i in 0..9 {
                    if self.xy_grid.get(i).unwrap().get(x).unwrap() == &val {
                        in_row = true;
                        break 'row_loop;
                    };
                }
                if !in_row {
                    let x_tmp: f64 = x.clone() as f64;
                    let y_tmp: f64 = y.clone() as f64;
                    let x_origin_pos = (((x_tmp / 3_f64).floor() as i32) * 3) as usize;
                    let y_origin_pos = (((y_tmp / 3_f64).floor() as i32) * 3) as usize;
                    'local_grid: for i in x_origin_pos..(x_origin_pos + 3) {
                        for j in y_origin_pos..(y_origin_pos + 3) {
                            if self.xy_grid.get(j).unwrap().get(i).unwrap() == &val {
                                can_place = false;
                                break 'local_grid;
                            }
                        }
                    }
                } else {
                    can_place = false;
                }
            }
            if can_place {
                self.xy_grid.get_mut(y).unwrap().push(val);
                self.xy_grid.get_mut(y).unwrap().swap_remove(x);
            }
            can_place
        }
        fn is_empty(&self, x: usize, y: usize) -> bool {
            self.xy_grid.get(y).unwrap().get(x).unwrap() == &0
        }
    }
    
    pub fn solve(val: Grid) -> Grid {
        let mut final_val = val;
        'outer: for j in 0..9 {
            for i in 0..9 {
                if final_val.is_empty(i, j) {
                    'inner: for k in 1..10 {
                        if final_val.place(i, j, k) {
                            let result = solve(final_val.clone());
                            if result.is_empty(0, 0) {
                                final_val.place(i, j, 0);
                            } else {
                                final_val = result;
                                break 'inner;
                            }
                        } else {
                            if k == 9 {
                                final_val = Grid::new(vec![vec![0; 9]; 9]);
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        final_val
    }

