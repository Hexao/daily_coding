mod easy {
    use daily_coding::easy::*;

    #[test]
    fn p001() {
        assert!(p001::solve(&[10, 15, 3, 7], 25));
        assert!(p001::solve(&[1, 3, 4, 0, 8, 9, 6, 7, 2, 5], 7));
        assert!(!p001::solve(&[1; 250], 14));
    }

    #[test]
    fn p008() {
        let root = daily_coding::root![1, 0, [[1, 1, 1], 0, 0]];
        assert_eq!(p008::count_unival_subtree(&root), 5);

        let root = daily_coding::root![[0, 1, 0], 2, [1, 0, 1]];
        assert_eq!(p008::count_unival_subtree(&root), 4);
    }

    #[test]
    fn p016() {
        use p016::Order;

        let mut order = Order::<3>::default();
        assert_eq!(order.get_last(0), None);

        order.record(10);
        assert_eq!(order.get_last(0), Some(10));
        assert_eq!(order.get_last(1), None);

        order.record(11);
        assert_eq!(order.get_last(0), Some(11));
        assert_eq!(order.get_last(1), Some(10));
        assert_eq!(order.get_last(2), None);

        for i in 12..20 {
            order.record(i);
            assert_eq!(order.get_last(0), Some(i));
            assert_eq!(order.get_last(1), Some(i - 1));
            assert_eq!(order.get_last(2), Some(i - 2));
            assert_eq!(order.get_last(3), None);
        }
    }
}

mod medium {
    use daily_coding::medium::*;

    #[test]
    fn p005() {
        use p005::{cons, car, cdr};

        assert_eq!(car(cons(5, 8)), 5);
        assert_eq!(cdr(cons(5, 8)), 8);

        assert_eq!(car(cons('a', 'j')), 'a');
        assert_eq!(cdr(cons('a', 'j')), 'j');
    }

    #[test]
    fn p007() {
        assert_eq!(p007::solve("1111"), 5);
        assert_eq!(p007::solve("0111"), 0);
        assert_eq!(p007::solve("1101"), 1);
        assert_eq!(p007::solve("1110"), 2);
        assert_eq!(p007::solve("2151410152118"), 60); // bonjour ?
    }

    #[test]
    fn p010() {
        use std::sync::{Arc, RwLock};
        const SLEEP: u64 = 250;
        const TARGET: i32 = 10;

        let arc = Arc::new(RwLock::new(0));
        let clone = arc.clone();

        let start = std::time::Instant::now();

        p010::call_after(move || {
            let mut val = clone.write().unwrap();
            *val = TARGET;
        }, SLEEP);

        assert!(start.elapsed().as_millis() >= SLEEP as u128);
        assert_eq!(*arc.read().unwrap(), TARGET);
    }

    #[test]
    fn p011() {
        const DICO: &[&str] = &["chien", "chevre", "chat", "lapin", "cheval"];

        let ch = p011::find_in(DICO, "ch");
        assert_eq!(ch, &["chien", "chevre", "chat", "cheval"]);

        let ch = p011::find_in(DICO, "che");
        assert_eq!(ch, &["chevre", "cheval"]);
    }

    #[test]
    fn p014() {
        let pi = p014::compute_pi();
        println!("{pi}");
        assert!((std::f64::consts::PI - pi).abs() < 0.00001);
    }

    #[test]
    fn p015() {
        const POOL: usize = 100_000;

        let data = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut dist = [0; 10];

        for _ in 0..POOL {
            dist[p015::pick_random(&data) as usize] += 1;
        }

        let var: Vec<_> = dist.iter().map(|el| {
            (el - (POOL  / 10) as i32).abs() as f32 / (POOL / 10) as f32
        }).collect();

        println!("{dist:5?}");
        println!("{var:.3?}");
        println!("{:.3?}", var.iter().fold(0.0, |f, &el| if el > f { el } else { f }));

        assert!(var.iter().all(|&el| el < 0.05));
    }
}

mod hard {
    use daily_coding::hard::*;

    #[test]
    fn p002() {
        assert_eq!(
            p002::solve(&[1, 2, 3, 4, 5]),
            &[120, 60, 40, 30, 24]
        );

        assert_eq!(
            p002::solve(&[4, 2, 8, 2, 4]),
            &[128, 256, 64, 256, 128]
        );
    }

    #[test]
    fn p004() {
        assert_eq!(p004::solve(vec![1, 2, 0]), 3);
        assert_eq!(p004::solve(vec![3, 4, -1, 1]), 2);
        assert_eq!(p004::solve(vec![-5, 1, 1, -9, 4, 0]), 2);
    }

    #[test]
    fn p006() {
        use p006::XorList;

        #[derive(Debug, PartialEq, Clone)]
        struct Trace(i32);
        impl Trace {
            fn new(val: i32) -> Self {
                println!("Create {}", val);
                Trace(val)
            }
        }
        impl Drop for Trace {
            fn drop(&mut self) {
                println!("Drop {}", self.0);
            }
        }

        let mut node = XorList::new(Trace::new(10));
        node.add(Trace::new(20));
        node.add(Trace::new(30));

        assert_eq!(node.get(0).unwrap().0, 10);
        assert_eq!(node.get(1).unwrap().0, 20);
        assert_eq!(node.get(2).unwrap().0, 30);
        assert_eq!(node.get(3), None);

        println!("End func");
    }

    #[test]
    fn p009() {
        assert_eq!(p009::solve(&[2, 4, 6, 2, 5]), 13);
        assert_eq!(p009::solve(&[5, 1, 1, 5]), 10);

        assert_eq!(p009::solve(&[4, 1, -4, 1, 4, 4]), 9);
        assert_eq!(p009::solve(&[-2, -1, 0, 1, 5, 4, 10]), 15);
    }

    #[test]
    fn p012() {
        assert_eq!(p012::solve_basic(4), 5);

        println!();
        assert_eq!(p012::solve_basic(5), 8);

        println!();
        assert_eq!(p012::solve_extended(10, &[1, 3, 5]), 47);
    }

    #[test]
    fn p013() {
        assert_eq!(p013::solve("abcba", 2), "bcb");
        assert_eq!(p013::solve("123456789", 1), "1");
        assert_eq!(p013::solve("barbapapa", 2), "apapa");
    }

    #[test]
    fn p017() {
        assert_eq!(p017::solve("root\n\tfolder\n\t\tfile.txt"), 20);
        assert_eq!(p017::solve("dir\n\tsubdir1\n\t\tfile1.ext\n\t\tsubsubdir1\n\tsubdir2\n\t\tsubsubdir2\n\t\t\tfile2.ext"), 32);
    }
}
