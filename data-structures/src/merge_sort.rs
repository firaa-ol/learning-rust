fn merge_sort(arr: Vec<u32>) -> Vec<u32> {
    if arr.len() == 1 {
        return arr;
    }

    let arr_1 = arr[0..arr.len() / 2].to_vec();
    let arr_2 = arr[(arr.len() / 2)..arr.len()].to_vec();

    let arr_1 = merge_sort(arr_1);
    let arr_2 = merge_sort(arr_2);

    return merge(arr_1, arr_2);
}

fn merge(mut arr1: Vec<u32>, mut arr2: Vec<u32>) -> Vec<u32> {
    let mut arr3: Vec<u32> = vec![];
    while arr1.len() > 0 && arr2.len() > 0 {
        if arr1[0] > arr2[0] {
            arr3.push(arr2[0]);
            arr2.remove(0);
        } else {
            arr3.push(arr1[0]);
            arr1.remove(0);
        }
    }

    // at this point arr1 or arr2 are empty
    if arr1.len() != 0 {
        arr3.append(&mut arr1);
    }

    if arr2.len() != 0 {
        arr3.append(&mut arr2);
    }

    arr3
}

#[cfg(test)]
mod tests {
    use crate::merge_sort::merge_sort;

    #[test]
    fn merge_sort_works() {
        let data = vec![78, 16, 86, 0, 45, 63, 27];

        assert_eq!(merge_sort(data), vec![0, 16, 27, 45, 63, 78, 86]);
    }
}
