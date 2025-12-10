module Chapter1Spec where

import Test.Hspec
import Lib
import Data.List

spec :: Spec
spec = do
  describe "PatternCount" $ do
    it "simplest possible test" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_1.txt" 2
    it "a little bit more" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_2.txt" 3
    it "test 3" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_3.txt" 4
    it "test 4" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_4.txt" 4
    it "test 5" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_5.txt" 2
    it "test 6" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_6.txt" 5
    it "test 7" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_7.txt" 9
    it "test 8" $ do testPatternCount "../../data/PatternCount/1A/inputs/input_8.txt" 294

  describe "FrequentWords" $ do
    it "test 1" $ do testFrequentWords "../../data/FrequentWords/testset.txt" ["ACGT"]

  describe "MinimumSkew" $ do
    it "test 1" $ do testMinimumSkew "../../data/Skew/testset.txt" [11,24]
    it "test 2" $ do testMinimumSkew "../../data/Skew/quiz.txt" [14]
    it "test 3" $ do testMinimumSkew "../../data/Skew/dataset_30277_10.txt" [125, 130, 132, 133, 134, 136, 157, 158]

  describe "HammingDistance" $ do
    it "test 1" $ do testHammingDistance "../../data/HammingDistance/testset.txt" 3
    it "test 2" $ do testHammingDistance "../../data/HammingDistance/quiz.txt" 43
    it "test 3" $ do testHammingDistance "../../data/HammingDistance/dataset_30278_3.txt" 783

  describe "patternPosition using Hamming distance" $ do
    it "test 1" $ do testPatternPositionsUsingHamming "../../data/PatternPositionsHamming/testset.txt" [6, 7, 26, 27]
    it "test 2" $ do testPatternPositionsUsingHamming "../../data/PatternPositionsHamming/dataset_30278_4.txt" [6, 7, 26, 27]  

sameElements :: (Ord a) => [a] -> [a] -> Bool
sameElements xs ys = sort xs == sort ys 


testPatternCount filename solution = do    
    content <- readFile filename
    let ls = lines content
        text = if null ls then "" else head ls
        pat  = if length ls > 1 then ls !! 1 else ""
    patternCount text pat `shouldBe` solution

testFrequentWords filename solution = do
    content <- readFile filename
    let ls = lines content
        text = if null ls then "" else head ls
        len  = if length ls > 1 then read $ head ls else 0
        result = frequentWords len text
    print $ unwords result
    sameElements result solution `shouldBe` True 

testMinimumSkew filename solution = do
    text <- readFile filename
    let result = minimumSkew text
    print $ unwords (map show result)
    sameElements result solution `shouldBe` True

testHammingDistance filename solution = do
    content <- readFile filename
    let ls = lines content
        text1 = if null ls then "" else head ls
        text2  = if length ls > 1 then head (tail ls) else ""
        result = hammingDistance text1 text2
    print result
    result `shouldBe` solution

testPatternPositionsUsingHamming filename solution = do    
    content <- readFile filename
    let ls = lines content
        pat  = if null ls then "" else head ls
        text  = if length ls > 1 then ls !! 1 else ""
        d = if length ls > 2 then read (ls !! 2) else 0
        result = patternPositionsUsingHamming pat text d
    print $ length text
    print $ unwords . map show $ result
    result `shouldBe` solution
--do
--    content <- readFile filename
--    let ls = lines content
--        text = if null ls then "" else head ls
--        count  = read $ if length ls > 1 then ls !! 1 else ""
--    putStrLn $ unwords ( frequentWords count text)
--    content <- readFile filename
--    putStrLn $ reverseComplement content