fn vec_sum(vec: &[u32]) -> Option<u32> {
    let mut list_iter = vec.iter();
    list_iter.try_fold(0u32, |sum, &list_iter| sum.checked_add(list_iter))
}

fn main() {
    let normal_list = [1, 2, 3];
    let overflow_list = [1, 5, 4294967290];
    let normal_sum = vec_sum(&normal_list);
    let overflow_sum = vec_sum(&overflow_list);

    match normal_sum {
        Some(i)=>println!("normal sum return: {i}"),
        None =>{
            println!("None")
        }
    }

    match overflow_sum {
        Some(i)=>println!("{i}"),
        None =>{
            println!("overflow sum return: None")
        }
    }
    
}
