[CmdletBinding()]
param(
  [ValidateRange(1024, 65535)]
  [int]$Port = 9335,
  [ValidateRange(500, 10000)]
  [int]$PollMilliseconds = 1500,
  [ValidateRange(1000, 30000)]
  [int]$RestartSettleMilliseconds = 2500
)

$ErrorActionPreference = 'Stop'
. (Join-Path $PSScriptRoot 'common-windows.ps1')
. (Join-Path $PSScriptRoot 'theme-windows.ps1')

$StateRoot = Join-Path $env:LOCALAPPDATA 'CodexDreamSkin'
$StatePath = Join-Path $StateRoot 'state.json'
$PowerShell = (Get-Command powershell.exe -ErrorAction Stop).Source
$StartScript = Join-Path $PSScriptRoot 'start-dream-skin.ps1'

function Enter-DreamSkinAutoReapplyLock {
  $sid = [System.Security.Principal.WindowsIdentity]::GetCurrent().User.Value
  $mutex = [System.Threading.Mutex]::new($false, "Local\CodexDreamSkin.$sid.AutoReapply")
  $acquired = $false
  try {
    $acquired = $mutex.WaitOne(0)
  } catch [System.Threading.AbandonedMutexException] {
    $acquired = $true
  }
  if (-not $acquired) {
    $mutex.Dispose()
    return $null
  }
  return $mutex
}

function Exit-DreamSkinAutoReapplyLock {
  param([Parameter(Mandatory = $true)][System.Threading.Mutex]$Mutex)
  try { $Mutex.ReleaseMutex() } finally { $Mutex.Dispose() }
}

function Get-DreamSkinAutoReapplyContext {
  if (Test-DreamSkinPaused -StateRoot $StateRoot) { return $null }
  if (-not (Test-Path -LiteralPath $StatePath -PathType Leaf)) { return $null }
  $state = Read-DreamSkinState -Path $StatePath
  if ($null -eq $state -or -not $state.port) { return $null }
  $statePort = [int]$state.port
  Assert-DreamSkinPort -Port $statePort

  # 仅对已校验且仍在托管目录内的已保存主题进行自动恢复。
  $paths = Get-DreamSkinThemePaths -StateRoot $StateRoot
  $null = Read-DreamSkinTheme -ThemeDirectory $paths.Active -SkipImageMetadata
  return [pscustomobject]@{ State = $state; Port = $statePort }
}

$autoLock = Enter-DreamSkinAutoReapplyLock
if ($null -eq $autoLock) { exit 0 }

try {
  $wasManaged = $false
  $unmanagedSince = $null
  $attemptedForCurrentRun = $false

  while ($true) {
    try {
      $context = Get-DreamSkinAutoReapplyContext
      if ($null -eq $context) { break }

      $codex = Get-DreamSkinCodexInstall
      $processes = @(Get-DreamSkinCodexProcesses -Codex $codex)
      if ($processes.Count -eq 0) {
        # ChatGPT 没有运行时绝不主动启动；只等待用户下一次打开它。
        $wasManaged = $false
        $unmanagedSince = $null
        $attemptedForCurrentRun = $false
        Start-Sleep -Milliseconds $PollMilliseconds
        continue
      }

      $identity = Get-DreamSkinVerifiedCdpIdentity -Port $context.Port -Codex $codex
      if ($null -ne $identity) {
        $wasManaged = $true
        $unmanagedSince = $null
        $attemptedForCurrentRun = $false
        Start-Sleep -Milliseconds $PollMilliseconds
        continue
      }

      if ($wasManaged -or $null -eq $unmanagedSince) {
        # 已托管会话消失后，给用户刚打开的 ChatGPT 留出稳定启动时间。
        $wasManaged = $false
        $unmanagedSince = [datetime]::UtcNow
        $attemptedForCurrentRun = $false
      }

      $elapsed = ([datetime]::UtcNow - $unmanagedSince).TotalMilliseconds
      if (-not $attemptedForCurrentRun -and $elapsed -ge $RestartSettleMilliseconds) {
        $attemptedForCurrentRun = $true
        if (-not (Test-DreamSkinPaused -StateRoot $StateRoot)) {
          # start 脚本会再次验证官方 Store 包身份、CDP 归属及暂停标记后才重启并注入。
          $startArguments = @(
            '-NoLogo', '-NoProfile', '-NonInteractive', '-ExecutionPolicy', 'RemoteSigned',
            '-File', $StartScript, '-Port', "$($context.Port)", '-RestartExisting', '-OneShot',
            '-RequireUnpaused', '-OperationLockTimeoutMilliseconds', '5000'
          )
          $result = Invoke-DreamSkinNative -FilePath $PowerShell -ArgumentList $startArguments
          # 本次启动失败时不循环强制关闭用户的 ChatGPT；等待下一次完整重启再尝试。
          if ($result.ExitCode -ne 0) { $null = $result.Output }
        }
      }
    } catch {
      # 状态写入或 Store 更新的短暂窗口不应终止守护进程，也不应触发重启。
    }
    Start-Sleep -Milliseconds $PollMilliseconds
  }
} finally {
  Exit-DreamSkinAutoReapplyLock -Mutex $autoLock
}
