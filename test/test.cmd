cargo build
@echo off

echo ================================================
echo    TEST 1 : this is supposed to fail on schema
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-screwed.json

echo ================================================
echo  TEST 2 : this is supposed to fail on dup verts
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-dup-verts.json

echo ================================================
echo       TEST 3 : this is supposed to work
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag.json


