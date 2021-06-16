cargo build
@echo off

@REM Testing error handling for if files are not even jsons

echo ================================================
echo    TEST A : when using invalid schema
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\not-a-json.json D:\Dev\Geo\data\cityjson\den-haag-screwed.json

echo ================================================
echo    TEST B : when using invalid json
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\not-a-json.json

echo ================================================
echo    TEST C : when using a json, but not a cityjson
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\not-a-cityjson.json





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


