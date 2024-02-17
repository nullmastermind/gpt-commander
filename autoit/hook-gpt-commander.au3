#include <GUIConstantsEx.au3>
#include <WindowsConstants.au3>
#include <MsgBoxConstants.au3>

HotKeySet("{F2}", "HandleHotKey")

Func HandleHotKey()
    Local $ssOldClipboard = ClipGet()
    Send("^c")
    Local $sOldClipboard = ClipGet()
    Local $aMousePos = MouseGetPos() ; Retrieve mouse position
    $aMousePos[0] -= 310
    If $aMousePos[0] < 0 Then
        $aMousePos[0] = 0
    EndIf
    Local $sCmd = ".\gpt-commander.exe " & $aMousePos[0] & " " & $aMousePos[1] ; Construct command with arguments
    RunWait($sCmd, @ScriptDir, @SW_SHOW) ; Run the program with mouse position as arguments
    Local $sNewClipboard = ClipGet()
    If $sOldClipboard <> $sNewClipboard Then
        Send("^v")
    EndIf
    ClipPut($ssOldClipboard)
EndFunc

; Main script loop
While 1
    Sleep(100)
WEnd