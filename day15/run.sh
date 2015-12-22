export PATH="/opt/cabal/1.22/bin/:/opt/ghc/7.10.3/bin/:$PATH"
cabal exec -- ghc -o prog prog.hs && ./prog small.txt
