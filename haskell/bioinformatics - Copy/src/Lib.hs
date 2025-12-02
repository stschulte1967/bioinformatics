module Lib
        ( patternCount,
            frequentWords,
            reverseComplement
        ) where

import Data.List (tails, sort, group)

-- | Count overlapping occurrences of a pattern in a text.
patternCount :: String -> String -> Int
patternCount text pat
        | null pat = 0
        | otherwise = length $ filter (==pat) (kmers (length pat) text)

-- | Return all most frequent k-mers in the given text.
frequentWords :: Int -> String -> [String]
frequentWords k text
        | k <= 0 = []
        | null text = []
        | otherwise = map snd $ filter ((== maxCount) . fst) counts
    where
        ks = kmers k text
        counts = map (\ts -> (length ts, head ts)) . group . sort $ ks
        maxCount = if null counts then 0 else maximum (map fst counts)

-- | Produce all k-mers (substrings of length k) of the input text.
kmers :: Int -> String -> [String]
kmers k xs = [take k t | t <- tails xs, length t >= k]

-- | Compute the reverse complement of a DNA string. Unknown bases map to 'N'.
reverseComplement :: String -> String
reverseComplement = reverse . map convert
    where
        convert c = case c of
            'A' -> 'T'
            'T' -> 'A'
            'C' -> 'G'
            'G' -> 'C'
            'a' -> 't'
            't' -> 'a'
            'c' -> 'g'
            'g' -> 'c'
            _   -> 'N'