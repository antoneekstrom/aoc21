module Part2 where

import Part1

data State = State Int Int Int

instance Submarine State where
  depth (State _ _ depth) = depth
  distance (State _ distance _) = distance
  apply (Instruction Down n) (State aim pos depth) = State (aim + n) pos depth
  apply (Instruction Up n) (State aim pos depth) = State (aim - n) pos depth
  apply (Instruction Forward n) (State aim pos depth) = State aim (pos + n) (depth + aim * n)

answer :: IO Int
answer = result (State 0 0 0) <$> instructions