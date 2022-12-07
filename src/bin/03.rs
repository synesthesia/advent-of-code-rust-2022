
mod item {
    #[repr(transparent)]
    #[derive(Copy, Clone)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item{
        type Error = String;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(format!("{} is not a valid item",value as char)),
            }
        }
    }

    impl std::fmt::Debug for Item {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0 as char)
        }
    }

    impl Item {
        pub(crate) fn priority(self) -> usize {
            match self {
                Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
                Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
                _ => unreachable!(),
            }
        }
    }
}


pub fn part_one(input: &str) -> Option<u32> {

    use item::Item;

    let mut total:u32 = 0;

    for line in input.lines(){
        let (first, second) = line.split_at(line.len() / 2);

        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>();

        let second_items = second
            .bytes()
            .map(Item::try_from)
            .collect::<Result<Vec<_>, _>>();

        println!("- {first_items:?} | {second_items:?}");

    }
    if total == 0 {None} else {Some(total)}
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {

    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;
    use item::Item;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_item(){
        let _a = Item::try_from(b'a');
        println!("{_a:?}");
        let _exclaim = Item::try_from(b'!');
        println!("{_exclaim:?}");
    }

    fn test_item_can_be_copied(){
        let a = Item::try_from(b'a');
        a.priority();
        //a.priority();
    }

    /*
    fn test_item_constructor_protected(){

        //let _a = Item(b'a');
        // uncommenting line above shold give a compiler error
    }
    */


}
