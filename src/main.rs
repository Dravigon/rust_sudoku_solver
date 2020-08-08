
mod sudoku_grid;


pub fn main() {
    let test = vec![
        vec![0,0,0,9,0,0,0,0,0],
        vec![0,0,7,0,0,0,0,3,0],
        vec![2,8,0,6,0,0,0,0,0],
        vec![0,0,0,0,0,0,0,1,0],
        vec![6,0,0,0,0,4,0,0,0],
        vec![8,0,2,0,0,0,3,0,0],
        vec![0,3,0,0,0,2,0,0,0],
        vec![0,0,9,0,7,6,1,0,0],
        vec![0,0,0,0,9,0,7,0,0]
        ];

    let test2 =  vec![
        vec![0,0,0,9,0,0,0,0,0],
        vec![0,0,7,0,0,0,0,3,0],
        vec![2,8,1,6,3,7,4,9,5],
        vec![9,5,4,7,0,3,8,1,0],
        vec![6,1,3,2,8,4,9,5,7],
        vec![8,7,2,1,5,9,3,6,4],
        vec![7,3,6,8,1,2,5,4,9],
        vec![5,2,9,4,7,6,1,8,3],
        vec![1,4,8,3,9,5,7,2,0]
        ];

    let unsolved_sudoku = sudoku_grid::Grid::new(test);
    let solved_sudoku = sudoku_grid::solve(unsolved_sudoku);

    let mut j = 1;
    for solved_row in solved_sudoku.get_as_vec(){
        let mut i = 1;
  
        for val in solved_row{
            
            print!("{}",val);
            if i%3==0{
                print!("\t");    
            }
            i=i+1;
        }
        if j%3==0{
            print!("\n\n");    
        }else{
            print!("\n");    
        }
        j=j+1;
    }

    // println!("Solution : \n {:?}", );
}


// Solution:
// 3,6,5,9,4,8,2,7,1
// 4,9,7,5,2,1,6,3,8
// 2,8,1,6,3,7,4,9,5
// 9,5,4,7,6,3,8,1,2
// 6,1,3,2,8,4,9,5,7
// 8,7,2,1,5,9,3,6,4
// 7,3,6,8,1,2,5,4,9
// 5,2,9,4,7,6,1,8,3
// 1,4,8,3,9,5,7,2,6
