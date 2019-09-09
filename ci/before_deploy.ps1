cargo build --release
Get-ChildItem
7z a %PROJECT_NAME%-%TRAVIS_TAG%-%TARGET%.zip target\release\%PROJECT_NAME%.exe