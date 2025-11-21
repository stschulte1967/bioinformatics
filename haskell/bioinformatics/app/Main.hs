
module Main (main) where

import Lib

filename :: FilePath
filename = "../../data/reverseComplement.txt"

main :: IO ()
main = do
--    content <- readFile filename
--    let ls = lines content
--        text = if null ls then "" else head ls
--        pat  = if length ls > 1 then ls !! 1 else ""
--    print $ patternCount text pat
--    content <- readFile filename
--    let ls = lines content
--        text = if null ls then "" else head ls
--        count  = read $ if length ls > 1 then ls !! 1 else ""
--    putStrLn $ unwords ( frequentWords count text)
    content <- readFile filename
    putStrLn $ reverseComplement content