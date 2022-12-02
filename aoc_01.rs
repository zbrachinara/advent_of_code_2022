
#[derive(Debug)]
struct Maxima<const N: usize>([u32; N]);

impl <const N: usize> Maxima<N> {
    fn zeroed() -> Self {
        Self([0; N])
    }
    fn trickle(&mut self, extra: u32) {
        if let Some(replace_ix) = self.0.iter().enumerate().find(|&(_, &e)| extra > e).map(|(i, _)| i) {
            self.0[replace_ix..].rotate_right(1);
            self.0[replace_ix] = extra;
        }
    }
    fn sum(self) -> u32 {
        self.0.into_iter().sum()
    }
}

fn main() {

    let out = std::fs::read_to_string("aoc").unwrap().split("\n\n")
        .map(|pack| pack.split("\n").filter_map(|u| str::parse::<u32>(u).ok()).sum::<u32>())
        .fold(Maxima::<3>::zeroed(), |mut maxima, e| {
            maxima.trickle(e);
            maxima
        })
        .sum();

    println!("result: {out:?}");

}
