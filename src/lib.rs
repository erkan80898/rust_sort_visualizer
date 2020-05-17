use piston_window::{PistonWindow, Event};
use plotters::prelude::{draw_piston_window, WHITE, IntoDrawingArea, ChartBuilder, IntoFont, Histogram, BLUE};
use plotters::style::Color;

/// This file contains varies sort implementations
/// All sorts written here take a lambda that will define
/// it's sorting order

/// Draws to the window
pub fn draw(window:&mut PistonWindow,vector:&Vec<i32>,min:i32,max:i32) -> Option<Event>{
    draw_piston_window(window, |b|{
        let root = b.into_drawing_area();
        root.fill(&WHITE)?;


        let mut cc = ChartBuilder::on(&root)
            .margin(10)
            .caption("Sorting Visualizer",("sans-serif", 30).into_font())
            .x_label_area_size(40)
            .y_label_area_size(40)
            .build_ranged(0..vector.len(),min..max)?;

        cc.configure_mesh()
            .x_label_formatter(&|x| format!("{}",x))
            .y_label_formatter(&|y|format!("{}",y))
            .x_labels(10)
            .y_labels(10)
            .x_desc("Index")
            .y_desc("Value")
            .axis_desc_style(("sans-serif", 15).into_font())
            .draw()?;

        cc.draw_series(Histogram::vertical(&cc)
            .style(BLUE.filled())
            .data(vector.iter().enumerate().map(|x|(x.0,*x.1))))?;
        Ok(())
    })
}

///INSERTION START
pub fn insertion_sort<F>(array: &mut Vec<i32>,f: Box<F>, window:&mut PistonWindow,min:i32,max:i32)
where F: ?Sized + Fn(&i32,&i32) -> bool
{
    for i in 1..array.len() {
        let mut j = i;
        while j > 0 && f(&array[j],&array[j-1]){
            draw(window,array,min,max);
            array.swap(j, j-1);
            j = j-1;
        }
    }
}
//INSERTION END

///SELECTION START
pub fn selection_sort<F>(array: &mut Vec<i32>,f: Box<F>,
                      window:&mut PistonWindow,min:i32,max:i32)
where F: ?Sized + Fn(&i32,&i32)->bool
{

    let mut minl;
    for i in 0..array.len() {
        minl = i;
        for j in (i+1)..array.len() {

            if f(&array[j],&array[minl]) {
                minl = j;
            }
        }
        let tmp = array[i];
        array[i] = array[minl];
        array[minl] = tmp;
        draw(window,&array,min,max);
    }
}
//SELECTION END

///QUICK START
pub fn quick_sort<F>(array: &mut Vec<i32>, l:i32, h:i32, f: &Box<F>,
                     window:&mut PistonWindow,min:i32,max:i32)
    where F: ?Sized + Fn(&i32, &i32) -> bool
{
    if h-l >= 1 {
        let pivot = partition_quick(array,l,h,&f,window,min,max) as i32;
        quick_sort(array, l, (pivot-1) as i32, f, window,min,max);
        quick_sort(array, (pivot + 1) as i32, h, f, window,min,max);
    }
}

fn partition_quick<F>(array: &mut Vec<i32>,l:i32,h:i32,f: &Box<F>,
                      window:&mut PistonWindow,min:i32,max:i32) -> usize
    where F: ?Sized + Fn(&i32, &i32) -> bool
{
    let pivot = (h-l)/2 + l;
    array.swap(pivot as usize, h as usize);
    let mut switch_index: usize = l as usize;

    for i in l..h+1 {
        if f(&array[i as usize], &array[h as usize]) {
            array.swap(i as usize, switch_index);
            draw(window,&array,min,max);
            switch_index += 1;
        }

    }
    draw(window,&array,min,max);
    array.swap(switch_index, h as usize);
    return switch_index;
}
//QUICK END

///MERGE START
fn merge<F>(array:&mut Vec<i32>,l1:i32, h1:i32,l2:i32,h2:i32,
            f: &Box<F>,y: &mut Vec<i32>,window:&mut PistonWindow,min:i32,max:i32)
where F: ?Sized + Fn(&i32, &i32) -> bool
{

    let mut i = l1;
    let mut j = l2;
    let mut k = l1 as usize;
    let list1_len = h1-l1;
    let list2_len = h2-l2;
    while i < list1_len && j < list2_len {
        if f(&array[i as usize],&array[j as usize]){
            y[k] = array[i as usize];
            k += 1;
            i += 1;
        } else {
            y[k] = array[j as usize];
            k += 1;
            j += 1;
        }
        draw(window,y,min,max);
    }
    if i < list1_len {
        y[k..].copy_from_slice(&array[i as usize..]);
    }
    if j < list2_len {
        y[k..].copy_from_slice(&array[j as usize..]);
    }

    draw(window,&y.to_vec(),min,max);
}

pub fn merge_sort_rec<F>(array: &mut Vec<i32>, l:i32, h:i32, f: &Box<F>, window:&mut PistonWindow, min:i32, max:i32)
    where F: ?Sized + Fn(&i32, &i32) -> bool
{
    let length = h-l;
    let m = length / 2;
    if length <= 0 {
        draw(window,&array.to_vec(),min,max);
        return;
    }

    merge_sort_rec(array,l,m,&f,window,min,max);
    merge_sort_rec(array,m+1,h,&f,window,min,max);

    let mut y = array.clone();
    merge(array,l, m,m+1,h, f,&mut y,window,min,max);

    *array = y.to_vec();
}
//MERGE END
