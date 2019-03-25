@rem This batch file defines the various LibZeroMQ environment variables and
@rem copies the dynamic library built with CMake to "zmq.lib" 
@rem Input:  
@rem     See :ERROR_ARGUMENT
@rem Dependencies:
@rem      For Windows operating system
@rem      Compiled libzmq binaries (CMake & mingw32-make, or
@rem                                CMake & Visual Studio, or
@rem                                Visual Studio with built in CMake)

@cls
@echo -------------------------
@echo LibZeroMQ: %~n0
@echo -------------------------
@echo.

@rem "set silent=@" for silent execution or set "silent=" for debugging
@set silent=@

%silent%if [%1]==[] @(
    %silent%set myMsg="Arguments expected."
    %silent%goto ERROR_ARGUMENT:
)

@echo Input arguments:
%silent%for %%x in (%*) do @(
    @echo ^ ^ ^ %%x
)
@echo.

@rem Clear all LibZeroMQ environment variables
%silent%call :TO_UPPER arg1 %1
%silent%if [%arg1%]==[-CLEAR_ALL] (
    @echo Clear all LibZeroMQ environment variables
    @echo.
    
    %silent%set LIBZMQ_PREFIX=
    %silent%set LIBZMQ_LIB_DIR=
    %silent%set LIBZMQ_INCLUDE_DIR=
    %silent%set LIBZMQ_BIN_DIR=
    
    %silent%set MyEnvVariable=LIBZMQ_PREFIX
    %silent%call :DELETE_ENV_REG
    
    %silent%set MyEnvVariable=LIBZMQ_LIB_DIR
    %silent%call :DELETE_ENV_REG
    
    %silent%set MyEnvVariable=LIBZMQ_INCLUDE_DIR
    %silent%call :DELETE_ENV_REG
    
    %silent%set MyEnvVariable=LIBZMQ_BIN_DIR
    %silent%call :DELETE_ENV_REG
    
    @echo.
    %silent%set LIBZMQ_PREFIX
    %silent%set LIBZMQ_LIB_DIR
    %silent%set LIBZMQ_INCLUDE_DIR
    %silent%set LIBZMQ_BIN_DIR
   
    @goto END:
)

