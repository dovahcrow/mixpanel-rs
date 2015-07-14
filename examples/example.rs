extern crate mixpanel;

use mixpanel::*;


fn main() {
    let mx = MixPanel::new("9a9d9ad3bc6c53b3f9ef968e5c206b87", "81407ce7c918edd53f1f4736ddbf3fe9");
    let ret = mx.export("2015-07-01", "2015-07-04").send();
    println!("{:?}", ret);
}
