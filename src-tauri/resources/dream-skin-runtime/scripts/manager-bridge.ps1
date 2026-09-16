[CmdletBinding()]
param(
  [Parameter(Mandatory = $true)]
  [ValidateSet('ensure-session', 'apply-image', 'set-opacity', 'restore', 'status')]
  [string]$Action,
  [string]$ImagePath,
  [ValidateRange(0.1, 1.0)]
  [double]$Opacity = 0.8,
  [ValidateRange(1024, 65535)]
  [int]$Port = 9335
)

$ErrorActionPreference = 'Stop'
$SkillRoot = Split-Path -Parent $PSScriptRoot
$StateRoot = Join-Path $env:LOCALAPPDATA 'CodexDreamSkin'
$StatePath = Join-Path $StateRoot 'state.json'
$Injector = Join-Path $PSScriptRoot 'injector.mjs'
$PowerShell = (Get-Command powershell.exe -ErrorAction Stop).Source

. (Join-Path $PSScriptRoot 'common-windows.ps1')
. (Join-Path $PSScriptRoot 'theme-windows.ps1')
. (Join-Path $PSScriptRoot 'localization-windows.ps1')

function Get-ManagerSession {
  $state = Read-DreamSkinState -Path $StatePath
  if ($null -eq $state -or -not $state.port -or -not $state.browserId) { return $null }
  $codex = Get-DreamSkinCodexInstall
  $statePort = [int]$state.port
  $identity = Get-DreamSkinVerifiedCdpIdentity -Port $statePort -Codex $codex
  if ($null -eq $identity) { return $null }
  return [pscustomobject]@{
    Codex = $codex
    Identity = $identity
    Port = $statePort
    State = $state
  }
}

function Start-ManagerSession {
  $startScript = Join-Path $PSScriptRoot 'start-dream-skin.ps1'
  $result = Invoke-DreamSkinNative -FilePath $PowerShell -ArgumentList @(
    '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'RemoteSigned',
    '-File', $startScript, '-Port', "$Port", '-RestartExisting', '-OneShot'
  )
  if ($result.ExitCode -ne 0) {
    $details = (($result.Output | Select-Object -Last 8) -join "`n").Trim()
    if (-not $details) { $details = 'Dream Skin startup failed.' }
    throw $details
  }
  $session = Get-ManagerSession
  if ($null -eq $session) {
    throw 'ChatGPT started, but no verified local CDP session was found. Check whether the Windows Store version permits CDP.'
  }
  return $session
}

function Ensure-ManagerSession {
  $session = Get-ManagerSession
  if ($null -eq $session) { $session = Start-ManagerSession }
  return $session
}

function Invoke-ManagerInjection {
  param([Parameter(Mandatory = $true)][object]$Session)
  $paths = Get-DreamSkinThemePaths -StateRoot $StateRoot
  $node = Get-DreamSkinNodeRuntime
  $result = Invoke-DreamSkinNative -FilePath $node.Path -ArgumentList @(
    $Injector, '--once', '--port', "$($Session.Port)", '--browser-id', $Session.Identity.BrowserId,
    '--theme-dir', $paths.Active, '--timeout-ms', '30000'
  )
  if ($result.ExitCode -ne 0) {
    $details = (($result.Output | Select-Object -Last 8) -join "`n").Trim()
    if (-not $details) { $details = 'Theme injection failed.' }
    throw $details
  }
}

