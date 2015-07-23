extern crate mixpanel;

use mixpanel::*;


fn main() {
    let mx = MixPanel::new("", "");
    let ret = mx.export("2015-07-01", "2015-07-04").send();
    println!("{:?}", ret);
}
