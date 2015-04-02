use std::collections::vec;

pub fn groupConsecutive (xs: Iter<i32>) -> Vec<Vec<i32>> {
	xs.fi
    List.foldBack (fun x acc -> 
            match acc, x with
            | [], _ -> [[x]]
            | (h :: t) :: rest, x when h - x <= 1 -> (x :: h :: t) :: rest
            | acc, x -> [x] :: acc) xs []
}
 
//[1; 2; 4; 5; 6; 9] |> groupConsecutive