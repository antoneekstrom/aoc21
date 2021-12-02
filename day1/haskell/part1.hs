module Part1 where

input :: IO [Int]
input = map read . lines <$> readFile "input"

numIncreases :: (Ord a, Num a) => [a] -> Int
numIncreases xs = length . filter (== True) $ zipWith (>) xs (head xs + 1 : xs)

main :: IO Int
main = numIncreases <$> input