@rem Read arguments & Test folder paths
%silent%set LIBZMQ_PREFIX=
%silent%set LIBZMQ_LIB_DIR=
%silent%set LIBZMQ_INCLUDE_DIR=
%silent%set LIBZMQ_BIN_DIR=
%silent%set LIB_TO_COPY=
@setlocal enableextensions
@setlocal enabledelayedexpansion
%silent%set FolderError=
%silent%if [%1]==[-LIBZMQ_PREFIX] (
    
    @echo Set up default LibZeroMQ environment variables

    @rem Expecting a specific folder structure
    %silent%if [%2]==[] (
        %silent%set myMsg="No value for '-LIBZMQ_PREFIX' provided."
        %silent%goto ERROR_ARGUMENT:
    )
    
    @rem LIBZMQ_PREFIX
    set LIBZMQ_PREFIX=%~2
    %silent%if not exist "!LIBZMQ_PREFIX!" (
        %silent%set FolderError="!LIBZMQ_PREFIX!"
        %silent%call :ERROR_FOLDER
    )
    
    @rem LIBZMQ_LIB_DIR
    %silent%set LIBZMQ_LIB_DIR=!LIBZMQ_PREFIX!\lib
    %silent%if not exist "!LIBZMQ_LIB_DIR!" (
        %silent%set FolderError="!LIBZMQ_LIB_DIR!"
        %silent%call :ERROR_FOLDER
    )
    
    @rem LIBZMQ_INCLUDE_DIR
    %silent%set LIBZMQ_INCLUDE_DIR=!LIBZMQ_PREFIX!\include
    %silent%if not exist "!LIBZMQ_INCLUDE_DIR!" (
        %silent%set FolderError="!LIBZMQ_INCLUDE_DIR!"
        %silent%call :ERROR_FOLDER
    )
    
    @rem LIBZMQ_BIN_DIR
    %silent%set LIBZMQ_BIN_DIR=!LIBZMQ_PREFIX!\bin
    %silent%if not exist "!LIBZMQ_BIN_DIR!" (
        %silent%set FolderError="!LIBZMQ_BIN_DIR!"
        %silent%call :ERROR_FOLDER
    )
    
    @rem -LIB_TO_COPY
    %silent%if [%3]==[-LIB_TO_COPY] (
        %silent%if [%4]==[] (
            %silent%set myMsg="No value for '-LIB_TO_COPY' provided."
            %silent%goto ERROR_ARGUMENT:
        ) else (
            %silent%set LIB_TO_COPY=%~4
        )
    )

) else (

    @echo Set up customized LibZeroMQ environment variables
    
    @rem LIBZMQ_LIB_DIR
    %silent%if not [%1]==[-LIBZMQ_LIB_DIR] (
        %silent%set myMsg="Argument '-LIBZMQ_LIB_DIR' expected, '%1' provided."
        %silent%goto ERROR_ARGUMENT:
    )
    %silent%if [%1]==[-LIBZMQ_LIB_DIR] (
        %silent%if [%2]==[] (
            %silent%set myMsg="No value for '-LIBZMQ_LIB_DIR' provided."
            %silent%goto ERROR_ARGUMENT:
        )
        %silent%set LIBZMQ_LIB_DIR=%~2
        %silent%if not exist "!LIBZMQ_LIB_DIR!" (
            %silent%set FolderError="!LIBZMQ_LIB_DIR!"
            %silent%call :ERROR_FOLDER
        )
    )

    @rem LIBZMQ_INCLUDE_DIR
    %silent%if not [%3]==[-LIBZMQ_INCLUDE_DIR] (
        %silent%set myMsg="Argument '-LIBZMQ_INCLUDE_DIR' expected, '%3' provided."
        %silent%goto ERROR_ARGUMENT:
    )
    %silent%if [%3]==[-LIBZMQ_INCLUDE_DIR] (
        %silent%if [%4]==[] (
            %silent%set myMsg="No value for '-LIBZMQ_INCLUDE_DIR' provided."
            %silent%goto ERROR_ARGUMENT:
        )
        %silent%set LIBZMQ_INCLUDE_DIR=%~4
        %silent%if not exist "!LIBZMQ_INCLUDE_DIR!" (
            %silent%set FolderError="!LIBZMQ_INCLUDE_DIR!"
            %silent%call :ERROR_FOLDER
        )
    )

    @rem LIBZMQ_BIN_DIR
    %silent%if not [%5]==[-LIBZMQ_BIN_DIR] (
        %silent%set myMsg="Argument '-LIBZMQ_BIN_DIR' expected, '%5' provided."
        %silent%goto ERROR_ARGUMENT:
    )
    %silent%if [%5]==[-LIBZMQ_BIN_DIR] (
        %silent%if [%6]==[] (
            %silent%set myMsg="No value for '-LIBZMQ_BIN_DIR' provided."
            %silent%goto ERROR_ARGUMENT:
        )
        %silent%set LIBZMQ_BIN_DIR=%~6
        %silent%if not exist "!LIBZMQ_BIN_DIR!" (
            %silent%set FolderError="!LIBZMQ_BIN_DIR!"
            %silent%call :ERROR_FOLDER
        )
    )

    @rem -LIB_TO_COPY
    %silent%if [%7]==[-LIB_TO_COPY] (
        %silent%if [%8]==[] (
            %silent%set myMsg="No value for '-LIB_TO_COPY' provided."
            %silent%goto ERROR_ARGUMENT:
        ) else (
            %silent%set LIB_TO_COPY=%~8
        )
    )
    
)
%silent%if defined FolderError (echo. & @goto :eof)

@rem Assign LibZeroMQ enviornment variables
%silent%if defined LIBZMQ_PREFIX (
    setx LIBZMQ_PREFIX !LIBZMQ_PREFIX!
) else (
    @echo.
    %silent%set MyEnvVariable=LIBZMQ_PREFIX
    %silent%call :DELETE_ENV_REG
)
%silent%setx LIBZMQ_LIB_DIR !LIBZMQ_LIB_DIR!
%silent%setx LIBZMQ_INCLUDE_DIR !LIBZMQ_INCLUDE_DIR!
%silent%setx LIBZMQ_BIN_DIR !LIBZMQ_BIN_DIR!

@echo.

