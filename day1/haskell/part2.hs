module Part2 where
import Part1 (numIncreases, input)

windows :: Num a => [a] -> [a]
windows [] = []
windows (x1:x2:x3:xs) = x1 + x2 + x3 : windows (x2:x3:xs)
windows _ = []

main :: IO Int
main = numIncreases . windows <$> input