function Invoke-ManagerRestore {
  # 仅移除页面样式会残留外观配置；同时恢复换肤前快照，才能回到原始官方配色。
  $restoreScript = Join-Path $PSScriptRoot 'restore-dream-skin.ps1'
  $result = Invoke-DreamSkinNative -FilePath $PowerShell -ArgumentList @(
    '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'RemoteSigned',
    '-File', $restoreScript, '-RestoreBaseTheme', '-ForceRestart'
  )
  if ($result.ExitCode -ne 0) {
    $details = (($result.Output | Select-Object -Last 8) -join "`n").Trim()
    if (-not $details) { $details = 'Restoring the official appearance failed.' }
    throw $details
  }
}

  function Set-ManagerOpacity {
    param([Parameter(Mandatory = $true)][object]$Session)
    if (Test-DreamSkinPaused -StateRoot $StateRoot) {
      return [pscustomobject]@{ active = $false; opacity = $Opacity; port = $Session.Port }
    }
    $paths = Get-DreamSkinThemePaths -StateRoot $StateRoot
  $themePath = Join-Path $paths.Active 'theme.json'
  if (-not (Test-Path -LiteralPath $themePath -PathType Leaf)) {
    return [pscustomobject]@{ active = $false; opacity = $Opacity; port = $Session.Port }
  }
  $active = Read-DreamSkinTheme -ThemeDirectory $paths.Active -SkipImageMetadata
  $theme = $active.Theme | ConvertTo-Json -Depth 16 | ConvertFrom-Json
  $theme | Add-Member -NotePropertyName imageOpacity -NotePropertyValue ([math]::Round($Opacity, 3)) -Force
  Write-DreamSkinTheme -ThemeDirectory $paths.Active -Theme $theme
  Set-DreamSkinPaused -Paused $false -StateRoot $StateRoot | Out-Null
  Invoke-ManagerInjection -Session $Session
  return [pscustomobject]@{ active = $true; opacity = [math]::Round($Opacity, 3); port = $Session.Port }
}

try {
  switch ($Action) {
    'ensure-session' {
      $session = Ensure-ManagerSession
      [pscustomobject]@{ active = -not (Test-DreamSkinPaused -StateRoot $StateRoot); port = $session.Port } |
        ConvertTo-Json -Compress
    }
    'apply-image' {
      if ([string]::IsNullOrWhiteSpace($ImagePath)) { throw 'No theme image path was provided.' }
      Assert-DreamSkinImageFile -Path $ImagePath
      $session = Ensure-ManagerSession
      Set-DreamSkinPaused -Paused $false -StateRoot $StateRoot | Out-Null
      $null = Initialize-DreamSkinThemeStore -SkillRoot $SkillRoot -StateRoot $StateRoot
      $theme = [pscustomobject]@{
        schemaVersion = 1
        id = 'custom'
        name = 'Custom Image Theme'
        appearance = 'auto'
        imageOpacity = [math]::Round($Opacity, 3)
        art = [pscustomobject]@{ focusX = $null; focusY = $null; safeArea = 'auto'; taskMode = 'auto' }
      }
      $null = Set-DreamSkinActiveTheme -ImagePath $ImagePath -Theme $theme -StateRoot $StateRoot
      Invoke-ManagerInjection -Session $session
      [pscustomobject]@{ active = $true; imagePath = [System.IO.Path]::GetFullPath($ImagePath); opacity = [math]::Round($Opacity, 3); port = $session.Port } |
        ConvertTo-Json -Compress
    }
    'set-opacity' {
      $session = Get-ManagerSession
      if ($null -eq $session) {
        [pscustomobject]@{ active = $false; opacity = [math]::Round($Opacity, 3); port = $null } |
          ConvertTo-Json -Compress
        break
      }
      Set-ManagerOpacity -Session $session | ConvertTo-Json -Compress
    }
    'restore' {
      Invoke-ManagerRestore
      [pscustomobject]@{ active = $false } | ConvertTo-Json -Compress
    }
    'status' {
      $session = Get-ManagerSession
      $paths = Get-DreamSkinThemePaths -StateRoot $StateRoot
      $active = $null
      try { $active = Read-DreamSkinTheme -ThemeDirectory $paths.Active -SkipImageMetadata } catch {}
      $opacity = 0.8
      if ($active -and $active.Theme.PSObject.Properties['imageOpacity']) {
        $opacity = [math]::Max(0.1, [math]::Min(1.0, [double]$active.Theme.imageOpacity))
      }
      [pscustomobject]@{
        active = ($null -ne $session -and -not (Test-DreamSkinPaused -StateRoot $StateRoot))
        imagePath = if ($active) { "$($active.ImagePath)" } else { $null }
        opacity = [math]::Round($opacity, 3)
        port = if ($session) { $session.Port } else { $null }
      } | ConvertTo-Json -Compress
    }
  }
} catch {
  Write-Error $_.Exception.Message
  exit 1
}
