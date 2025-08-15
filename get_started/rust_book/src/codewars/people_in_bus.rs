pub fn number(bus_stops:&[(i32, i32)]) -> i32 {
    // let mut passagiers = 0; 
    // let mut getting_off = 0;
    // bus_stops.iter().for_each(|el| {
    //     println!("{el:?}");
    //     passagiers += el.0;
    //     getting_off += el.1;
    // }); 
    // passagiers - getting_off
    bus_stops.iter().map(|el| el.0 - el.1).sum()
}

#[test]
fn returns_expected() {
  assert_eq!(number(&[(10,0),(3,5),(5,8)]), 5);
  assert_eq!(number(&[(3,0),(9,1),(4,10),(12,2),(6,1),(7,10)]), 17);
  assert_eq!(number(&[(3,0),(9,1),(4,8),(12,2),(6,1),(7,8)]), 21);
}
