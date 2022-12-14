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
        monkey_in_the_middle => (Some(95472), Some(17926061332u64)),
        hill_climbing_algorithm => (Some(330), Some(321)),
        distress_signal => (6428, 22464),
        regolith_reservoir => (897, 26683),
        beacon_exclusion_zone => (5144286, Some(10229191267339u64)),
        proboscidea_volcanium => (1737, 2216),
        // TODO: Figure out why the answer is off by 1,
        // it should be 1..2*7*
        pyroclastic_flow => (3137, Some(1564705882328u64)),
        boiling_boulders => (4536, 2606)
    );
}
