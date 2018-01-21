// Solver for the coin enigma
// _ + _ * _^2 + _^3 - _ = 399
// red coin: 2
// corroded : 3
// shiny: 5
// concave: 7
// blue coin: 9

fn main() {
    let values = [2,3,5,7,9];
    for i in 0..(5*4*3*2) {
        try_partition(&values, i);
    }
}

fn try_partition(values: &[usize], mut n: usize) {
    let mut pos = [n%5;5];
    for i in 1..5 {
        n = n/(5-i+1);
        pos[i] = (n)%(5-i);
        for j in 0..i {
            if pos[j] <= pos[i] {
                pos[i] += 1;
            }
        }
    }
    if 399 == values[pos[0]] + values[pos[1]]*values[pos[2]].pow(2) + values[pos[3]].pow(3) - values[pos[4]]{
        let mut out = "".to_string();
        for &p in pos.iter() {
            out += match p {
                0 => "red ",
                1 => "corroded ",
                2 => "shiny ",
                3 => "concave ",
                4 => "blue ",
                _ => ""
            };
        }
        println!("{}", out);
    }
}
