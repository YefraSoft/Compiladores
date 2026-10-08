@echo off
REM JFlexFork launcher: JFLEX_HOME is the folder above bin\
set JFLEX_HOME=%~dp0..

java -Xmx128m -jar "%JFLEX_HOME%\target\jflex-1.5.0-fork.jar" %*
