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

echo =========================================================
echo  TEST 3 : this is supposed to fail on topology semantics
echo =========================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-wrong-topology.json

echo =====================================================
echo  TEST 4 : this is supposed to fail on topology logic
echo =====================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-wrong-topology-2.json

echo ================================================
echo       TEST 5 : should work : Den Haag
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag.json

echo ================================================
echo       TEST 6 : should work : Rotterdam
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\rotterdam.json

echo ================================================
echo       TEST 7 : should work : New York
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\new-york.json


