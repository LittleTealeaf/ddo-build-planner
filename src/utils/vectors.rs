#[macro_export]
macro_rules! flatten_vecs {
    ($($vec: expr),*) => {
        {
            let vecs = [$($vec),*];
            let mut size = 0;
            for vec in vecs {
                size += vec.len();
            }
            let mut combined_vec = Vec::with_capacity(size);
            for vec in vecs {
                for item in vec {
                    combined_vec.push(item);
                }
            }
        } 
    };
}
