import System.Process

main = do
    text <- readProcess "ls ["-a"] ""
    putStrLn "Hello"
