fn main() {
    let mut x:[i32;9]=[1,1,2,3,4,1,3,4,5];
    let mut sum = 0;
    for i in 0..x.len() {
        let mut flag = 0;
        for j in i+1..x.len() {
            if x[i]==x[j] {
                flag+=1;
                x[j]=0;
            }
        }
        if flag!=0 {sum = sum + x[i];}
    }
    println!("{}", sum);
}
