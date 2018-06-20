pub fn factors(n: u64) -> Vec<u64> {
    let mut root: u64 = (n as f64).sqrt().round() as u64;
    let mut workn = n.clone();
    let mut fs: Vec<u64> = vec![];
    while workn != 1 {
        if root == 1 {
            fs.push(workn);
            break;
        }
        if workn % root == 0 && is_prime(root){
            fs.push(root);
            workn /= root;
        } else {
            root -= 1; 
        }
    }
    fs.sort();
    fs
}

fn is_prime(n: u64) -> bool {
    let end: u64 = (n as f64).sqrt().round() as u64;
    for i in 2..end {
        if n % i == 0 {
            return false; 
        } 
    } 
    return true;
}



