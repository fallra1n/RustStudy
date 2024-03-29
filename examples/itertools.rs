use itertools::ExtendedIterator;

fn main() {
    println!("[TEE]");
    {
        let data = vec![1, 2];
        let base = data.iter();
        let (mut f, mut s) = base.tee();
        println!("{:?} === 1", f.next().unwrap());
        println!("{:?} === 1", s.next().unwrap());
        println!("{:?} === 2", f.next().unwrap());
        println!("{:?} === 2", s.next().unwrap());
    }

    println!("\n[LAZE CYCLE]");
    {
        let data = vec![1, 2, 5];
        let base = data.iter();
        let mut lc = base.lazy_cycle();
        println!("{:?} === 1", lc.next().unwrap());
        println!("{:?} === 2", lc.next().unwrap());
        println!("{:?} === 5", lc.next().unwrap());
        println!("{:?} === 1", lc.next().unwrap());
        println!("{:?} === 2", lc.next().unwrap());
    }

    println!("\n[EXTRACT]");
    {
        let data = vec![1, 2, 3, 4, 5];
        let base = data.iter();
        let mut exd = base.clone().extract(2);
        println!("{:?} === 1", exd.1.next().unwrap());
        println!("{:?} === 2", exd.1.next().unwrap());
        println!("{:?} === 4", exd.1.next().unwrap());
        println!("{:?} === 5", exd.1.next().unwrap());
    }

    println!("\n[GROUP BY]");
    {
        let data = vec![1, 2, 3, 4, 5];
        let mut base = data.iter();
        let mut gb = base.clone().group_by(|x| {
            if **x < 3 {
                return 10000;
            }

            return 5000;
        });

        println!("{:?} === 10000 : 1,2", gb.next().unwrap());
        println!("{:?} === 5000 : 3, 4, 5", gb.next().unwrap());
    }
}
