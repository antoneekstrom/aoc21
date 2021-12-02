module Part2 where

import Part1 hiding (Location (..), answer, apply, travel)

data State = State Int Int Int

apply (Instruction Down n) (State aim pos depth) = State (aim + n) pos depth
apply (Instruction Up n) (State aim pos depth) = State (aim - n) pos depth
apply (Instruction Forward n) (State aim pos depth) = State aim (pos + n) (depth + aim * n)

travel :: [Instruction] -> State
travel = foldr apply (State 0 0 0)

answer =
  do
    (State aim pos depth) <- travel . reverse <$> instructions
    return $ pos * depth