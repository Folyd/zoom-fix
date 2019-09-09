Get-ChildItem target\$Env:TARGET\release
7z a $Env:PROJECT_NAME-$Env:TRAVIS_TAG-$Env:TARGET.zip target\$Env:TARGET\release\$Env:PROJECT_NAME.exe
Get-ChildItem