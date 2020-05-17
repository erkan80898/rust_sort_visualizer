use piston_window::{PistonWindow, WindowSettings,Window};
use rand::Rng;
use sort_algo_visualizer::*;
use std::io;
use piston_window::Input as InputClose;
use piston_window::Event::Input;
use std::time::Duration;

const SORTS:&str = "Insertion, Selection, Quick, Merge";

///Enum that represents user selection
enum Sort{
    Insertion,
    Selection,
    Quick,
    Merge
}

impl Sort{

    fn new(sort:&str) -> Result<Sort,String>{
        match &sort.to_lowercase()[..]{
            "insertion" => Ok(Sort::Insertion),
            "selection" => Ok(Sort::Selection),
            "quick" => Ok(Sort::Quick),
            "merge" => Ok(Sort::Merge),
            _ => Err(String::from("No such sort found"))
        }
    }
}

///CLI processor
fn cli_helper<'q>(len:&mut u32, min:&mut i32, max:&mut i32) -> Result<(Sort,&'q dyn Fn(&i32, &i32) -> bool),String>{

    let mut input = String::new();

    println!("Input the desired array size to sort!");
    match io::stdin().read_line(&mut input){
        Ok(_) =>
            match input.trim().parse::<u32>(){
                Ok(length) => *len = length,
                Err(e) => {
                    return Err(format!("Input invalid: {}", e))
                }
            },
        Err(e) =>{
            return Err(format!("Issue reading input: {}", e))
        }
    }
    input.clear();

    println!("Input the min value contained in the array!");
    match io::stdin().read_line(&mut input){
        Ok(_) =>
            match input.trim().parse::<i32>(){
                Ok(min_val) => {*min = min_val;},
                Err(e) => {
                    return Err(format!("Input invalid: {}", e))
                }
            },
        Err(e) =>{
            return Err(format!("Issue reading input: {}", e))
        }
    }
    input.clear();

    println!("Input the max value contained in the array!");
    match io::stdin().read_line(&mut input){
        Ok(_) =>
            match input.trim().parse::<i32>(){
                Ok(max_val) => {
                    if *min > max_val{
                        return Err(format!("Min was greater than max!"));
                    }
                    *max = max_val;
                },
                Err(e) => {
                    return Err(format!("Input invalid: {}",e));
                }
            },
        Err(e) =>{
            return Err(format!("Issue reading the input: {}",e));
        }
    }

    println!("Select your sort or enter \"help\" to \
    view all available sorts");
    let sort;
    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match &input.trim().to_lowercase()[..]{
                     "help" => {
                        println!("{}", SORTS);
                        continue;
                    }
                    str => {
                        match Sort::new(str){
                            Ok(x) =>{
                                sort = x;
                                break;
                            }
                            Err(e) => {
                                return Err(format!("{}", e))
                            }
                        }
                    }
                }
            }
            Err(e) => {
                return Err(format!("Issue reading input: {}", e))
            }
        }
    }
    let f:Box<&dyn Fn(&i32, &i32) -> bool>;
    loop {
        input.clear();
        println!("Select ordering: \"Ascending\" or \"Descending\"");
        match io::stdin().read_line(&mut input) {
            Ok(_) =>
                match &input.trim().to_lowercase()[..] {
                    "ascending" => {
                        f = Box::new(&|&x, &y| x < y);
                        break;
                    }
                    "descending" => {
                        f = Box::new(&|&x, &y| x > y);
                        break;
                    }
                    _ => {
                        println!("Invalid ordering");
                        continue;
                    }
                },
            Err(e) => {
                return Err(format!("Issue reading input: {}", e))
            }
        }
    }

    return Ok((Sort::Quick,*f));
}

fn main() {
    println!("Welcome to sort visualizer!");
    let mut len:u32 = 0;
    let mut min= 0;
    let mut max= 0;

    loop {
        let mut rng = rand::thread_rng();

        let f;
        let sort;
        let result = cli_helper(&mut len,&mut min,&mut max);

        match result{
            Ok(data) => {
                sort = data.0;
                f = Box::new(data.1);
            }
            Err(e) => {
                println!("{}",e);
                continue;
            }
        }

        let mut vector: Vec<i32> = (0..len).map(|_| rng.gen_range(min, max)).collect();

        let mut window: PistonWindow = WindowSettings::new
            ("Sorting Visualizer", [800, 800]).samples(4)
            .build()
            .unwrap();

        let len = (vector.len()-1) as i32 ;
        match sort{
            Sort::Insertion => insertion_sort(&mut vector,f,&mut window,min,max),
            Sort::Selection => selection_sort(&mut vector,f,&mut window,min,max),
            Sort::Quick => quick_sort(&mut vector, 0, len, &f, &mut window,min,max),
            Sort::Merge => merge_sort_rec(&mut vector,0,len, &f, &mut window,min,max),
        }
        loop {
            draw(&mut window,&vector,min,max);
            match window.wait_event_timeout(Duration::from_millis(500)) {
                Some(Input(InputClose::Close(_),_)) => {break},
                _ => continue,
            }
        }
    }
}


