fn main(){
//fn abbrev_name(name: &str){
    let name: &str = "Sam Harris";
    let a = &mut name.split_whitespace();
    let z = a.into_iter().nth(0).unwrap().chars().nth(0).unwrap();
    let w = a.into_iter().nth(1).unwrap().chars().nth(0).unwrap();
    println!("{}.{}", z,w);

    //let name = name.to_uppercase().to_string();
    //let a: Vec<_> = name.split_whitespace().collect();
    //for i in
        /*
        let mut x = name.split(" ");
        let w = x.nth(0).unwrap().to_uppercase();
        let z = x.nth(1).unwrap().to_uppercase();
    */
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sample_tests() {
        assert_eq!(abbrev_name("Sam Harris"), "S.H");
        assert_eq!(abbrev_name("Patrick Feenan"), "P.F");
        assert_eq!(abbrev_name("Evan Cole"), "E.C");
        assert_eq!(abbrev_name("P Favuzzi"), "P.F");
        assert_eq!(abbrev_name("David Mendieta"), "D.M");
    }
}
*/