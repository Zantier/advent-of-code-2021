import Data.List

main = do
    input <- getContents
    let nums = map read . lines $ input :: [Int]
    print $ solve1 nums
    print $ solve2 nums

solve1 :: [Int] -> Int
solve1 nums =
    length . filter predicate $ zip nums (tail nums)

solve2 :: [Int] -> Int
solve2 nums =
    let nums' = map sum $ windows 3 nums
    in
        solve1 nums'

predicate (a,b) = a < b

windows :: Int -> [a] -> [[a]]
windows n xs =
    dropTail n . map (take n) $ tails xs

-- Drop n elements from the tail of the list
dropTail :: Int -> [a] -> [a]
dropTail n lst =
    take (length lst - n) lst
