module Day1 (day1) where

import System.IO
import Data.List

-- Function to split a list into two lists with every other element
splitEveryOther :: [a] -> ([a], [a])
splitEveryOther = foldr (\x (ys, zs) -> (x : zs, ys)) ([], [])

count :: (Eq a) => a -> [a] -> Int
count x = length . filter (== x)

day1 :: IO ()
day1 = do
  handle <- openFile "res/input/day1.txt" ReadMode
  fileContents <- hGetContents handle

  let input = map read (words fileContents)
  let (left, right) = splitEveryOther input

  let part1 = sum [abs (uncurry (-) a) | a <- zip (sort left) (sort right)]

  putStr "Day 1 Part 1: "
  print part1

  let part2 = sum [a * count a right | a <- left]

  putStr "Day 1 Part 2: "
  print part2

  hClose handle
