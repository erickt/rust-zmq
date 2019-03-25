@rem This batch file verifies all LibZeroMQ environment variables and
@rem add the LibZeroMQ bin folder to the local session path .
@rem Dependencies:
@rem      For Windows operating system

@cls
@echo -------------------------
@echo LibZeroMQ: %~n0
@echo -------------------------
@echo.

@rem "set silent=@" for silent execution or set "silent=" for debugging
@set silent=@

@rem setlocal enableextensions


@rem Test LibZeroMQ environment variables
%silent%set EnvVarError=
%silent%if not defined LIBZMQ_LIB_DIR (
    %silent%set EnvVarError=LIBZMQ_LIB_DIR
    %silent%call :ERROR_LIBZEROMQ_VARS
)
%silent%if not defined LIBZMQ_INCLUDE_DIR (
    %silent%set EnvVarError=LIBZMQ_INCLUDE_DIR
    %silent%call :ERROR_LIBZEROMQ_VARS
)
%silent%if not defined LIBZMQ_BIN_DIR (
    %silent%set EnvVarError=LIBZMQ_BIN_DIR
    %silent%call :ERROR_LIBZEROMQ_VARS
)
%silent%if defined EnvVarError (
    @set EnvVarError=
    @echo.
    @pause
    @goto :eof
)

@rem Test folder names
@setlocal enabledelayedexpansion
%silent%set FolderError=
%silent%if not exist "!LIBZMQ_LIB_DIR!" (
    %silent%set FolderError="!LIBZMQ_LIB_DIR!"
    %silent%call :ERROR_FOLDER
)
%silent%if not exist "!LIBZMQ_INCLUDE_DIR!" (
    %silent%set FolderError="!LIBZMQ_INCLUDE_DIR!"
    %silent%call :ERROR_FOLDER
)
%silent%if not exist "!LIBZMQ_BIN_DIR!" (
    %silent%set FolderError="!LIBZMQ_BIN_DIR!"
    %silent%call :ERROR_FOLDER
)
%silent%if defined FolderError (
    @set FolderError=
    @echo.
    @pause
    @goto :eof
)
@endlocal

@rem Test environment variables

@rem Update path to enable access to the ZeroMQ DLLs defined in zmq.lib 
%silent%call :RESET_ERROR_LEVEL
%silent%path | findstr /i %LIBZMQ_BIN_DIR%
@if not defined silent (echo. & echo ErrorLevel after findstr = %errorlevel%)
%silent%if errorlevel 1 (
    %silent%set "path=%LIBZMQ_BIN_DIR%;%path%"
    @echo Local session path updated
    @echo.
    @path
) else (
    @echo.
    @echo Local session path is up to date
)

@rem Test if "zmq.lib" exits
%silent%if not exist "%LIBZMQ_LIB_DIR%\zmq.lib" (
    @echo Error: Could not find "%LIBZMQ_LIB_DIR%\zmq.lib"
    @echo.
    @pause
)

@goto END:

:ERROR_LIBZEROMQ_VARS
@echo.
@echo Error: '%EnvVarError%' LibZeroMQ environment variable not set
@exit /b 0

:ERROR_NAME
@echo.
@echo Error: %EnvVarError%
@exit /b 0

:ERROR_FOLDER
@echo.
@echo Error: Folder %FolderError% does not exist
@exit /b 0

:RESET_ERROR_LEVEL
@if not defined silent (echo. & echo ErrorLevel before reset = %errorlevel%)
@exit /b 0

:END
@echo.
@if not defined silent pause

