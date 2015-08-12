use std::collections::vec;

pub fn groupConsecutive (xs: Iterator<i32>) -> Vec<Vec<i32>> {
	xs.foldBack (|x, acc| {
        match (acc, x) {
        	([], _) => [[x]]
            ([h, t, rest], x) if h - x <= 1 => { [x, h, t, rest] }
            (acc, x) => ([[x], acc], xs)
		}
	})
}

//[1; 2; 4; 5; 6; 9] |> groupConsecutive
