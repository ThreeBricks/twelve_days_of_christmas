fn main() {
    let days = ["First","Second","Third","Fourth","Fifth","Sixth","Seventh","Eight","Ninth","Tenth","Eleventh","Twelfth"];
    let gifts = ["A partridge in a pear tree","Two turtle doves, and","Three french hens","Four calling birds","Five golden rings","Six geese-a-laying","Seven swans a-swimming","Eight maids a-milking","Nine ladies dancing","Ten lords-a-leaping","Eleven pipers piping","Twelve drummers drumming"];

    for day_num in 0..12 {
        println!("On the {  } day of Christmas, my true love sent to me", days[day_num]);
        for gift_num in (0..day_num+1).rev(){
            println!("{}",gifts[gift_num]);
        }
        println!("");
    }
}
