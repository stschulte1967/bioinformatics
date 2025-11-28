
module Main (main) where

import Lib

filename :: FilePath
--filename = "../../data/FindClumbs/dataset_30274_5.txt"
filename = "../../data/FindClumbs/E_coli_set.txt"

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
--   content <- readFile filename
--  putStrLn $ reverseComplement content
--    content <- readFile filename
--    let ls = lines content
--        pat  = if null ls then "" else head ls
--        text = if length ls > 1 then ls !! 1 else ""
--    putStrLn $ unwords ( map show (patternPositions text pat))    
    content <- readFile filename
    let 
        ls = lines content
        text = if null ls then "" else head ls
        params = words (if length ls > 1 then ls !! 1 else "")
        k = read . head $ params
        l = read (params !! 1)
        t = read (params !! 2)
        result = findClumbs k l t text
    --putStrLn $ unwords (findClumbs k l t text)
    putStrLn $ unwords result
    print $ length result


