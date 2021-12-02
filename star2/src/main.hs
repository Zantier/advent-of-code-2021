import Data.List

main = do
    interact $ solve
    where
        solve s =
            let nums = map read . lines $ s :: [Int]
                nums' = map sum $ windows 3 nums
            in
                show . length . filter pred $ zip nums' (tail nums')
        pred (a,b) = a < b

windows :: Int -> [a] -> [[a]]
windows n xs =
    dropTail n . map (take n) $ tails xs

-- Drop n elements from the tail of the list
dropTail :: Int -> [a] -> [a]
dropTail n lst =
    take (length lst - n) lst