@rem Copy dynamic lib to "zmq.lib" 
%silent%if defined LIB_TO_COPY (
    %silent%if exist "!LIBZMQ_LIB_DIR!\zmq.lib" del  /f /q "!LIBZMQ_LIB_DIR!\zmq.lib"
    %silent%xcopy "!LIBZMQ_LIB_DIR!\%LIB_TO_COPY%" "!LIBZMQ_LIB_DIR!\zmq.lib*" /v /y
    %silent%if not exist "!LIBZMQ_LIB_DIR!\zmq.lib" (
        @echo Error: Could not create "!LIBZMQ_LIB_DIR!\zmq.lib"
    ) else (
        @echo ^ ^ Copied '%LIB_TO_COPY%' to 'zmq.lib' in
        @echo ^ ^ ^ ^ '!LIBZMQ_LIB_DIR!'
    )
)
@echo.

@rem endlocal

@echo LIBZMQ_PREFIX      = '%LIBZMQ_PREFIX%'
@echo LIBZMQ_LIB_DIR     = '%LIBZMQ_LIB_DIR%'
@echo LIBZMQ_INCLUDE_DIR = '%LIBZMQ_INCLUDE_DIR%'
@echo LIBZMQ_BIN_DIR     = '%LIBZMQ_BIN_DIR%'

%silent%if not exist "!LIBZMQ_LIB_DIR!\zmq.lib" (
    @echo.
    @echo Warning: Library file 'zmq.lib' does not exist in
    @echo ^ ^ '!LIBZMQ_LIB_DIR!'
)

    
@goto END:

:ERROR_ARGUMENT
@echo.
@echo Error: %myMsg%
@echo.
@echo Usage:
@echo.
@echo    %~n0 -CLEAR_ALL
@echo            to remove all LibZeroMQ environment variables
@echo.
@echo                           or
@echo.
@echo    %~n0 -LIBZMQ_PREFIX "<LIBZMQ_PREFIX>" -LIB_TO_COPY "<Optional: Dynamic Library Name>"
@echo            to set up default LibZeroMQ environment variables:
@echo                 LIBZMQ_LIB_DIR     = ^<LIBZMQ_PREFIX^>\lib
@echo                 LIBZMQ_INCLUDE_DIR = ^<LIBZMQ_PREFIX^>\include
@echo                 LIBZMQ_BIN_DIR     = ^<LIBZMQ_PREFIX^>\bin
@echo.
@echo                           or
@echo.
@echo    %~n0 -LIBZMQ_LIB_DIR "<library folder>" -LIBZMQ_INCLUDE_DIR "<include folder>" 
@echo    -LIBZMQ_BIN_DIR "<bin folder>" -LIB_TO_COPY "<Optional: Dynamic Library Name>"
@echo            to set up customized LibZeroMQ environment variables
@echo.
@pause
@goto :eof

:ERROR_FOLDER
@echo.
@echo Error: Folder %FolderError% does not exist
@goto :eof

:RESET_ERROR_LEVEL
@if not defined silent (echo. & echo ErrorLevel before reset = %errorlevel%)
@goto :eof

:TO_UPPER
%silent%for /f "usebackq delims=" %%a in (`powershell "\"%2\".toUpper()"`) do @(
    set "%~1=%%~a"
)
@goto :eof

:DELETE_ENV_REG
@if not defined MyEnvVariable (echo. & echo DELETE_ENV_REG: Environment variable 'MyEnvVariable' must be defined & @goto :eof)
@rem User environment
@echo Removing '%MyEnvVariable%' from user environment
%silent%call :RESET_ERROR_LEVEL
%silent%reg query HKEY_CURRENT_USER\Environment | findstr /i %MyEnvVariable%
@if not defined silent (echo. & echo ErrorLevel after findstr = %errorlevel%)
%silent%if %errorlevel% EQU 0 (
    @echo Remove user environment variable: %MyEnvVariable%
    %silent%powershell [Environment]::SetEnvironmentVariable^('%MyEnvVariable%', $null, 'User'^)
)
@rem System environment
@echo Removing '%MyEnvVariable%' from system environment
%silent%call :RESET_ERROR_LEVEL
%silent%reg query "HKEY_LOCAL_MACHINE\SYSTEM\CurrentControlSet\Control\Session Manager\Environment" | findstr /i %MyEnvVariable%
@if not defined silent (echo. & echo ErrorLevel after findstr = %errorlevel%)
%silent%if %errorlevel% EQU 0 (
    @echo Remove system environment variable: %MyEnvVariable%
    %silent%powershell [Environment]::SetEnvironmentVariable^('%MyEnvVariable%', $null, 'Machine'^)
)
@echo.
@goto :eof


:END
@rem Cleanup
@set arg1=
@set MyEnvVariable=
@set FolderError= 
@rem End
@echo.
@if not defined silent pause

