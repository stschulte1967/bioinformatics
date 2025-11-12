module Lib
    ( patternCount,
      frequentWords
    ) where

patternCount :: String -> String -> Integer
patternCount [] _ = 0
patternCount (x:xs) pat = 
    if take patternLength (x:xs) == pat
    then
        1 + patternCount xs pat
    else patternCount xs pat
    where
        patternLength = length pat  

frequentWords :: String -> Int -> [String]
frequentWords text k = [""]

