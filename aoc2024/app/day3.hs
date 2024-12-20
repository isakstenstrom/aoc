module Day3 where

import System.IO
import Text.Regex.TDFA

-- Split a string based on a predicate
wordsWhen :: (Char -> Bool) -> String -> [String]
wordsWhen p s = case dropWhile p s of
                  "" -> []
                  s' -> w : wordsWhen p s''
                        where (w, s'') = break p s'

-- Multiply numbers extracted from a string
multiplyNumbers :: String -> Int
multiplyNumbers str = 
    let numbers = wordsWhen (== ',') (drop 4 (init str))
        num1 = read (numbers !! 0) :: Int
        num2 = read (numbers !! 1) :: Int
    in num1 * num2

-- Filter elements based on specific conditions
filterElements :: [String] -> Bool -> [String]
filterElements [] _ = []
filterElements (x:xs) shouldFilter = 
    let shouldFilter2 = case x of
                          "do()" -> False
                          "don't()" -> True
                          _ -> shouldFilter
    in case x of
        "do()" -> filterElements xs shouldFilter2
        "don't()" -> filterElements xs shouldFilter2
        _ -> if shouldFilter2 
             then filterElements xs shouldFilter2
             else x : filterElements xs shouldFilter2

day3Part1 :: String -> Int
day3Part1 fileContents = 
  let regexPattern = "mul\\([0-9]+,[0-9]+\\)"
      matches = getAllTextMatches (fileContents =~ regexPattern :: AllTextMatches [] String)
  in sum (map multiplyNumbers matches)

day3Part2 :: String -> Int
day3Part2 fileContents = 
  let regexPattern2 = "mul\\([0-9]+,[0-9]+\\)|do\\(\\)|don't\\(\\)"
      matches2 = getAllTextMatches (fileContents =~ regexPattern2 :: AllTextMatches [] String)
      filteredMatches = filterElements matches2 False
  in sum (map multiplyNumbers filteredMatches)

day3 :: IO ()
day3 = do
  handle <- openFile "res/input/day3.txt" ReadMode
  fileContents <- hGetContents handle

  putStr "Day 3 Part 1: "
  print (day3Part1 fileContents)

  putStr "Day 3 Part 2: "
  print (day3Part2 fileContents)

  hClose handle
