module Lib
    ( patternCount,
      frequentWords,
      reverseComplement,
      patternPositions,
      findClumbs,
      skew,
      minimumSkew,
      hammingDistance
    ) where

import Data.Ord (comparing, Down (Down))
import Data.List (tails, sort, sortBy, isPrefixOf, findIndices, group, foldl')
import qualified Data.HashMap.Strict as HM
import qualified Data.HashSet as HS
import Data.Array (Array, listArray, (!))
import Data.Int (Int)

patternCount :: String -> String -> Int
patternCount _ [] = 0
patternCount text pat = length $ filter (isPrefixOf pat) (tails text)


frequentWords :: Int -> String -> [String]
frequentWords k text
    | k <= 0 = []
    | otherwise =
            case kmers k text of
                [] -> []
                ks -> findMax . sortRun . countRun . sort $ ks

kmers :: Int -> String -> [String]
kmers k xs = [take k t | t <- tails xs, length t >= k]

sortRun :: [(Int, String)] -> [(Int, String)]
sortRun = sortBy (comparing (Down . fst))

countRun :: [String] -> [(Int, String)]
countRun = map (\bs@(b:_) -> (length bs, b)) . group

findMax :: [(Int, String)] -> [String]
findMax [] = []
findMax xs@(x:_) = map snd $ takeWhile ((== fst x) . fst) xs

findBigger :: Int -> [(Int, String)] -> [String]
findBigger _ [] = []
findBigger n xs = map snd $ takeWhile ((>= n) . fst) xs

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
patternPositions _ [] = []
patternPositions text pat = findIndices (isPrefixOf pat) (tails text)

findClumbs :: Int -> Int -> Int -> String -> [String]
findClumbs k l t text
    | k <= 0 || t <= 0 || l < k = []
    | length text < l = []
    | otherwise =
        let n = length text
            kmList = kmers k text
            totalK = length kmList
            kmArr :: Array Int String
            kmArr = listArray (0, totalK - 1) kmList
            initialCount = l - k + 1
            -- build counts for the first window (kmers at indices 0 .. initialCount-1)
            counts0 = foldl' (\m i -> HM.insertWith (+) (kmArr ! i) 1 m) HM.empty [0 .. initialCount - 1]
            acc0 = HS.fromList [s | (s,c) <- HM.toList counts0, c >= t]
            -- slide windows: for window starting at i (chars), the outgoing kmer index = i-1, incoming kmer index = i + l - k
            step i (m, acc) =
                let outIdx = i - 1
                    inIdx = i + l - k
                    outK = kmArr ! outIdx
                    inK = kmArr ! inIdx
                    m' = HM.update (\c -> if c > 1 then Just (c-1) else Nothing) outK m
                    oldIn = HM.lookupDefault 0 inK m'
                    m'' = HM.insertWith (+) inK 1 m'
                    acc' = if oldIn + 1 >= t then HS.insert inK acc else acc
                in (m'', acc')
            (_, accFinal) = foldl' (flip step) (counts0, acc0) [1 .. (n - l)]
        in sort $ HS.toList accFinal

positionsOfMin :: (Ord a) => [a] -> [Int]
positionsOfMin [] = []  -- Handle empty list
positionsOfMin xs =
    let minVal = minimum xs
    in [i | (i, x) <- zip [0..] xs, x == minVal]

skew :: String -> [Int]
skew text = scanl (+) 0 (map (\c -> if c == 'C' then -1 else if c == 'G' then 1 else 0) text)

minimumSkew :: String -> [Int]
minimumSkew text = positionsOfMin $ skew text

hammingDistance :: String -> String -> Int
hammingDistance text1 text2 = sum ( zipWith (\x y -> if x == y then 0 else 1) text1 text2 )