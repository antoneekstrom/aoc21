module Main where

import qualified Part1 (main)
import qualified Part2 (main)

main = mapM_ print =<< sequence [Part1.main, Part2.main]