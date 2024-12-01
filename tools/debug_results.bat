pushd "%~dp0"

cd ..
call target\debug\AdventOfCode2024.exe > README.md

popd
