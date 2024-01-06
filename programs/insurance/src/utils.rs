pub fn remove_from_vector<T: PartialEq>(
    element: T,
    vector: &mut Vec<T>,
) -> Result<(), &'static str> {
    if let Some(index) = vector.iter().position(|x| *x == element) {
        vector.remove(index);
        Ok(())
    } else {
        Err("Element not found in the vector")
    }
}
