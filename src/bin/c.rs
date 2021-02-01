use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        ab: [(usize, usize);m],
        k: usize,
        cd: [(usize, usize);k],
    }

    let mut ans =0;

    for bit in 0..1 << k{
        let mut dish = vec![false; n+1];
        for j in 0..k{
            if bit & (1<<j) == 0 {
                dish[cd[j].0] = true
            }
            else {
                dish[cd[j].1] = true
            }
        }

        let mut satisfied = 0;
        for i in 0..m{
            if dish[ab[i].0] && dish[ab[i].1] {
                satisfied += 1
            }
        }
        ans = std::cmp::max(ans, satisfied)
    }

    println!("{}",ans)
}
