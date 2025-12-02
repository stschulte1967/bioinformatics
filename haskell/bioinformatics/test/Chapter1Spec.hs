module Chapter1Spec where

import Test.Hspec
import Lib

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
    it "test 1" $ do testFrequentWords "../../data/PatternCount/1A/inputs/input_1.txt" 2

sameElements xs ys = sort xs == sort ys 
sameElements :: (Ord a) => [a] -> [a] -> Bool

testPatternCount filename solution = do    
    content <- readFile filename
    let ls = lines content
        text = if null ls then "" else head ls
        pat  = if length ls > 1 then ls !! 1 else ""
    patternCount text pat `shouldBe` solution

--do
--    content <- readFile filename
--    let ls = lines content
--        text = if null ls then "" else head ls
--        count  = read $ if length ls > 1 then ls !! 1 else ""
--    putStrLn $ unwords ( frequentWords count text)
--    content <- readFile filename
--    putStrLn $ reverseComplement content