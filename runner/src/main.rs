const DAY_9_PART_2: &str = "
####.#....###..#....####..##..####.#....
#....#....#..#.#.......#.#..#....#.#....
###..#....#..#.#......#..#......#..#....
#....#....###..#.....#...#.##..#...#....
#....#....#....#....#....#..#.#....#....
####.####.#....####.####..###.####.####.
";

fn main() {
    runner::days!(
        calorie_counting => (Some(68467), Some(203420)),
        rock_paper_scissors => (11475, 16862),
        rucksack_reorganization => (8139, 2668),
        camp_cleanup => (471, 888),
        supply_stacks => (String::from("VCTFTJQCG"), String::from("GCFGLDNJZ")),
        tuning_trouble => (Some(1760), Some(2974)),
        no_space_left_on_device => (1444896, Some(404395)),
        treetop_tree_house => (1690, Some(535680)),
        rope_bridge => (5930, 2443),
        cathode_ray_tube => (14780, DAY_9_PART_2.trim() /* ELPLZGZL */),
        monkey_in_the_middle => (Some(95472), Some(17926061332u64))
    );
}
