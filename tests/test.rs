mod easy {
    use daily_coding::easy::*;

    #[test]
    fn p001() {
        assert!(p001::solve(&[10, 15, 3, 7], 25));
        assert!(p001::solve(&[1, 3, 4, 0, 8, 9, 6, 7, 2, 5], 7));
        assert!(!p001::solve(&[1; 250], 14));
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
        assert_eq!(p007::solve("2151410152118"), 60); // bonjour ?
    }
}

mod hard {
    use daily_coding::hard::*;

    #[test]
    fn p002() {
        assert_eq!(
            p002::solve(&[1, 2, 3, 4, 5]),
            vec![120, 60, 40, 30, 24]
        );

        assert_eq!(
            p002::solve(&[4, 2, 8, 2, 4]),
            vec![128, 256, 64, 256, 128]
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
}
