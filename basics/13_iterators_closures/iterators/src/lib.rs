#[cfg(test)]
mod tests {
    #[test]
    fn iter() {
        let v1 = vec![1,2,3];

        // This is an iterator: a way to iter inside of
        // structs and vectors
        let v1_iter = v1.iter();
        for val in v1_iter {
            println!("Got {val}");
        }
    }
    
    #[test]
    fn iter_next() {
        let v1 = vec![1,2,3];
        let mut v1_iter = v1.iter();

        // iterators use different methods to do different 
        // things. next() goes to the next item in the list.
        // and returns a Options, where there is Some<T> until
        // None gets in.
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iter_sum() {
        let v1 = vec![1,2,3];
        let v1_iter = v1.iter();

        // the sum method sums every item in the iter
        let total: i32 = v1_iter.sum();
        assert_eq!(total, 6);

        // iters have a lot of different methods to use that
        // makes it really easy to create powerful one liners
        let total2: i32 = v1.iter().sum();
        assert_eq!(total2, 6);

        // usually to create something like this you will use
        // a for loop. That is verbose and prone to error. 
        // Iterators are just so convinient. Also they have the
        // same performace if not more than regular loops
    }

    #[test]
    fn iter_map() {
        // map is an iterator that produces another iterator
        let v1: Vec<i32> = vec![1,2,3];
        let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
        assert_eq!(v2, [2,3,4]);
    }
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: i32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: i32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod shoe_tests {
    use super::*;

    #[test]
    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneakers"),
            },
            Shoe {
                size: 11,
                style: String::from("stiletto"),
            },
            Shoe {
                size: 10,
                style: String::from("anvedi nando stile"),
            }
        ];
        let in_my_size = shoes_in_size(shoes, 10);
        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneakers"),
                },
                Shoe {
                    size: 10,
                    style: String::from("anvedi nando stile"),
                },
            ]            
        );
    }
}
