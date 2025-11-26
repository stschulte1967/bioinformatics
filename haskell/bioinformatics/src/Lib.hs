module Lib
    ( patternCount,
      frequentWords,
      reverseComplement,
      patternPositions,
      findClumbs
    ) where

import Data.Ord (comparing, Down (Down))
import Data.List (tails, sort, sortBy, isPrefixOf, findIndices)

patternCount :: String -> String -> Integer
patternCount [] _ = 0
patternCount (x:xs) pat = 
    if take patternLength (x:xs) == pat
    then
        1 + patternCount xs pat
    else patternCount xs pat
    where
        patternLength = length pat  

frequentWords :: Int -> String -> [String]
frequentWords k = findMax . sortRun . countRun . sortKmers . kmers k 

kmers :: Int -> String -> [String]
kmers k xs = map (take k) (tails xs)

sortKmers :: [String] -> [String]
sortKmers = sort

sortRun :: [(Int, String)] -> [(Int, String)]
sortRun = sortBy (comparing Down)

countRun :: [String] -> [(Int, String)]
countRun [] = []
countRun xs = (1 + length vu, w) : countRun vv 
    where
        w = head xs 
        (vu, vv) = span (==w) (tail xs)

findMax :: [(Int, String)] -> [String]
findMax xs =map snd $ takeWhile (\x -> fst x == fst (head xs)) xs

findBigger :: Int -> [(Int, String)] -> [String]
findBigger n xs =map snd $ takeWhile (\x -> fst x >= n) xs

reverseComplement :: String -> String
reverseComplement  = reverse . map convert
    where
        convert x = case x of
            'A' -> 'T'
            'T' -> 'A'
            'C' -> 'G'
            'G' -> 'C'
            _ -> 'X'

patternPositions :: String -> String -> [Int]
patternPositions text pat = findIndices (isPrefixOf pat) (tails text)

findClumbs :: String -> Int -> Int -> Int -> [String]
findClumbs text k _ t = findBigger t . sortRun . countRun . sortKmers . kmers k $ text