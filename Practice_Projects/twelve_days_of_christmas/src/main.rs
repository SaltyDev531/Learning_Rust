static DAY_ONE: &str = "a partridge in a pear tree";
static DAY_TWO: &str = "Two turtle doves";
static DAY_THREE: &str = "Three French hens";
static DAY_FOUR: &str = "Four calling birds";
static DAY_FIVE: &str = "Five golden rings";
static DAY_SIX: &str = "Six geese a-laying";
static DAY_SEVEN: &str = "Seven swans a-swimming";
static DAY_EIGHT: &str = "Eight maids a-milking";
static DAY_NINE: &str = "Nine ladies dancing";
static DAY_TEN: &str = "Ten lords a-leaping";
static DAY_ELEVEN: &str = "Eleven pipers piping";
static DAY_TWELVE: &str = "Twelve drummers drumming";
static TRUE_LOVE: &str = "my true love sent to me";

fn main() {
    let gifts = [
        DAY_ONE, DAY_TWO, DAY_THREE, DAY_FOUR, DAY_FIVE, DAY_SIX, DAY_SEVEN, DAY_EIGHT, DAY_NINE,
        DAY_TEN, DAY_ELEVEN, DAY_TWELVE,
    ];

    let ordinals = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    for (day, ordinal) in ordinals.iter().enumerate() {
        println!("On the {} day of Christmas", ordinal);
        println!("{}", TRUE_LOVE);

        for gift in (0..=day).rev() {
            if gift == 0 && day > 0 {
                println!("And {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }

        println!();
    }
}
