#[derive(Debug, Clone)]
pub struct Grid {
    xy_grid: Vec<Vec<i32>>,
}
impl Grid {
    ///Creates a new Grid instance from the provided Vec<Vec<i32>>
    pub fn new(val: Vec<Vec<i32>>) -> Grid {
        Grid { xy_grid: val }
    }
    ///Returns the grid as a Vec<Vec<i32>>
    pub fn get_as_vec(&self) -> Vec<Vec<i32>> {
        self.xy_grid.clone()
    }
    ///returns the value at the provided x,y coordinates
    fn get_value(&self, x: usize, y: usize) -> i32 {
        *(self.xy_grid.get(y).unwrap().get(x).unwrap())
    }
    ///returns the origin values of the subgrid for the provided x,y coordinates
    fn get_subgrid_origin_pos(&self, x: usize, y: usize) -> (usize, usize) {
        let x_origin_pos = x - x%3;
        let y_origin_pos = y - y%3;
        (x_origin_pos, y_origin_pos)
    }
    ///Checks if a value can be placed in a row and returns a boolean
    fn check_placement_row(&self, y: usize, val: i32) -> bool {
        !self.xy_grid.get(y).unwrap().iter().any(|x| x == &val)
    }
    ///Checks if a value can be placed in a column and returns a boolean
    fn check_placement_coloumn(&self, x: usize, val: i32) -> bool {
        let mut in_row: bool = false;
        'row_loop: for i in 0..9 {
            if self.get_value(x, i) == val {
                in_row = true;
                break 'row_loop;
            };
        }
        !in_row
    }
    ///Checks if a value can be placed in the subgrid of the provided coordinates
    /// and returns a boolean
    fn check_placement_subgrid(&self, x: usize, y: usize, val: i32) -> bool {
        let mut can_place: bool = true;
        let x_y_subrid_origin = self.get_subgrid_origin_pos(x, y);
        let x_origin_pos = x_y_subrid_origin.0;
        let y_origin_pos = x_y_subrid_origin.1;
        'local_grid: for i in x_origin_pos..(x_origin_pos + 3) {
            for j in y_origin_pos..(y_origin_pos + 3) {
                if self.get_value(i, j) == val {
                    can_place = false;
                    break 'local_grid;
                }
            }
        }
        can_place
    }
    ///Checks if a value can be placed in the given coordinates 
    ///and places it and returns a boolean
    fn place(&mut self, x: usize, y: usize, val: i32) -> bool {
        let can_place = self.check_placement_row(y, val)
            && self.check_placement_coloumn(x, val)
            && self.check_placement_subgrid(x, y, val);
        if can_place {
            self.xy_grid.get_mut(y).unwrap().push(val);
            self.xy_grid.get_mut(y).unwrap().swap_remove(x);
        }
        can_place
    }
    ///Empties the values(i.e places a zero) in the provided coordinates
    fn empty_value(&mut self, x: usize, y: usize) {
        self.xy_grid.get_mut(y).unwrap().push(0);
        self.xy_grid.get_mut(y).unwrap().swap_remove(x);
    }
    ///Checks if there is value in the provided coordinates
    fn is_empty(&self, x: usize, y: usize) -> bool {
        self.xy_grid.get(y).unwrap().get(x).unwrap() == &0
    }
}
///Solves the provided Grid and returns the solved grid
pub fn solve(val: Grid) -> Grid {
    let mut final_val = val;
    'outer: for j in 0..9 {
        for i in 0..9 {
            if final_val.is_empty(i, j) {
                'inner: for k in 1..10 {
                    if final_val.place(i, j, k) {
                        let result = solve(final_val.clone());
                        if result.is_empty(0, 0) {
                            final_val.empty_value(i, j);
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