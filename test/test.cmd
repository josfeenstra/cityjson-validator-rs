cargo build
@echo off

@REM echo ================================================
@REM echo    TEST 1 : this is supposed to fail on schema
@REM echo ================================================
@REM "./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-screwed.json

@REM echo ================================================
@REM echo  TEST 2 : this is supposed to fail on dup verts
@REM echo ================================================
@REM "./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-dup-verts.json

echo =========================================================
echo  TEST 3 : this is supposed to fail on topology semantics
echo =========================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-wrong-topology.json

echo =====================================================
echo  TEST 4 : this is supposed to fail on topology logic
echo =====================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag-wrong-topology-2.json

echo ================================================
echo       TEST 5 : this is supposed to work
echo ================================================
"./target/debug/cityjson-validator.exe" D:\Dev\Geo\data\cityjson\schema.json D:\Dev\Geo\data\cityjson\den-haag.json


