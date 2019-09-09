cargo build --release
mkdir staging
copy target\release\%PROJECT_NAME%.exe staging
copy target\release\build\*\out\_%PROJECT_NAME%.ps1 staging
cd staging
7z a ../%PROJECT_NAME%-%TRAVIS_TAG%-%TARGET%.zip *