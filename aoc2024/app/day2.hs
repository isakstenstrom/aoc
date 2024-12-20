module Day2 where

import System.IO

-- Function to produce all possible lists with one item removed
removeOne :: [a] -> [[a]]
removeOne [] = []
removeOne xs = [take i xs ++ drop (i + 1) xs | i <- [0..length xs - 1]]

day2Part1 :: [[Int]] -> Int
day2Part1 input = 
  let differences = [zipWith (-) x (tail x) | x <- input]
      decending = length $ filter (all (`elem` [1, 2, 3])) differences
      ascending = length $ filter (all (`elem` [-1, -2, -3])) differences
  in decending + ascending

day2Part2 :: [[Int]] -> Int
day2Part2 input = 
  let multipleInputs = [removeOne x | x <- input] :: [[[Int]]]
      differences = [[zipWith (-) x (tail x) | x <- y] | y <- multipleInputs]
      decending = [if (any (all (`elem` [1, 2, 3])) x) then 1 else 0 | x <- differences]
      ascending = [if (any (all (`elem` [-1, -2, -3])) x) then 1 else 0 | x <- differences]
  in sum decending + sum ascending

day2 :: IO ()
day2 = do
  handle <- openFile "res/input/day2.txt" ReadMode
  fileContents <- hGetContents handle

  let input = [map read (words x) | x <- lines fileContents] :: [[Int]]

  putStr "Day 2 Part 1: "
  print (day2Part1 input)

  putStr "Day 2 Part 2: "
  print (day2Part2 input)

  hClose handle
