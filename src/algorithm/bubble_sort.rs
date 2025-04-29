pub fn bubble_sort(nums: &mut Vec<i32>) {
    let mut len = nums.len() - 1;

    while len > 0 {
        for i in 0..len {
            if nums[i] > nums[i + 1] {
                nums.swap(i, i + 1);
            }
        }
        len -= 1;
    }
}
