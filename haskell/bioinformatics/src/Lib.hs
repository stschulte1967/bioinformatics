module Lib
    ( patternCount
    ) where

patternCount :: String -> String -> Integer
patternCount [] _ = 0
patternCount text pattern = 
    if take patternLength text == pattern
    then
        1 + patternCount (tail text) pattern
    else patternCount (tail text) pattern
    where
        patternLength = length pattern  

