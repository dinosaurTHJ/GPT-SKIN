[CmdletBinding()]
param(
  [int]$Port = 9335,
  [switch]$Uninstall,
  [switch]$RestoreBaseTheme,
  [switch]$RecoverConfigBackup,
  [switch]$PromptRestart,
  [switch]$ForceRestart,
  [switch]$NoRelaunch,
  # 管理界面的恢复按钮只清理当前页面，不关闭官方应用。
  [switch]$KeepRunning
)

$ErrorActionPreference = 'Stop'
$utf8 = [System.Text.UTF8Encoding]::new($false)
[Console]::OutputEncoding = $utf8
[Console]::InputEncoding = $utf8
$OutputEncoding = $utf8
$PortExplicit = $PSBoundParameters.ContainsKey('Port')
. (Join-Path $PSScriptRoot 'common-windows.ps1')
. (Join-Path $PSScriptRoot 'theme-windows.ps1')
. (Join-Path $PSScriptRoot 'localization-windows.ps1')

$operationLock = Enter-DreamSkinOperationLock
try {
  if ($RestoreBaseTheme -and $RecoverConfigBackup) {
    throw 'Choose either -RestoreBaseTheme or -RecoverConfigBackup, not both.'
  }
  if ($KeepRunning -and ($Uninstall -or $RecoverConfigBackup -or $PromptRestart -or $ForceRestart -or $NoRelaunch)) {
    throw 'Live restore cannot be combined with uninstall, full config recovery, or restart options.'
  }
  Assert-DreamSkinPort -Port $Port

  $StateRoot = Join-Path $env:LOCALAPPDATA 'CodexDreamSkin'
  # 先暂停自动恢复，避免恢复官方主题期间被后台守护进程重新注入皮肤。
  Set-DreamSkinPaused -Paused $true -StateRoot $StateRoot | Out-Null
  $language = Resolve-DreamSkinLanguage -StateRoot $StateRoot
  $themePaths = Get-DreamSkinThemePaths -StateRoot $StateRoot
  Ensure-DreamSkinManagedDirectory -Path $themePaths.Root -Root $themePaths.Root
  $StatePath = Join-Path $StateRoot 'state.json'
  $state = Read-DreamSkinState -Path $StatePath
  if (-not $PortExplicit -and $null -ne $state -and $state.port) {
    $Port = [int]$state.port
    Assert-DreamSkinPort -Port $Port
  }

  $currentCodex = $null
  try { $currentCodex = Get-DreamSkinCodexInstall } catch { Write-Warning $_.Exception.Message }
  $savedPathCandidate = Get-DreamSkinCodexStatePathCandidate -State $state
  $savedCodex = Get-DreamSkinCodexInstallFromState -State $state
  $candidateMatchesCurrent = [bool]($null -ne $savedPathCandidate -and $null -ne $currentCodex -and
    (Test-DreamSkinPathEqual -Left $savedPathCandidate.PackageRoot -Right $currentCodex.PackageRoot) -and
    (Test-DreamSkinPathEqual -Left $savedPathCandidate.Executable -Right $currentCodex.Executable))
  if ($null -ne $savedPathCandidate -and $null -eq $savedCodex -and -not $candidateMatchesCurrent) {
    $unverifiedSavedRunning = (Get-DreamSkinCodexProcesses -Codex $savedPathCandidate).Count -gt 0
    $unverifiedSavedOwnsPort = Test-DreamSkinCodexPortOwner -Port $Port -Codex $savedPathCandidate
    if ($unverifiedSavedRunning -or $unverifiedSavedOwnsPort) {
      throw 'The saved Codex path is still active but no longer matches a registered OpenAI.Codex package. Close it manually; state and configuration were preserved.'
    }
  }
  $savedIsDifferent = [bool]($null -ne $savedCodex -and $null -ne $currentCodex -and
    -not (Test-DreamSkinPathEqual -Left $savedCodex.Executable -Right $currentCodex.Executable))
  $currentRunning = $null -ne $currentCodex -and (Get-DreamSkinCodexProcesses -Codex $currentCodex).Count -gt 0
  $savedRunning = $null -ne $savedCodex -and (Get-DreamSkinCodexProcesses -Codex $savedCodex).Count -gt 0
  $savedOwnsPort = $null -ne $savedCodex -and (Test-DreamSkinCodexPortOwner -Port $Port -Codex $savedCodex)
  if ($savedIsDifferent -and $currentRunning -and ($savedRunning -or $savedOwnsPort)) {
    throw 'Multiple Codex package versions are active. Close them manually before restore; state and configuration were preserved.'
  }

  $codex = $currentCodex
  if ($savedRunning -or $savedOwnsPort -or $null -eq $currentCodex) {
    $codex = $savedCodex
    if ($null -ne $codex -and $savedIsDifferent) {
      Write-Warning 'Using the saved Codex package identity to close its older active CDP session.'
    } elseif ($null -ne $codex -and $null -eq $currentCodex) {
      Write-Warning 'Using the saved Codex identity after revalidating it against the registered Store package.'
    }
  }
  $relaunchCodex = if ($null -ne $currentCodex) { $currentCodex } else { $codex }
  $codexRunning = $null -ne $codex -and (Get-DreamSkinCodexProcesses -Codex $codex).Count -gt 0
  $portOwnedByCodex = $null -ne $codex -and (Test-DreamSkinCodexPortOwner -Port $Port -Codex $codex)
  if ($portOwnedByCodex -and -not $codexRunning) {
    throw 'A Codex-owned listener exists without a manageable Codex process; state was preserved.'
  }
  if ($null -ne $state -and $null -eq $codex -and -not (Test-DreamSkinPortAvailable -Port $Port)) {
    throw "Port $Port is still active, but Codex ownership cannot be verified. State and configuration were preserved."
  }

  if ($KeepRunning) {
    # 有监听端口时仍须核实包身份和 browserId，不能向其他进程注入清理代码。
    if (-not (Test-DreamSkinPortAvailable -Port $Port)) {
      if ($null -eq $codex) { throw '无法确认当前应用身份，已保留会话和外观备份。' }
      $identity = Get-DreamSkinVerifiedCdpIdentity -Port $Port -Codex $codex
      if ($null -eq $identity) { throw '无法确认当前换肤连接，已保留会话和外观备份。' }
      $node = Get-DreamSkinNodeRuntime
      $injector = Join-Path $PSScriptRoot 'injector.mjs'
      $result = Invoke-DreamSkinNative -FilePath $node.Path -ArgumentList @(
        $injector, '--remove', '--port', "$Port", '--browser-id', $identity.BrowserId,
        '--timeout-ms', '30000'
      )
      if ($result.ExitCode -ne 0) {
        throw ('移除页面皮肤失败：' + (($result.Output | Select-Object -Last 8) -join "`n").Trim())
      }
    } elseif ($codexRunning) {
      # 正常启动的官方进程没有皮肤注入；仍持有调试参数的进程不能假定已清理。
      $debugStatus = Get-DreamSkinCodexDebugArgumentStatus -Processes @(Get-DreamSkinCodexProcesses -Codex $codex) -Port $Port
      if ($debugStatus -ne 'not-forwarded' -and $debugStatus -ne 'protocol-redirected') {
        throw '当前调试连接不可用，无法确认页面皮肤已移除。请手动关闭并重新打开应用后再恢复。'
      }
    }
    $backup = Join-Path $StateRoot 'config.before-dream-skin.toml'
    $config = Join-Path $HOME '.codex\config.toml'
    if ($RestoreBaseTheme -and (Test-Path -LiteralPath $backup -PathType Leaf)) {
      Restore-DreamSkinBaseTheme -ConfigPath $config -BackupPath $backup
      $archiveStamp = (Get-Date).ToString('yyyyMMdd-HHmmss-fff') + '-' + [guid]::NewGuid().ToString('N')
      Archive-DreamSkinConfigBackup -BackupPath $backup -ArchivePath (Join-Path $StateRoot "config.restored-$archiveStamp.toml")
    }
    # 保留暂停标记及会话身份，防止守护进程重新换肤，也便于下次应用主题复用连接。
    Write-Host '官方外观已恢复，应用保持运行。'
    return
  }

  $shouldCloseCodex = $codexRunning
  $forceAuthorized = [bool]$ForceRestart
  if ($shouldCloseCodex -and $PromptRestart) {
    $restartMessage = if ($NoRelaunch) {
      Get-DreamSkinText -Key 'RestoreCloseNoRelaunch' -Language $language
    } else {
      Get-DreamSkinText -Key 'RestoreClose' -Language $language
    }
    $forceAuthorized = Confirm-DreamSkinRestart -Message $restartMessage
    if (-not $forceAuthorized) {
      Write-Host (Get-DreamSkinText -Key 'RestoreCancelled' -Language $language)
      exit 0
    }
  }

  $backup = Join-Path $StateRoot 'config.before-dream-skin.toml'
  $config = Join-Path $HOME '.codex\config.toml'
  if ($RecoverConfigBackup) {
    if (-not (Test-Path -LiteralPath $backup)) { throw 'No pre-install config backup is available.' }
    $null = Read-DreamSkinUtf8File -Path $backup
  } elseif ($RestoreBaseTheme) {
    if (-not (Test-Path -LiteralPath $backup)) { throw 'No pre-install config backup is available.' }
    $null = Read-DreamSkinUtf8File -Path $backup
    $null = Read-DreamSkinUtf8File -Path $config
  }

  $restoreError = $null
  try {
    Stop-DreamSkinTrayProcess
    if ($shouldCloseCodex) {
      Stop-DreamSkinCodex -Codex $codex -AllowForce:$forceAuthorized
      if ($portOwnedByCodex -and -not (Wait-DreamSkinPortAvailable -Port $Port -TimeoutSeconds 5)) {
        throw "Port $Port is still listening after Codex closed; state was preserved for inspection."
      }
    }

    $recordedInjectorStopped = Stop-DreamSkinRecordedInjector -State $state
    if (-not $recordedInjectorStopped) {
      $staleStatePath = Archive-DreamSkinStateFile -Path $StatePath
      Write-Warning "Archived stale Dream Skin state at $staleStatePath"
    }

    if ($RecoverConfigBackup) {
      $stamp = (Get-Date).ToString('yyyyMMdd-HHmmss-fff') + '-' + [guid]::NewGuid().ToString('N')
      $recoveryBackup = Join-Path $StateRoot "config.before-recovery-$stamp.toml"
      Restore-DreamSkinConfigBackup -ConfigPath $config -BackupPath $backup -RecoveryBackupPath $recoveryBackup
      Write-Host "Recovered the exact pre-install config; previous current config saved at $recoveryBackup"
    } elseif ($RestoreBaseTheme) {
      Restore-DreamSkinBaseTheme -ConfigPath $config -BackupPath $backup
    }
    if ($RecoverConfigBackup -or $RestoreBaseTheme) {
      $archiveStamp = (Get-Date).ToString('yyyyMMdd-HHmmss-fff') + '-' + [guid]::NewGuid().ToString('N')
      $archivePath = Join-Path $StateRoot "config.restored-$archiveStamp.toml"
      Archive-DreamSkinConfigBackup -BackupPath $backup -ArchivePath $archivePath
      Write-Host "Archived the completed pre-install backup at $archivePath"
    }

    Remove-Item -LiteralPath $StatePath -Force -ErrorAction SilentlyContinue
    Remove-Item -LiteralPath (Join-Path $StateRoot 'paused') -Force -ErrorAction SilentlyContinue
    if ($Uninstall) {
      $desktop = [Environment]::GetFolderPath('Desktop')
      $startMenu = Join-Path $env:APPDATA 'Microsoft\Windows\Start Menu\Programs'
      @(
        (Join-Path $desktop 'Codex Dream Skin.lnk'),
        (Join-Path $desktop 'Codex Dream Skin - Restore.lnk'),
        (Join-Path $desktop 'Codex Dream Skin - Tray.lnk'),
        (Join-Path $startMenu 'Codex Dream Skin.lnk'),
        (Join-Path $startMenu 'Codex Dream Skin - Tray.lnk')
      ) | ForEach-Object { Remove-Item -LiteralPath $_ -Force -ErrorAction SilentlyContinue }
    }

    if ($shouldCloseCodex -and -not $NoRelaunch) {
      if ($null -eq $relaunchCodex -or -not (Test-Path -LiteralPath $relaunchCodex.Executable)) {
        throw 'Codex cannot be reopened because its current executable is unavailable.'
      }
      $null = Start-DreamSkinCodex -Codex $relaunchCodex
    }
  } catch {
    $restoreError = $_
    if ($shouldCloseCodex -and -not $NoRelaunch -and $null -ne $relaunchCodex -and
      (Get-DreamSkinCodexProcesses -Codex $codex).Count -eq 0 -and (Test-Path -LiteralPath $relaunchCodex.Executable)) {
      try { $null = Start-DreamSkinCodex -Codex $relaunchCodex } catch {
        Write-Warning 'Restore failed and Codex could not be reopened automatically.'
      }
    }
    throw $restoreError
  }

  Write-Host 'Dream Skin restore actions completed; any saved CDP session was closed.'
} catch {
  # 子进程只输出原始失败原因，防止调用方截取末尾日志时丢失异常正文。
  [Console]::Error.WriteLine($_.Exception.Message)
  exit 1
} finally {
  Exit-DreamSkinOperationLock -Mutex $operationLock
}
