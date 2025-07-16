fn main() {
    println!("Hello, world!");
    let vector1: Vec<i32> = Vec::from([1, 2, 3, 4, 5]);
    
    println!("Vector1: {:?}", vector1);
    vector1.iter().for_each(|x| println!("{}", x));

    println!("==========================");


    // let index = vector1.iter().position(|&r| r == 2).unwrap();
    // println!("index {}", index);

    println!("==========================");

    let result: Vec<i32> = two_sum(vector1, 3);
    println!("Result of two_sum: {:?}", result);

}


fn two_sum(nums: Vec<i32>, target: i32)-> Vec<i32>{
    for (index_x, x) in nums.iter().enumerate(){
        for (index_y, y) in nums.iter().enumerate(){
            // println!("y: {}", y);
            if x + y == target {
                if index_x == index_y {
                    continue; // Skip if both indices are the same
                }

                println!("Found a pair: {} + {} = {}", x, y, target);
                return vec![index_x.try_into().unwrap(), index_y.try_into().unwrap()];
            }
        }
    }

    vec![]
}
