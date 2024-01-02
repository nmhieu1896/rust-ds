#[allow(dead_code)]
#[derive(Debug)]
pub struct FenwickTree {
    arr: Vec<i32>,
    prefix_arr: FenwickArr,
}

#[derive(Debug)]
pub struct FenwickArr(Vec<i32>);
#[allow(dead_code)]
impl FenwickArr {
    pub fn sum_at(&self, idx: usize) -> i32 {
        // sum by the binary representation of idx
        // for example, sum_at idx 7 (111) is sum of idx 7(111) + idx 6(110) + idx 4(100)
        let mut sum = 0;
        let mut from = idx;
        loop {
            if from == 0 {
                break;
            }
            sum += self.0[from];
            // switch off the rightmost 1 bit
            from = from & from.wrapping_sub(1)
        }
        return sum;
    }
}

#[allow(dead_code)]
impl FenwickTree {
    pub fn new(origin_arr: Vec<i32>) -> FenwickTree {
        let mut prefix_arr: FenwickArr = FenwickArr(vec![0; origin_arr.len() + 1]);

        for (idx, item) in origin_arr.iter().enumerate() {
            let tree_idx = idx + 1;
            let count = tree_idx.count_ones();

            // If the tree_idx is odd, then it is a beginning of a new layer.
            prefix_arr.0[tree_idx] = if tree_idx & 1 != 0 {
                item.clone()
            } else if count == 1 {
                // If it belong to the first layer,
                // then it is the sum of all the last items of all the layers. plus current item of origin_arr.
                prefix_arr.sum_at(tree_idx - 1) + item.clone()
            } else {
                // else, it is the sum of previous_prefix and current item of origin_arr.
                prefix_arr.0[tree_idx - 1] + item.clone()
            }
        }

        return FenwickTree {
            arr: origin_arr.clone(),
            prefix_arr: prefix_arr,
        };
    }
}

pub fn _run() {
    let arr = vec![3, 2, -1, 6, 5, 4, -3, 3, 7, 2, 3];

    let sum_arr = arr.iter().fold(vec![], |mut acc, item| {
        acc.push(acc.last().unwrap_or(&0) + item);
        return acc;
    });

    let tree = FenwickTree::new(arr);
    println!("origin :      {:?}", tree.arr);
    println!("sum_ar :      {:?}", sum_arr);
    println!("{:?}", tree.prefix_arr);

    let idx_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];
    idx_vec.iter().for_each(|idx| {
        assert!(tree.prefix_arr.sum_at(*idx) == sum_arr[*idx - 1]);
    });
}
