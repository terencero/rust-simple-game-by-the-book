pub fn print_the_lyrics_to_the_twelve_days_of_christmas() {
    struct Lyrics {
        days: [String; 12],
        presents: [String; 11],
    }

    impl Lyrics {
        fn get_days(&self, index: usize) -> &String {
            &self.days[index]
        }

        fn get_presents(&self, index: usize) -> &String {
            &self.presents[index]
        }
    }

    let lyrics = Lyrics {
        days: [
            String::from("first"),
            String::from("second"),
            String::from("third"),
            String::from("fourth"),
            String::from("fifth"),
            String::from("sixth"),
            String::from("seventh"),
            String::from("eighth"),
            String::from("ninth"),
            String::from("tenth"),
            String::from("eleventh"),
            String::from("twelfth"),
        ],
        presents: [
            String::from("Two turtle doves"),
            String::from("Three french hens"),
            String::from("Four calling birds"),
            String::from("Five golden rings"),
            String::from("Six geese a-laying"),
            String::from("Seven swans a-swimming"),
            String::from("Eight maids a-milking"),
            String::from("Nine ladies dancing"),
            String::from("Ten lords a-leaping"),
            String::from("Eleven pipers piping"),
            String::from("Twelve drummers drumming"),
        ],
    };
    for day in 0..12 {
        let chorus_line = lyrics.get_days(day);
        println!("On the {} day of Christmas, my true love sent to me", chorus_line);

        if day > 0 {
            for element in 0..day {
                let next_line = lyrics.get_presents(element);
                let and = if element + 1 == day { ", and" } else { "" };
                println!("{next_line}{and}");
            };
        }

        println!("A partridge in a pear tree");
        println!("");
    };
}
