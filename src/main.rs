use std::io;
fn main() {

    println!("Enter the box num");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let box_num:i32 =input.trim().parse().unwrap();
 
    let mut golds = 1;
    let mut boxes = 1;
    let mut total_gold = 0;
    
    for gold in 1..box_num {
        if boxes ==1 {
            golds == 1*gold;
            total_gold = total_gold+golds;
            boxes+=1;
        }
             golds = 2*gold; 
             total_gold = total_gold + golds;
                
    } 

    println!("Total golds on your box {}", total_gold);

    println!("Do you want to know your Total gold? say yes or no");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let input:String =input.trim().parse().unwrap();
    
    if input == "yes" {

    let total_boxes = 92;
    let mut boxes = 2;

for gold in 1..total_boxes {
    if boxes == 2 {
        golds == 1*gold;
        total_gold = total_gold+golds;
        boxes+=1;
    }
         golds = 2*gold; 
         total_gold = total_gold + golds;
            
} 
    
    println!("Total golds you have  {}", total_gold);
}

}
