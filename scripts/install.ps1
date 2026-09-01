param(
    [string]$Version = $(if ($env:APOSLOP_VERSION) { $env:APOSLOP_VERSION } else { "latest" }),
    [string]$InstallDir = $(if ($env:APOSLOP_INSTALL_DIR) { $env:APOSLOP_INSTALL_DIR } else { Join-Path $env:LOCALAPPDATA "Programs\aposlop\bin" }),
    [switch]$SkipSignatureVerification
)

$ErrorActionPreference = "Stop"
$repository = "https://github.com/EzyGang/aposlop"
$issuer = "https://token.actions.githubusercontent.com"
$skipSignature = $SkipSignatureVerification -or $env:APOSLOP_SKIP_SIGNATURE_VERIFY -eq "1"

if ([Environment]::OSVersion.Platform -ne [PlatformID]::Win32NT) {
    throw "aposlop installer: this script supports Windows only"
}

$architecture = switch ([Runtime.InteropServices.RuntimeInformation]::OSArchitecture.ToString()) {
    "X64" { "amd64" }
    "Arm64" { "arm64" }
    default { throw "aposlop installer: unsupported architecture" }
}

if ($Version -eq "latest") {
    $headers = @{ "User-Agent" = "aposlop-installer" }
    $release = Invoke-RestMethod -Uri "https://api.github.com/repos/EzyGang/aposlop/releases/latest" -Headers $headers
    $tag = $release.tag_name
    $Version = $tag -replace '^v', ''
} else {
    $Version = $Version -replace '^v', ''
    $tag = "v$Version"
}

if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    throw "aposlop installer: invalid release version: $Version"
}

$archive = "aposlop-$Version-windows-$architecture.zip"
$downloadRoot = "$repository/releases/download/$tag"
$temporary = Join-Path ([IO.Path]::GetTempPath()) "aposlop-$([Guid]::NewGuid())"
$expanded = Join-Path $temporary "expanded"
New-Item -ItemType Directory -Path $temporary, $expanded -Force | Out-Null

try {
    $archivePath = Join-Path $temporary $archive
    $checksumsPath = Join-Path $temporary "SHA256SUMS"
    Invoke-WebRequest -Uri "$downloadRoot/$archive" -OutFile $archivePath -UseBasicParsing
    Invoke-WebRequest -Uri "$downloadRoot/SHA256SUMS" -OutFile $checksumsPath -UseBasicParsing

    $checksumLine = Get-Content $checksumsPath | Where-Object { $_ -match "\s$([Regex]::Escape($archive))$" } | Select-Object -First 1
    if (-not $checksumLine) {
        throw "aposlop installer: checksum is missing for $archive"
    }
    $expected = ($checksumLine -split '\s+')[0].ToLowerInvariant()
    $actual = (Get-FileHash -Path $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($actual -ne $expected) {
        throw "aposlop installer: checksum verification failed for $archive"
    }

    $identity = "$repository/.github/workflows/release.yml@refs/tags/$tag"
    if (-not $skipSignature) {
        if (-not (Get-Command cosign -ErrorAction SilentlyContinue)) {
            throw "aposlop installer: cosign is required. Use -SkipSignatureVerification to verify only the checksum"
        }
        $archiveBundle = "$archivePath.sigstore.json"
        Invoke-WebRequest -Uri "$downloadRoot/$archive.sigstore.json" -OutFile $archiveBundle -UseBasicParsing
        & cosign verify-blob $archivePath --bundle $archiveBundle --certificate-identity $identity --certificate-oidc-issuer $issuer
        if ($LASTEXITCODE -ne 0) {
            throw "aposlop installer: archive signature verification failed"
        }
    }

    Expand-Archive -Path $archivePath -DestinationPath $expanded -Force
    $binary = Join-Path $expanded "aposlop.exe"
    if (-not $skipSignature) {
        $binaryBundle = Join-Path $expanded "aposlop.sigstore.json"
        & cosign verify-blob $binary --bundle $binaryBundle --certificate-identity $identity --certificate-oidc-issuer $issuer
        if ($LASTEXITCODE -ne 0) {
            throw "aposlop installer: binary signature verification failed"
        }
    }

    New-Item -ItemType Directory -Path $InstallDir -Force | Out-Null
    Copy-Item -Path $binary -Destination (Join-Path $InstallDir "aposlop.exe") -Force

    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    $pathEntries = if ($userPath) { $userPath -split ';' } else { @() }
    if ($pathEntries -notcontains $InstallDir) {
        $newPath = if ($userPath) { "$userPath;$InstallDir" } else { $InstallDir }
        [Environment]::SetEnvironmentVariable("Path", $newPath, "User")
        $env:Path = "$env:Path;$InstallDir"
    }

    Write-Output "Installed aposlop $Version to $InstallDir\aposlop.exe"
} finally {
    Remove-Item -Path $temporary -Recurse -Force -ErrorAction SilentlyContinue
}
