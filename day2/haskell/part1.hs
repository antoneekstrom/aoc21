module Part1 where

import Data.Char (isLetter)

input :: IO [String]
input = lines <$> readFile "input"

data Direction = Forward | Up | Down
  deriving (Show)

data Instruction = Instruction Direction Int
  deriving (Show)

parse :: String -> Instruction
parse s =
  let d = takeWhile isLetter s
      n = read $ drop (length d + 1) s :: Int
   in case d of
        "forward" -> Instruction Forward n
        "up" -> Instruction Up n
        "down" -> Instruction Down n
        _ -> error "Invalid direction"

instructions :: IO [Instruction]
instructions = map parse <$> input

data Location = Location Int Int
  deriving (Show)

apply :: Instruction -> Location -> Location
apply (Instruction Forward n) (Location pos depth) = Location (pos + n) depth
apply (Instruction Down n) (Location pos depth) = Location pos (depth + n)
apply (Instruction Up n) (Location pos depth) = Location pos (depth - n)

travel :: [Instruction] -> Location
travel = foldr apply (Location 0 0)

answer =
  do
    (Location pos depth) <- travel <$> instructions
    return $ pos * depth