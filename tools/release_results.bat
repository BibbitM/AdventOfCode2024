pushd "%~dp0"

cd ..
call target\release\AdventOfCode2024.exe > README.md

popd
