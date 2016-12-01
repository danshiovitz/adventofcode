import System.Environment
import qualified Data.Text as T
import Data.Function (on)
import Data.List
import Text.Printf
import Text.Regex.PCRE

data Properties = Properties {
  capacity :: Int, durability :: Int, flavor :: Int, texture :: Int, calories :: Int
  } deriving (Show)

data Ingredient = Ingredient {
  name :: String, properties :: Properties
  } deriving (Show)

data Component = Component {
  ingredient :: Ingredient, amount :: Int
  } deriving (Show)
                 
data Cookie = Cookie {
  totalProperties :: Properties, score :: Int, components :: [Component] 
  } deriving (Show)

-- https://wiki.haskell.org/Haskell_IO_for_Imperative_Programmers#IO
solveFile fileName quantity = do
  s <- readFile fileName
  let ingredients = (map parseLine . lines) s
      cookies = listCookies ingredients quantity
      bestCookie = maximumBy (on compare score) cookies in
    printf "Best Cookie: %d\n" (score bestCookie)

parseLine :: String -> Ingredient
parseLine line = do
  -- https://github.com/erantapaa/haskell-regexp-examples/blob/master/RegexExamples.hs
  let regex = "(\\S+): capacity (-?\\d+), durability (-?\\d+), flavor (-?\\d+), texture (-?\\d+), calories (-?\\d+)"
      matches = getAllTextSubmatches $ line =~ regex :: [String]
      name = matches!!1
      capacity = read (matches!!2) :: Int
      durability = read (matches!!3) :: Int
      flavor = read (matches!!4) :: Int
      texture = read (matches!!5) :: Int
      calories = read (matches!!6) :: Int
      props = Properties capacity durability flavor texture calories in
    Ingredient name props

listCookies :: [Ingredient] -> Int -> [Cookie]
listCookies ingredients qty = do
  let components = possibleComponents ingredients qty in
    map assembleCookie components

possibleComponents :: [Ingredient] -> Int -> [[Component]]
possibleComponents [] qty = error "No ingredients available"
possibleComponents ingredients 0 = [[]]
possibleComponents [single] qty = [[Component single qty]]
possibleComponents ingredients qty = do
  let remaining = tail ingredients
      addComponent amount = ((:) (Component (head ingredients) amount))
      addFor amount = map (addComponent amount) (possibleComponents remaining (qty - amount))
    in
    foldl (++) [[]] (map addFor [0,1..qty])

assembleCookie comps = do
  let props = sumProperties comps
      totalScore = scoreProperties props
    in
    Cookie{totalProperties=props, score=totalScore, components=comps}

sumProperties comps = do
  let per f c = (f (properties (ingredient c))) * (amount c)
      summul f = sum (map (per f) comps)
  Properties{
    capacity=summul capacity,
    durability=summul durability,
    flavor=summul flavor,
    texture=summul texture,
    calories=summul calories
    }

scoreProperties props = do
  let values = [(capacity props), (durability props), (flavor props), (texture props)] in
    if (((calories props) /= 500) || (any ((>=) 0) values)) then
      0
    else
      product values

main = do
  (fileName:quantityStr:args) <- getArgs
  let quantity = read quantityStr :: Int in
    solveFile fileName quantity
