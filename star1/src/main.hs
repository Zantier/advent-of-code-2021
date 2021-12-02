main = do
    interact $ solve
    where
        solve s =
            let nums = map read . lines $ s :: [Int]
            in
                show . length . filter pred $ zip nums (tail nums)
        pred (a,b) = a < b
