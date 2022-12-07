
mod item {
    #[repr(transparent)]
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub(crate) struct Item(u8);

    impl TryFrom<u8> for Item{
        type Error = color_eyre::Report;
        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
                _ => Err(color_eyre::eyre::eyre!("{} is not a valid item",value as char)),
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
    use std::collections::HashSet;
    use itertools;

    let mut total:u32 = 0;

    for line in input.lines(){
        let (first, second) = line.split_at(line.len() / 2);

        let first_items = first
            .bytes()
            .map(Item::try_from)
            .collect::<Result<HashSet<_>, _>>().ok();

            itertools::process_results(second.bytes().map(Item::try_from), |mut it| {
                it.find(|&item| first_items.contains(&item))
                //    .map(|item| dbg!(item.priority()))
                //    .ok_or_else(|| color_eyre::eyre::eyre!("compartments have no items in common"))
            })//?


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

    #[test]
    fn test_item_can_be_copied(){
        let a = Item::try_from(b'a').unwrap();
        a.priority();
        a.priority();
    }

    /*
    fn test_item_constructor_protected(){

        //let _a = Item(b'a');
        // uncommenting line above shold give a compiler error
    }
    */


}
