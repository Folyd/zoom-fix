cargo build --release
Get-ChildItem target\release
7z a $Env:PROJECT_NAME-$Env:TRAVIS_TAG-Env:TARGET.zip target\release\$ENV:PROJECT_NAME.exe